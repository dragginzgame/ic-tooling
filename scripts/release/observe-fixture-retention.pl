#!/usr/bin/env perl
use strict;
use warnings;
use Cwd qw(abs_path);
use File::Find;
use JSON::PP;

# Retention checks only shell initialization/EXIT prefixes. Evidence checks use
# synthetic archive inputs. Neither reads package manifests, locks or ledgers.
# Enumerating every shell input is deliberately conservative for both stages.
@ARGV == 0 or die "usage: observe-fixture-retention.pl\n";
my $stage = $ENV{TOOLING_FIXTURE_STAGE} // die "missing fixture stage\n";
$stage =~ /\Afixture-(?:retention|evidence)\z/ or die "unknown fixture stage\n";
my %files = map { $_ => 1 } qw(ci/tooling-checks.tsv scripts/release/observe-fixture-retention.pl
    scripts/ci/select-tooling-checks.sh scripts/release/reuse-fixture.sh);
find({no_chdir=>1,wanted=>sub {
    return unless /\.sh\z/;
    -f $_ && !-l $_ or die "nonregular fixture shell input: $_\n";
    $files{$_}=1;
}}, 'scripts');
my %tools;
for my $name (qw(awk bash cat cmp cp dirname env grep gzip ln mkdir mktemp mv perl rg rm sed tar tee)) {
    my ($path) = grep { -f $_ && -x $_ } map { "$_/$name" } split /:/, $ENV{PATH};
    defined $path or die "fixture tool missing: $name\n";
    $tools{$name}=abs_path($path) // die "fixture tool unresolved: $name\n";
    $files{$tools{$name}}=1;
}
$files{abs_path($_)}=1 for values %INC;
print JSON::PP->new->canonical->utf8->encode({schema=>1,files=>[sort keys %files],
    configuration=>{contract=>"tooling-$stage/1",tools=>\%tools}}),"\n";
