#!/usr/bin/env perl
use strict;
use warnings;
use JSON::PP;
use File::Basename qw(dirname);
use File::Temp qw(tempdir);
use File::Path qw(remove_tree);
use File::Spec;
use Errno qw(ENOENT);
use Digest::SHA qw(sha256_hex);
my $check = @ARGV == 6 && pop(@ARGV) eq '--check';
my ($source, $group, $baseline_path, $plan_path, $context_path) = @ARGV;
@ARGV == 5 or die "usage: select-validation.pl SOURCE GROUP BASELINE PLAN CONTEXT\n";
my $json = JSON::PP->new->canonical->utf8;
sub read_file {
    my ($path) = @_;
    -f $path && !-l $path && -s $path <= 1048576 or die "invalid selection input $path\n";
    open my $in, '<:raw', $path or die "read $path: $!\n";
    local $/; my $raw = <$in>; close $in or die "close $path: $!\n"; return $raw;
}
sub command {
    my (@argv) = @_;
    open my $in, '-|', @argv or die "execute $argv[0]: $!\n";
    binmode $in; local $/; my $out = <$in>;
    close $in or die "failed command $argv[0]\n";
    return $out;
}
sub git { return command('git', @_); }
sub write_json {
    my ($path, $value) = @_;
    if ($check) {
        read_file($path) eq $json->encode($value)."\n" or die "saved selection no longer matches source/configuration: $path\n";
        return;
    }
    !-e $path && !-l $path or die "selection output already exists: $path\n";
    open my $out, '>:raw', $path or die "write $path: $!\n";
    print {$out} $json->encode($value), "\n" or die "write $path: $!\n";
    close $out or die "close $path: $!\n";
}
$source =~ /\A(?:[0-9a-f]{40}|[0-9a-f]{64})\z/ or die "invalid source\n";
git('rev-parse', '--verify', "$source^{commit}") eq "$source\n" or die "wrong source\n";
my $policy = $json->decode(read_file('ci/release-validation.json'));
my $coverage = $json->decode(read_file('ci/release-validation-coverage.json'));
my $groups = $json->decode(read_file('ci/cargo-release-groups.json'));
$policy->{schema} == 1 && $coverage->{schema} == 1 or die "unknown selection schema\n";
my @groups = sort map { $_->{name} } @{$groups->{groups}};
grep($_ eq $group, @groups) or die "unknown release group\n";
join(',', sort(@{$coverage->{focused}}, keys %{$coverage->{fallback}})) eq join(',', @groups)
    or die "incomplete group coverage\n";
join(',', sort keys %{$coverage->{consumers}}) eq join(',', sort @{$coverage->{focused}})
    or die "incomplete focused consumer coverage\n";
