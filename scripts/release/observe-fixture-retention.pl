#!/usr/bin/env perl
use strict;
use warnings;
use Cwd qw(abs_path);
use File::Find;
use JSON::PP;

# This fixture renders only shell initialization/EXIT prefixes and stops before
# Cargo, Git, setup and product fixture bodies. Package manifests, locks and
# ledgers are not read. Enumerating every shell input is deliberately conservative.
@ARGV == 0 or die "usage: observe-fixture-retention.pl\n";
my %files = map { $_ => 1 } qw(ci/tooling-checks.tsv scripts/release/observe-fixture-retention.pl);
find({no_chdir=>1,wanted=>sub {
    return unless /\.sh\z/;
    -f $_ && !-l $_ or die "nonregular fixture shell input: $_\n";
    $files{$_}=1;
}}, 'scripts');
my %tools;
for my $name (qw(awk bash cat cp dirname env grep ln mkdir mktemp rg rm sed tee)) {
    my ($path) = grep { -f $_ && -x $_ } map { "$_/$name" } split /:/, $ENV{PATH};
    defined $path or die "fixture tool missing: $name\n";
    $tools{$name}=abs_path($path) // die "fixture tool unresolved: $name\n";
    $files{$tools{$name}}=1;
}
$files{abs_path($_)}=1 for values %INC;
print JSON::PP->new->canonical->utf8->encode({schema=>1,files=>[sort keys %files],
    configuration=>{contract=>'tooling-fixture-retention/1',tools=>\%tools}}),"\n";
