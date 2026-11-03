#!/usr/bin/env perl
# Shared companions: scripts/ci/validation-evidence.pl scripts/ci/ValidationPlan.pm
use strict;
use warnings;
use JSON::PP;
use File::Find;
use Cwd qw(getcwd);
use FindBin qw($RealBin);
use lib "$RealBin/../ci";
use ValidationPlan qw(document digest);

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
my @files;
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
print "tooling-group-gate-2\n$source\n$group\n$gate\n";
for my $file (sort @files) { print "$file\t", digest($root, $file), "\n"; }
