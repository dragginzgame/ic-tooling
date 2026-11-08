#!/usr/bin/env perl
use strict;
use warnings;
use Cwd qw(abs_path);
use File::Find;
use JSON::PP;
use File::Copy;
use File::Path;
use Test::More ();

# Retention checks only shell initialization/EXIT prefixes. Evidence checks use
# synthetic archive inputs. Neither reads package manifests, locks or ledgers.
# Workflow and scratch matrices likewise use synthetic metadata. Track their
# actual workflow, Make, policy and script inputs, leaving Cargo metadata checks
# with the fresh per-group fixtures. New files in these trees invalidate reuse.
@ARGV == 0 or die "usage: observe-fixture-retention.pl\n";
my $stage = $ENV{TOOLING_FIXTURE_STAGE} // die "missing fixture stage\n";
$stage =~ /\Afixture-(?:retention|evidence|ci-selection|scratch)\z/ or die "unknown fixture stage\n";
my %files = map { $_ => 1 } qw(ci/tooling-checks.tsv scripts/release/observe-fixture-retention.pl
    scripts/ci/select-tooling-checks.sh scripts/release/reuse-fixture.sh);
my $routing = $stage eq 'fixture-ci-selection' || $stage eq 'fixture-scratch';
find({no_chdir=>1,wanted=>sub {
    return if -d $_ && !-l $_;
    return unless $routing || /\.sh\z/;
    -f $_ && !-l $_ or die "nonregular fixture input: $_\n";
    $files{$_}=1;
}}, $routing ? qw(scripts ci make .github docs rules) : ('scripts'));
if ($routing) {
    $files{$_}=1 for qw(Makefile rust-toolchain.toml .shared-tooling.snapshot DRAGGINZGAME.md);
}
my %tools;
for my $name (qw(awk bash cat cmp cp dirname env grep gzip jq ln mkdir mktemp mv perl rg rm sed tar tee),
    $routing ? qw(chmod cut find git head make pwd sort touch tr uname wc) : ()) {
    my ($path) = grep { -f $_ && -x $_ } map { "$_/$name" } split /:/, $ENV{PATH};
    defined $path or die "fixture tool missing: $name\n";
    $tools{$name}=abs_path($path) // die "fixture tool unresolved: $name\n";
    $files{$tools{$name}}=1;
}
if ($stage eq 'fixture-ci-selection') {
    my $path = abs_path($ENV{YQ} // die "missing fixture yq\n") // die "unresolved fixture yq\n";
    -f $path && -x $path or die "fixture yq unavailable\n";
    $tools{yq}=$path;
    $files{$path}=1;
}
$files{abs_path($_)}=1 for values %INC;
print JSON::PP->new->canonical->utf8->encode({schema=>1,files=>[sort keys %files],
    configuration=>{contract=>"tooling-$stage/1",tools=>\%tools}}),"\n";
