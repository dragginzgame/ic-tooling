#!/usr/bin/env perl
use strict;
use warnings;
use Test::More;
use File::Temp qw(tempdir);
use File::Path qw(make_path);
use Cwd qw(abs_path);

my $script = abs_path('scripts/ci/setup-diagnostics.pl');
my $temp = tempdir(CLEANUP => 1);
my $fixture = "$temp/fail.pl";
open my $file, '>', $fixture or die $!;
print {$file} <<'CHILD';
use strict;
use warnings;
use File::Path qw(make_path);
my $root = $ENV{CI_SETUP_ROOT};
# Assemble synthetic markers without making the fixture source resemble a key.
my $key_begin = '-----BEGIN ' . 'PRIVATE KEY-----';
my $key_end = '-----END ' . 'PRIVATE KEY-----';
$| = 1;
if ($ARGV[0] eq 'success') { print "prepared\n"; exit 0; }
if ($ARGV[0] eq 'missing') { exit 19; }
if ($ARGV[0] eq 'signal') { kill 'TERM', $$; sleep 2; exit 99; }
if ($ARGV[0] eq 'collector') {
    open my $file, '>', $root or die $!;
    print {$file} 'not a directory'; close $file;
    print STDERR "compiler fixture failure\n"; exit 23;
}
make_path("$root/tools/.failed-attempt");
open my $log, '>', "$root/tools/.failed-attempt/install.log" or die $!;
print {$log} "download: certificate verification failed\n";
print {$log} qq({"password":"hidden-password"}\n);
print {$log} "$key_begin\nprivate-key-body\n$key_end\n";
close $log;
open my $outside, '>', "$root/outside.txt" or die $!;
print {$outside} 'must-not-follow-link'; close $outside;
symlink "$root/outside.txt", "$root/tools/.failed-attempt/stderr.log" or die $!;
make_path("$root/outside-dir");
open my $linked, '>', "$root/outside-dir/install.log" or die $!;
print {$linked} 'must-not-follow-directory'; close $linked;
symlink "$root/outside-dir", "$root/tools/linked" or die $!;
open my $unselected, '>', "$root/tools/.failed-attempt/config.json" or die $!;
print {$unselected} 'must-not-select-config'; close $unselected;
open my $giant, '>', "$root/tools/.failed-attempt/download.log" or die $!;
print {$giant} 'x' x (1024 * 1024 + 1); close $giant;
for (1..1000) { print "setup progress $_ ", 'building a fixture ' x 6, "\n"; }
print "Authorization: Bearer known-auth\n";
print "token=", 't' x 10000, "\n";
print "https://user:pass\@host/file?key=hidden-url\n";
print "https://host/file#hidden-fragment\n";
print "UNCLASSIFIED_SETTING=hidden-environment-value\n";
print "Bearer hidden-bearer\n";
print qq({"access_key": "hidden-access-key"}\n);
print $key_begin, 'x' x 9000, "\noversized-private-body\n$key_end\n";
print "$ENV{TEST_SECRET}\n";
print "$key_begin\nother-private-body\n$key_end\n";
print "::warning::untrusted workflow annotation\n";
print STDERR "compiler: fixture link failure\n";
if ($ARGV[0] eq 'timeout') { sleep 30; }
exit 37;
CHILD
close $file;

