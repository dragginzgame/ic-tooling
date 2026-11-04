#!/usr/bin/env perl
# Shared companions: scripts/ci/validation-evidence.pl scripts/ci/ValidationPlan.pm
use strict;
use warnings;
use JSON::PP;
use File::Find;
use Cwd qw(getcwd);
use FindBin qw($RealBin);
use lib "$RealBin/../ci";
use ValidationPlan qw(document digest relative_path);

# Shared owns execution evidence; Tooling owns the gate pointer and frozen
# receipt inventory used after release-owned version preparation.
my ($operation, $source, $group, $pointer) = @ARGV;
@ARGV == 4 && $operation =~ /\A(?:verify|check|receipt)\z/
    or die "usage: validation-proof.pl verify|check|receipt SOURCE GROUP POINTER\n";
-f $pointer && !-l $pointer && -s $pointer <= 1048576 or die "missing or redirected gate pointer\n";
open my $in, '<:raw', $pointer or die "read gate pointer: $!\n";
my $raw = do { local $/; <$in> };
close $in or die "close gate pointer: $!\n";
my $record = JSON::PP->new->utf8->decode($raw);
$record->{schema} == 1 && $record->{source} eq $source && $record->{group} eq $group
    or die "wrong gate source/group\n";
my $gate = $record->{directory};
$gate =~ m{\A(?:\.git/release-state|target/release-validation)/[A-Za-z0-9_.-]+\z}
    && -d $gate && !-l $gate or die "invalid gate evidence directory\n";
my $root = getcwd();
my $context = document($root, "$gate/context.json");
$context->{source} eq $source && $context->{group} eq $group or die "wrong gate context\n";
my @runs = glob "$gate/logs/validation.*";
@runs == 1 && -d $runs[0] && !-l $runs[0] or die "missing or ambiguous gate execution\n";
my $proof_path = "$gate/shared-proof.json";
my @stage_files;
my $plan = document($root, "$gate/plan.json");
my $requires_retention = grep {
    $_->{target} eq 'release-stage-tooling-command-check' && $_->{required}
} @{$plan->{stages}};
my @admissions = glob "$gate/stage-admissions/*.json";
@admissions == ($requires_retention ? 1 : 0)
    or die "missing or unexpected reusable-stage admission\n";
sub cache_file {
    my ($path) = @_;
    defined($path) && !ref($path) && $path =~ s{\A\Q$root\E/}{}
        or die "stage artifact outside the workspace\n";
    relative_path($path);
    $path =~ m{\Atarget/release-stage-cache/(?:proofs|attempts)/}
        or die "stage artifact outside its owned cache\n";
    return $path;
}
for my $path (@admissions) {
    $path eq "$gate/stage-admissions/fixture-retention.json"
        or die "unknown reusable stage\n";
    my $admission = document($root, $path);
    $admission->{kind} eq 'release-stage-admission' && $admission->{schema} == 1
        && $admission->{source} eq $source && $admission->{group} eq $group
        && $admission->{stage} eq 'fixture-retention'
        && $admission->{contract} eq 'tooling-fixture-retention/1'
        && $admission->{mode} =~ /\A(?:executed|reused)\z/
        or die "wrong reusable-stage identity\n";
    # Shared admits execution/reuse. Tooling seals the unchanged original bytes,
    # including their logs, into its existing recovery custody inventory.
    my $execution = cache_file($admission->{execution_proof});
    digest($root, $execution) eq $admission->{execution_proof_sha256}
        or die "stage execution proof changed before receipt sealing\n";
    push @stage_files, $execution;
    my $original = document($root, $execution);
    for my $stream (qw(stdout stderr)) {
        my $log = $original->{logs}{$stream};
        my $file = cache_file($log->{path});
        digest($root, $file) eq $log->{sha256}
            or die "stage execution log changed before receipt sealing\n";
        push @stage_files, $file;
    }
}
if ($operation ne 'receipt') {
    my @args = ($operation, $root, "$gate/plan.json", $source, $group, $runs[0], "$gate/context.json");
    push @args, $proof_path if $operation eq 'check';
    exec $^X, "$RealBin/../ci/validation-evidence.pl", @args;
    die "start shared evidence verifier: $!\n";
}
my $proof = document($root, $proof_path);
$proof->{kind} eq 'group-validation-evidence' && $proof->{source} eq $source
    && $proof->{group} eq $group && $proof->{run} eq $runs[0]
    or die "wrong shared gate proof\n";
my @files = @stage_files;
{
    local $SIG{__WARN__} = sub { die @_ };
    find({no_chdir=>1,wanted=>sub {
        -l $_ and die "redirected gate evidence\n";
        return if -d $_;
        -f $_ or die "nonregular gate evidence\n";
        push @files, $_;
    }}, $gate);
}
# This inventory is compared with the receipt sealed by the live Shared check.
# It grants no new validation or reuse when prepared versions differ from inputs.
print "tooling-group-gate-3\n$source\n$group\n$gate\n";
for my $file (sort @files) { print "$file\t", digest($root, $file), "\n"; }
