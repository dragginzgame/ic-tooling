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
print {$log} "-----BEGIN PRIVATE KEY-----\nprivate-key-body\n-----END PRIVATE KEY-----\n";
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
print "-----BEGIN PRIVATE KEY-----", 'x' x 9000, "\noversized-private-body\n-----END PRIVATE KEY-----\n";
print "$ENV{TEST_SECRET}\n";
print "-----BEGIN PRIVATE KEY-----\nother-private-body\n-----END PRIVATE KEY-----\n";
print "::warning::untrusted workflow annotation\n";
print STDERR "compiler: fixture link failure\n";
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
        exec $^X, $script, 'compiler', $^X, $fixture, $mode;
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
# Exercise the workflow's actual upload-report branch without a hosted upload.
my $workflow = read_file('.github/workflows/build-release-candidate.yml');
my ($report) = $workflow =~ /- name: Report setup evidence\n.*?        run: \|\n((?:          [^\n]*\n)+)/s;
ok(defined $report, 'workflow upload-result handler selected');
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
done_testing;