sub read_file {
    open my $f, '<', $_[0] or die $!;
    local $/; return <$f> // '';
}
sub run {
    my ($mode) = @_;
    local $ENV{CI_SETUP_ROOT} = "$temp/$mode";
    local $ENV{GITHUB_OUTPUT} = "$temp/$mode.outputs";
    local $ENV{TEST_SECRET} = 'exact-environment-secret';
    my $pid = open my $out, '-|';
    defined $pid or die $!;
    if (!$pid) {
        open STDERR, '>&', STDOUT or die $!;
        my @deadline = $mode eq 'timeout' ? ('timeout', '--kill-after=1s', '0.2s') : ();
        exec $^X, $script, 'compiler', @deadline, $^X, $fixture, $mode;
        die $!;
    }
    local $/; my $printed = <$out> // '';
    close $out;
    return ($? >> 8, $printed);
}
my ($status, $printed) = run('failure');
is($status, 37, 'original setup failure survives capture and collection');
like($printed, qr/compiler: fixture link failure/, 'underlying error surfaced');
unlike($printed, qr/^::warning::/m, 'untrusted workflow commands are not emitted');
cmp_ok(length($printed), '<', 10000, 'printed diagnostic is bounded');
my $outputs = read_file("$temp/failure.outputs");
my ($evidence) = $outputs =~ /^evidence=(.+)$/m;
ok(defined $evidence && -d $evidence, 'only completed collection publishes upload path');
my @files = glob "$evidence/*.txt";
is(scalar @files, 3, 'only output, metadata and selected hidden attempt log retained');
my $retained = join '', map { read_file($_) } @files;
like($retained, qr/certificate verification failed/, 'hidden attempt diagnostic survives');
like($printed, qr/certificate verification failed/, 'hidden failure is surfaced before upload');
like($retained, qr/attempt-1=install.log source_path_sha256=[0-9a-f]{64}/, 'attempt identity retained without disclosing arbitrary paths');
like($retained, qr/earlier output, oversized lines or bytes beyond the read limit omitted/, 'truncation explicit');
like($retained, qr/oversized attempt log omitted/, 'giant attempt log explicitly omitted');
for my $secret (qw(hidden-password known-auth hidden-url hidden-fragment hidden-environment-value hidden-bearer hidden-access-key exact-environment-secret private-key-body other-private-body oversized-private-body must-not-follow-link must-not-follow-directory must-not-select-config)) {
    unlike($retained . $printed, qr/\Q$secret\E/, "sensitive/unselected bytes excluded: $secret");
}
cmp_ok(length($retained), '<', 70000, 'retention bound holds for setup and attempt logs');
($status, $printed) = run('missing');
is($status, 19, 'missing attempt logs do not erase failure');
($evidence) = read_file("$temp/missing.outputs") =~ /^evidence=(.+)$/m;
like(read_file("$evidence/compiler-status.txt"), qr/No selected attempt logs present/, 'missing attempts explicit');
like(read_file("$evidence/compiler-output.txt"), qr/no usable diagnostic/, 'empty command output explicit');
($status, $printed) = run('collector');
is($status, 23, 'collector failure preserves original status');
like($printed, qr/Diagnostic collection failed/, 'collector failure separately reported');
ok(!-e "$temp/collector.outputs", 'failed collection supplies no upload path');
($status, $printed) = run('success');
is($status, 0, 'successful setup remains successful');
ok(!-e "$temp/success.outputs", 'success has no failure artifact');
($status, $printed) = run('signal');
is($status, 143, 'terminated child retains shell-style signal status');
SKIP: {
    skip 'GNU timeout wiring belongs to the Ubuntu repository job', 4 unless $^O eq 'linux';
    ($status, $printed) = run('timeout');
    is($status, 124, 'setup deadline remains a failing timeout status');
    ($evidence) = read_file("$temp/timeout.outputs") =~ /^evidence=(.+)$/m;
    like(read_file("$evidence/compiler-status.txt"), qr/original_status=124/, 'timeout status retained for upload');
    like(read_file("$evidence/compiler-attempt-1.txt"), qr/certificate verification failed/, 'timeout retains hidden attempt evidence');
    unlike(read_file("$evidence/compiler-output.txt") . $printed, qr/exact-environment-secret/, 'timeout evidence stays sanitized');
}
# Exercise the workflow's actual upload-report branch without a hosted upload.
my $workflow = read_file('.github/workflows/build-release-candidate.yml');
SKIP: {
    skip 'pinned scanner setup runs on Ubuntu', 8 unless $^O eq 'linux';
    my ($setup) = $workflow =~ /- name: Prepare pinned secret scanner\n.*?        run: \|\n((?:          [^\n]*\n)+)/s;
    ok(defined $setup, 'actual scanner setup command selected');
    $setup =~ s/^          //mg;
    open my $setup_file, '>', "$temp/scanner-setup.sh" or die $!;
    print {$setup_file} $setup; close $setup_file;
    make_path("$temp/bin");
    open my $curl, '>', "$temp/bin/curl" or die $!;
    print {$curl} "#!/bin/sh\necho 'scanner download fixture failed' >&2\necho 'Authorization: Bearer scanner-fixture-secret' >&2\nexit 22\n";
    close $curl; chmod 0755, "$temp/bin/curl" or die $!;
    local $ENV{PATH} = "$temp/bin:$ENV{PATH}";
    local $ENV{CI_SETUP_ROOT} = "$temp/scanner";
    local $ENV{TOKO_TOOLING_DOWNLOAD_CACHE} = "$temp/empty-cache";
    local $ENV{GITHUB_ENV} = "$temp/scanner.env";
    local $ENV{GITHUB_OUTPUT} = "$temp/scanner.outputs";
    my $pid = open my $out, '-|';
    defined $pid or die $!;
    if (!$pid) { open STDERR, '>&', STDOUT or die $!; exec 'bash', '-e', "$temp/scanner-setup.sh"; die $!; }
    local $/; my $printed = <$out> // ''; close $out;
    is($? >> 8, 22, 'real pinned installer failure survives workflow wrapper');
    ok(!-e "$temp/scanner.env", 'failed scanner setup does not publish an executable');
    my ($evidence) = read_file("$temp/scanner.outputs") =~ /^evidence=(.+)$/m;
    ok(defined $evidence && -d $evidence, 'scanner failure supplies upload evidence');
    my $output = read_file("$evidence/scanner-output.txt");
    like($output, qr/scanner download fixture failed/, 'underlying scanner setup error retained');
    unlike($output . $printed, qr/scanner-fixture-secret/, 'scanner output is sanitized');
    like(read_file("$evidence/scanner-status.txt"), qr/original_status=22/, 'scanner original status recorded');
    is(scalar(glob "$evidence/*.gz"), undef, 'download payload excluded from evidence');
}
for my $name ('Report setup evidence', 'Report scanner setup evidence') {
    my ($report) = $workflow =~ /- name: \Q$name\E\n.*?        run: \|\n((?:          [^\n]*\n)+)/s;
    ok(defined $report, "$name handler selected");
    $report =~ s/^          //mg;
    open my $handler, '>', "$temp/report.sh" or die $!;
    print {$handler} $report; close $handler;
    for my $outcome (qw(failure success)) {
        local $ENV{UPLOAD_OUTCOME} = $outcome;
        local $ENV{EVIDENCE_URL} = $outcome eq 'success' ? 'https://example.invalid/artifact/1' : '';
        local $ENV{GITHUB_STEP_SUMMARY} = "$temp/$outcome.summary";
        my $pid = open my $output, '-|';
        defined $pid or die $!;
        if (!$pid) { exec 'bash', "$temp/report.sh"; die $!; }
        local $/; my $result = <$output> // '';
        close $output;
        is($? >> 8, $outcome eq 'success' ? 0 : 1, "upload $outcome handler status");
        if ($outcome eq 'failure') { like($result, qr/evidence missing or upload failed/, 'upload failure explicitly reported'); }
        else { like(read_file($ENV{GITHUB_STEP_SUMMARY}), qr{https://example.invalid/artifact/1}, 'artifact link added to summary'); }
    }
}
done_testing;