my %owned = map { $_ => 1 } map { @{$_->{members}} } @{$groups->{groups}};
my @focus = grep { length } split /\n/, command('bash', 'scripts/ci/validation-roster.sh', 'focused');
my %focus = map { $_ => 1 } @focus;
my @full = grep { length } split /\n/, command('bash', 'scripts/ci/validation-roster.sh', 'full');
my %full = map { $_ => 1 } @full;
my %common = map { $_ => 1 } @{$coverage->{common}};
my %exclusions = map {
    my $contract = $_;
    map { $_ => $contract->{reason} } @{$contract->{targets}}
} grep { grep { $_ eq $group } @{$_->{groups}} } @{$coverage->{exclusions}};
my $baseline = $json->decode(read_file($baseline_path));
my $narrow = 0;
my $reason = $coverage->{fallback}{$group} // 'No admitted full native CI baseline is available.';
my $scratch;
my $number = 0;
sub toml {
    my ($revision, $path) = @_;
    $scratch //= tempdir('tooling-selection.XXXXXX', DIR => $check ? ($ENV{TMPDIR} // '/tmp') : dirname($context_path), CLEANUP => 0);
    my $file = "$scratch/input-" . ++$number . '.toml';
    open my $out, '>:raw', $file or die "write selection input: $!\n";
    print {$out} git('show', "$revision:$path") or die "retain selection input: $!\n";
    close $out or die "close selection input: $!\n";
    return $json->decode(command($ENV{YQ} // '.tools/host/bin/yq', '-p', 'toml', '-o', 'json', '.', $file));
}
sub version_only {
    my ($path, $before, $after) = @_;
    if ($path eq 'Cargo.toml') {
        for my $root ($before, $after) {
            for my $name (keys %owned) {
                ref($root->{workspace}{dependencies}{$name}) eq 'HASH' or die "missing owned catalog entry\n";
                delete $root->{workspace}{dependencies}{$name}{version};
            }
        }
    } elsif ($path eq 'Cargo.lock') {
        for my $root ($before, $after) {
            for my $package (@{$root->{package}}) {
                delete $package->{version} if !$package->{source} && $owned{$package->{name}};
            }
        }
    } else {
        delete $before->{package}{version}; delete $after->{package}{version};
    }
    return $json->encode($before) eq $json->encode($after);
}
sub supported_environment {
    for my $name (qw(CARGO RUSTC RUSTDOC RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER RUSTFLAGS CARGO_ENCODED_RUSTFLAGS CARGO_ENCODED_RUSTDOCFLAGS RUSTDOCFLAGS RUSTC_BOOTSTRAP CARGO_TARGET_DIR),
        grep { /\ACARGO_(?:BUILD_|TARGET_|PROFILE_|FEATURE_|CFG_)|\A(?:CC|CXX|AR|LD)(?:_|\z)|\A(?:SDKROOT|MACOSX_DEPLOYMENT_TARGET)\z/ } keys %ENV) {
        return 0 if defined($ENV{$name}) && length($ENV{$name});
    }
    # Unmodelled Cargo configuration keeps the established full gate. Do not parse
    # or reproduce Cargo's configuration precedence in this consumer selector.
    my @directories;
    my $directory = git('rev-parse', '--show-toplevel'); chomp $directory;
    while (1) { push @directories, "$directory/.cargo"; my $parent = dirname($directory); last if $parent eq $directory; $directory = $parent; }
    push @directories, $ENV{CARGO_HOME} // "$ENV{HOME}/.cargo";
    for my $dir (@directories) {
        for my $file ("$dir/config", "$dir/config.toml") {
            my @stat = lstat $file;
            return 0 if @stat || $! != ENOENT;
        }
    }
    return 1;
}
sub qualified_baseline {
    my ($record) = @_;
    command('jq', '-f', 'ci/release-validation-baseline.jq', $baseline_path) eq "true\n" or return 0;
    return system('git', 'merge-base', '--is-ancestor', $record->{source}, $source) == 0;
}
my @changes;
if (grep($_ eq $group, @{$coverage->{focused}})) {
    if ($baseline && qualified_baseline($baseline) && supported_environment()) {
        @changes = grep { length } split /\0/, git('diff', '--name-only', '--no-renames', '-z', $baseline->{source}, $source, '--');
        my ($selected_group) = grep { $_->{name} eq $group } @{$groups->{groups}};
        my $package = $selected_group->{primary};
        my $safe = join(',', @{$selected_group->{members}}) eq $package;
        for my $path (@changes) {
            next if $path =~ m{\Acrates/\Q$package\E/(?:src/|tests/|README\.md\z)};
            next if $path =~ m{\Areleases/[^/]+/CHANGELOG\.md\z} || $path eq 'docs/status/current.md';
            if ($path eq 'Cargo.toml' || $path eq 'Cargo.lock' || $path =~ m{\A(?:crates|apps)/[^/]+/Cargo\.toml\z}) {
                $safe &&= version_only($path, toml($baseline->{source}, $path), toml($source, $path));
            } else { $safe = 0; }
        }
        if ($safe) {
            my $metadata = $json->decode(command('cargo', 'metadata', '--no-deps', '--locked', '--offline', '--format-version', '1'));
            my @parents = sort map {
                my $parent = $_;
                map { "$parent->{name}:" . ($_->{kind} // 'normal') }
                    grep { $_->{name} eq $package } @{$parent->{dependencies}}
            } @{$metadata->{packages}};
            my @expected = sort @{$coverage->{consumers}{$group}};
            $safe &&= join(',', @parents) eq join(',', @expected);
            my ($owner) = grep { $_->{name} eq $package } @{$metadata->{packages}};
            $safe &&= defined($owner) && ($group ne 'native' || !@{$owner->{dependencies}});
            # Text's runtime callers are bounded to these tested owners. A new
            # call surface needs an explicit mapping before it can be omitted.
            if ($group eq 'text') {
                my $references = git('grep', '-l', 'ic_host_text', $source, '--', 'crates/ic-host-tools/src', 'crates/ic-testkit-dev/src');
                for my $path (grep { length } split /\n/, $references) {
                    $path =~ s/^\Q$source\E://;
                    $safe &&= $path =~ m{\Acrates/ic-host-tools/src/icp_project/} ||
                        $path eq 'crates/ic-testkit-dev/src/bin/check_packages.rs';
                }
            }
            if ($group eq 'tools' || $group eq 'build') {
                my $import = $group eq 'tools' ? 'ic_host_tools' : 'ic_build';
                my $references = git('grep', '-l', $import, $source, '--', 'crates/ic-testkit-dev');
                for my $path (grep { length } split /\n/, $references) {
                    $path =~ s/^\Q$source\E://;
                    my $public = $path eq 'crates/ic-testkit-dev/tests/public_contracts.rs';
                    my $build = $path =~ m{\Acrates/ic-testkit-dev/(?:src/(?:bin/(?:tests/|check_packages\.rs\z))|tests/canister_wire\.rs\z)};
                    $safe &&= $public || ($group eq 'build' && $build);
                }
            }
        }
        $narrow = $safe ? 1 : 0;
        $reason = $narrow ? "Admitted $group-only change from a complete three-host native baseline; direct/reverse consumer contracts selected." :
            'Changes or consumer relationships exceed the qualified group scope; retain the complete gate.';
    } else { $reason = 'No matching complete native baseline or bounded Cargo environment; retain the complete gate.'; }
}
my %selected = $narrow ? (%common, %focus) : %full;
my @stages = map {
    my $target = $_; $target =~ s/^release-stage-//;
    {target => $_, required => $selected{$target} ? JSON::PP::true : JSON::PP::false,
     reason => $selected{$target} ? $reason : $narrow ? $exclusions{$target} :
        'Focused alternative is covered by the complete workspace gate.'}
} @{$policy->{stages}};
my @inputs = ('Makefile','Cargo.toml','Cargo.lock','rust-toolchain.toml','.shared-tooling.snapshot','ci/tool-versions.env','ci/tooling-checks.tsv',
    'ci/release-editor.env','ci/cargo-release-groups.json','ci/release-validation-coverage.json','ci/release-validation-baseline.jq',
    'make/release-validation-local.mk','make/testkit.mk','scripts/release/select-validation.pl',
    'scripts/release/validation-stage.sh','scripts/release/validation-focus.sh','scripts/release/validation-baseline.sh',
    'scripts/release/validation.sh','scripts/release/validation-proof.pl',
    'scripts/ci/validation-roster.sh','scripts/ci/workspace-validation.sh',
    'scripts/ci/validation-evidence.pl','scripts/ci/ValidationPlan.pm',
    'scripts/ci/check-release-source.sh',$context_path,$baseline_path);
push @inputs, map { $_->{changelog} } @{$groups->{groups}};
my %environment = map { $_ => sha256_hex($ENV{$_}) } grep {
    /\A(?:CARGO(?:_|$)|RUST|CC(?:_|$)|CXX(?:_|$)|AR(?:_|$)|LD(?:_|$)|TARGET_|HOST_|MACOSX_DEPLOYMENT_TARGET$|SDKROOT$|SOURCE_DATE_EPOCH$|PATH$|HOME$|TMPDIR$)/ &&
        $_ !~ /\ACARGO_(?:MAKEFLAGS|RELEASE_METADATA_EDITOR|RELEASE_METADATA_MAX_BYTES)\z/
} keys %ENV;
$environment{CARGO_NET_OFFLINE} = sha256_hex('true');
$environment{RUSTUP_AUTO_INSTALL} = sha256_hex('0');
write_json($context_path, {schema=>1,environment=>\%environment,source=>$source,group=>$group,tree=>scalar(git('rev-parse',"$source^{tree}")),
    selection=>$narrow?'focused':'full',baseline=>$baseline ? $baseline->{source} : undef,reason=>$reason,
    host=>scalar(command('uname','-sm')),changes=>\@changes});
write_json($plan_path, {schema=>1,source=>$source,group=>$group,scope=>'group',policy=>'ci/release-validation.json',
    inputs=>\@inputs,stages=>\@stages});
# Re-admission probes are disposable after success, not new qualification
# artifacts. Failed probes and the original plan's observations remain retained.
if ($check && defined $scratch) {
    remove_tree($scratch, {error => \my $errors});
    !@$errors or die "cannot remove owned selection probes: $scratch\n";
}
print "$reason\n";
