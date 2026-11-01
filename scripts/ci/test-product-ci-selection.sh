#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd -P)"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/tooling-ci-selection.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "CI selection fixture retained"' EXIT
"$root/.tools/host/bin/yq" -o json '.' "$root/.github/workflows/ci.yml" > "$fixture/workflow.json"
perl - "$root" "$fixture" <<'PERL'
use strict;
use warnings;
use JSON::PP qw(decode_json);
use File::Copy qw(copy);
use File::Path qw(make_path);
use Test::More;
my ($root, $fixture) = @ARGV;
sub read_file { open my $f, '<', $_[0] or die $!; local $/; return <$f>; }
sub write_file {
    my ($path, $value) = @_;
    open my $f, '>', $path or die $!;
    print $f $value;
    close $f or die $!;
}
sub run { system(@_) == 0 or die "command failed: @_\n"; }
sub capture {
    open my $f, '-|', @_ or die $!;
    local $/; my $value = <$f>;
    close $f or die "command failed: @_\n";
    $value =~ s/\n\z//;
    return $value;
}
my $workflow = decode_json(read_file("$fixture/workflow.json"));
my ($classifier) = grep { ($_->{id} // '') eq 'classify' } @{$workflow->{jobs}{'change-scope'}{steps}};
ok($classifier, 'workflow has a named classifier');
is($classifier->{env}{CHANGE_HEAD}, '${{ github.sha }}', 'classification uses the tested commit');
is($classifier->{env}{CHANGE_EVENT}, '${{ github.event_name }}', 'classification uses the actual event');
is($classifier->{env}{CHANGE_BASE}, q~${{ github.event_name == 'pull_request' && github.event.pull_request.base.sha || github.event.before }}~, 'comparison starts at the event base');
is($classifier->{env}{CHANGE_REF}, q~${{ github.event_name == 'pull_request' && format('refs/heads/{0}', github.base_ref) || github.ref }}~, 'comparison uses the exact branch or tag ref');
is($workflow->{jobs}{'change-scope'}{steps}[0]{with}{'fetch-depth'}, 0, 'classification has complete comparison history');
my $full = q|${{ needs.change-scope.outputs.scope == 'full' }}|;
my $docs = q|${{ needs.change-scope.outputs.scope == 'docs' }}|;
my @guards;
my @documentation;
for my $name (qw(native msrv)) {
    my $job = $workflow->{jobs}{$name};
    is($job->{needs}, 'change-scope', "$name depends on classification");
    is($job->{if}, '${{ !cancelled() }}', "$name does not silently skip failed classification");
    for my $step (@{$job->{steps}}) {
        my $label = $step->{name} // $step->{uses};
        if ($label eq 'Require a successful classification') {
            is($step->{env}{CLASSIFICATION_RESULT}, '${{ needs.change-scope.result }}', 'guard observes job result');
            is($step->{env}{CHANGE_SCOPE}, '${{ needs.change-scope.outputs.scope }}', 'guard observes scope');
            push @guards, $step->{run};
        } elsif ($label eq 'Validate documentation only') {
            is($step->{if}, $docs, "$name selects the documentation command");
            push @documentation, $step->{run};
        } elsif ($label =~ /^actions\/checkout\@/ || $label eq 'Verify native host') {
            # Checkout and host identity are cheap and run for both profiles.
        } elsif ($label =~ /^Retain .*evidence$/) {
            like($step->{if}, qr/needs\.change-scope\.outputs\.scope == 'full'/, 'failure artifacts require full qualification');
        } else {
            is($step->{if}, $full, "$name gates $label on full qualification");
        }
    }
}
is(scalar @guards, 2, 'both stable jobs guard classification');
is(scalar @documentation, 2, 'both stable jobs validate documents');
is_deeply($workflow->{jobs}{native}{strategy}{matrix}{include}, [
    {runner => 'ubuntu-24.04', os => 'Linux', architecture => 'x86_64'},
    {runner => 'macos-15-intel', os => 'Darwin', architecture => 'x86_64'},
    {runner => 'macos-15', os => 'Darwin', architecture => 'arm64'},
], 'full qualification retains all three native hosts and stable matrix identities');
ok(!exists $workflow->{on}{pull_request}{paths} && !exists $workflow->{on}{push}{paths}, 'no workflow path filters');
ok(!exists $workflow->{jobs}{'lint-and-security'}{if}, 'lint and security remain active');

local @ENV{qw(GIT_CONFIG_GLOBAL GIT_CONFIG_NOSYSTEM GIT_AUTHOR_NAME GIT_AUTHOR_EMAIL GIT_COMMITTER_NAME GIT_COMMITTER_EMAIL)} =
    ('/dev/null', 1, 'CI fixture', 'fixture@example.invalid', 'CI fixture', 'fixture@example.invalid');
delete @ENV{qw(GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE GIT_COMMON_DIR GIT_OBJECT_DIRECTORY GIT_ALTERNATE_OBJECT_DIRECTORIES)};
my $repo = "$fixture/repository";
make_path("$repo/ci", "$repo/src", "$repo/scripts/ci");
my $policy = read_file("$root/ci/docs-only-paths.txt");
my @paths = grep { $_ ne '' && !/^#/ } split /\n/, $policy;
ok(@paths > 0, 'product prose policy is explicit');
for my $path (@paths) {
    unlike(read_file("$root/$path"), qr/Shared companions:/, "$path is not an export companion declaration");
    unlike($path, qr/(?:README|CHANGELOG)\.md\z/, "$path is not packaged documentation or a ledger");
    (my $parent = "$repo/$path") =~ s{/[^/]+\z}{};
    make_path($parent);
    write_file("$repo/$path", "Product prose\n");
}
for my $file (qw(classify-ci-change.pl check-product-doc-links.sh check-documentation-links.pl)) {
    copy("$root/scripts/ci/$file", "$repo/scripts/ci/$file") or die $!;
}
write_file("$repo/ci/docs-only-paths.txt", $policy);
write_file("$repo/src/lib.rs", "// code\n");
chdir $repo or die $!;
run('git', 'init', '-q');
run('git', 'config', 'core.hooksPath', '/dev/null');
run('git', 'config', 'commit.gpgsign', 'false');
sub commit { run('git', 'add', '--all'); run('git', 'commit', '-qm', 'Disposable CI selection'); return capture('git', 'rev-parse', 'HEAD'); }
my $base = commit();
sub restore_base { run('git', 'reset', '--hard', '-q', $base); }
sub scope {
    my ($before, $head, $event, $ref) = @_;
    local @ENV{qw(CHANGE_BASE CHANGE_HEAD CHANGE_EVENT CHANGE_REF GITHUB_OUTPUT)} =
        ($before, $head, $event // 'pull_request', $ref // 'refs/heads/main', "$fixture/scope-output");
    write_file($ENV{GITHUB_OUTPUT}, '');
    run('bash', '-e', '-o', 'pipefail', '-c', $classifier->{run});
    my $output = read_file($ENV{GITHUB_OUTPUT});
    $output =~ /\Ascope=(docs|full)\n\z/ or die "invalid workflow output: $output";
    return $1;
}
write_file($paths[0], "Updated prose\n");
my $doc_head = commit();
is(scope($base, $doc_head), 'docs', 'actual PR classifier command selects product prose');
is(scope($base, $doc_head, 'push'), 'docs', 'actual push classifier command selects product prose');
is(scope($base, $doc_head, 'push', 'refs/tags/foundation-v1.0.0'), 'full', 'tags retain full checks');
is(scope('0' x 40, $doc_head), 'full', 'missing history retains full checks');
for my $command (@documentation) {
    local $ENV{GITHUB_STEP_SUMMARY} = "$fixture/summary";
    run('bash', '-e', '-o', 'pipefail', '-c', $command);
    write_file($paths[0], "[Broken](missing.md)\n");
    isnt(system('bash', '-e', '-o', 'pipefail', '-c', $command), 0, 'actual docs command refuses broken links');
    write_file($paths[0], "Updated prose\n");
}
for my $command (@guards) {
    for my $result (qw(success failure skipped)) {
        for my $scope ('docs', 'full', '', 'unknown') {
            local @ENV{qw(CLASSIFICATION_RESULT CHANGE_SCOPE)} = ($result, $scope);
            my $passed = system('bash', '-e', '-o', 'pipefail', '-c', $command) == 0;
            is($passed, $result eq 'success' && ($scope eq 'docs' || $scope eq 'full'), 'actual job guard fails closed');
        }
    }
}
restore_base(); write_file('src/lib.rs', "// changed\n");
is(scope($base, commit()), 'full', 'code change retains full qualification');
restore_base(); write_file('src/lib.rs', "// changed\n"); write_file($paths[0], "Updated prose\n");
is(scope($base, commit()), 'full', 'mixed change retains full qualification');
restore_base(); write_file('unknown.md', "Unknown prose\n");
is(scope($base, commit()), 'full', 'unknown Markdown retains full qualification');
restore_base(); write_file('ci/docs-only-paths.txt', "$policy# changed\n");
is(scope($base, commit()), 'full', 'policy edit retains full qualification');
done_testing();
PERL
fixture_complete=true
echo 'Product workflow selection, docs checks and fail-closed job guards passed'
