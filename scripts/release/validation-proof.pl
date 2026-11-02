#!/usr/bin/env perl
use strict;
use warnings;
use JSON::PP;
use File::Find;
use File::Spec;
use Digest::SHA qw(sha256_hex);
my ($source, $group, $pointer) = @ARGV;
@ARGV == 3 or die "usage: validation-proof.pl SOURCE GROUP POINTER\n";
my $json = JSON::PP->new->canonical->utf8;
sub read_file {
    my ($file) = @_;
    -f $file && !-l $file or die "missing or redirected gate evidence: $file\n";
    open my $in, '<:raw', $file or die "read $file: $!\n";
    local $/; my $raw = <$in>; close $in or die "close evidence: $!\n"; return $raw;
}
my $record = $json->decode(read_file($pointer));
$record->{schema} == 1 && $record->{source} eq $source && $record->{group} eq $group or die "wrong gate source/group\n";
my $root = $record->{directory};
$root =~ m{\A(?:\.git/release-state|target/release-validation)/[A-Za-z0-9_.-]+\z} && -d $root && !-l $root or die "invalid gate evidence directory\n";
my @files;
find({no_chdir=>1,wanted=>sub {
    -l $_ and die "redirected gate evidence\n";
    return if -d $_;
    -f $_ or die "nonregular gate evidence\n";
    push @files, $_;
}}, $root);
my $plan = $json->decode(read_file("$root/plan.json"));
$plan->{source} eq $source && $plan->{group} eq $group or die "wrong saved plan\n";
my @runs = glob "$root/logs/validation.*";
@runs == 1 && -d $runs[0] && !-l $runs[0] or die "missing or ambiguous gate execution\n";
my $run = $runs[0];
# Match every obligation to the exact admitted plan and an actual successful Make log.
read_file("$root/plan.json") eq read_file("$run/parsed-inputs.json.plan.json") or die "runner selected different plan\n";
read_file($plan->{policy}) eq read_file("$run/parsed-inputs.json.policy.json") or die "runner policy changed\n";
my %events;
for my $line (split /\n/, read_file("$run/plan-events.tsv")) {
    next if $line =~ /^target\tstate\t/;
    my ($target,$state,$detail) = split /\t/, $line, 3;
    $events{$target} = [$state,$detail];
}
my %timings;
for my $line (split /\n/, read_file("$run/timings.tsv")) {
    next if $line =~ /^target\tresult\t/;
    my ($target,$state,$seconds,$log) = split /\t/, $line, 4;
    !exists $timings{$target} or die "duplicate gate timing\n";
    $timings{$target} = [$state,$log];
}
my %expected;
for my $stage (@{$plan->{stages}}) {
    my $target = $stage->{target};
    !$expected{$target}++ or die "duplicate gate obligation\n";
    my $event = $events{$target} // die "missing gate event\n";
    if ($stage->{required}) {
        $event->[0] eq 'PASS' or die "required gate is failed or incomplete: $target\n";
        my $timing = $timings{$target} // die "missing execution evidence\n";
        $timing->[0] eq 'PASS' && $timing->[1] eq $event->[1] or die "mismatched execution evidence\n";
        my $log = File::Spec->rel2abs($timing->[1]);
        my $directory = File::Spec->rel2abs($run);
        index($log, "$directory/") == 0 or die "execution log escapes gate evidence\n";
        read_file($log);
    } else {
        $event->[0] eq 'not-applicable' && !exists $timings{$target} or die "excluded gate was executed or relabelled\n";
    }
}
keys(%expected) == keys(%events) or die "unknown gate event\n";
!grep(!$expected{$_}, keys %timings) or die "unknown gate timing\n";
my $context = $json->decode(read_file("$root/context.json"));
$context->{source} eq $source && $context->{group} eq $group or die "wrong gate context\n";
# Record only observations; no stage reuse is granted by this format.
print "tooling-group-gate-1\n$source\n$group\n$root\n";
for my $file (sort @files) { print "$file\t", sha256_hex(read_file($file)), "\n"; }
