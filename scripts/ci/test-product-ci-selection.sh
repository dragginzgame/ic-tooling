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
ok(!grep({ exists $_->{uses} } @{$workflow->{jobs}{msrv}{steps}}), "compiler gate performs no duplicate setup or compilation");
my $full = q|${{ needs.change-scope.outputs.scope == 'full' }}|;
my $qualification = q|${{ needs.change-scope.outputs.scope == 'full' && github.event_name == 'workflow_dispatch' }}|;
my $docs = q|${{ needs.change-scope.outputs.scope == 'docs' }}|;
my @guards;
my @documentation;
for my $name (qw(native msrv)) {
    my $job = $workflow->{jobs}{$name};
    if ($name eq "native") { is($job->{needs}, "change-scope", "native depends on classification"); }
    else { is_deeply($job->{needs}, ["change-scope", "native"], "compiler gate depends on classification and all native proofs"); }
    is($job->{if}, '${{ !cancelled() }}', "$name does not silently skip failed classification");
    for my $step (@{$job->{steps}}) {
        my $label = $step->{name} // $step->{uses};
        if ($label eq 'Require a successful classification') {
            is($step->{env}{CLASSIFICATION_RESULT}, '${{ needs.change-scope.result }}', 'guard observes job result');
            is($step->{env}{CHANGE_SCOPE}, '${{ needs.change-scope.outputs.scope }}', 'guard observes scope');
            push @guards, $step->{run};
        } elsif ($label eq 'Validate documentation only') {
            is($step->{if}, $docs, "$name selects the documentation command");
            push @documentation, $step->{run} if $name eq "native";
        } elsif ($label =~ /^actions\/checkout\@/ || $label eq 'Verify native host') {
            # Checkout and host identity are cheap and run for both profiles.
        } elsif ($label =~ /^Retain .*evidence$/) {
            like($step->{if}, qr/needs\.change-scope\.outputs\.scope == 'full'/, 'failure artifacts require full qualification');
        } elsif ($label eq 'Qualify focused release contracts' || $label eq 'Retain release-validation comparison') {
            is($step->{if}, $qualification, 'matched-input comparison runs only for explicit qualification');
        } else {
            is($step->{if}, $full, "$name gates $label on full qualification");
        }
    }
}
is(scalar @guards, 2, 'both stable jobs guard classification');
is(scalar @documentation, 1, 'native owns document validation; compiler status reuses that result');
is_deeply($workflow->{jobs}{native}{strategy}{matrix}{include}, [
    {runner => 'ubuntu-24.04', os => 'Linux', architecture => 'x86_64'},
    {runner => 'macos-15-intel', os => 'Darwin', architecture => 'x86_64'},
    {runner => 'macos-15', os => 'Darwin', architecture => 'arm64'},
], 'full qualification retains all three native hosts and stable matrix identities');
ok(!exists $workflow->{on}{pull_request}{paths} && !exists $workflow->{on}{push}{paths}, 'no workflow path filters');
ok(!exists $workflow->{jobs}{'lint-and-security'}{if}, 'lint and security remain active');

# Execute the actual full/focused workflow bodies, substituting only the tools.
# A required failure must stop later owners and archive retention.
{
    my ($complete) = grep { ($_->{name} // '') eq 'Run native CI and publication qualification' } @{$workflow->{jobs}{native}{steps}};
    my ($focused) = grep { ($_->{name} // '') eq 'Qualify focused release contracts' } @{$workflow->{jobs}{native}{steps}};
    my $checkout = "$fixture/matched inputs";
    make_path("$checkout/bin", "$checkout/ci", "$checkout/scripts/ci");
    run('bash', "$root/scripts/ci/prepare-script-fixture.sh", $root, $checkout, 'scripts/ci/validation-roster.sh');
    write_file("$checkout/bin/make", <<'SH');
#!/usr/bin/env bash
set -euo pipefail
[[ "$*" == ci-publication ]]
printf 'workspace\n' >> "$WORKFLOW_EVENTS"
[[ ${WORKFLOW_FAIL:-} != workspace ]] || exit 29
SH
    write_file("$checkout/scripts/ci/run-validation-targets.sh", <<'SH');
#!/usr/bin/env bash
set -euo pipefail
[[ $VALIDATION_LOG_DIR == "$RUNNER_TEMP/release-validation/$RELEASE_GROUP" ]]
printf '%s\t%s\n' "$RELEASE_GROUP" "$*" >> "$WORKFLOW_EVENTS"
[[ ${WORKFLOW_FAIL:-} != "$RELEASE_GROUP" ]] || exit 29
SH
    write_file("$checkout/scripts/ci/archive-product-evidence.sh", <<'SH');
#!/usr/bin/env bash
set -euo pipefail
[[ $# == 3 && $1 == "$PWD" && $2 == "$RUNNER_TEMP" && $3 == "$RUNNER_TEMP/release-validation/archives.tar.gz" ]]
printf 'archives\n' >> "$WORKFLOW_EVENTS"
SH
    chmod 0755, "$checkout/bin/make" or die $!;
    write_file("$checkout/qualify.sh", "$complete->{run}\n$focused->{run}");
    local $ENV{PATH} = "$checkout/bin:$ENV{PATH}";
    local $ENV{WORKFLOW_EVENTS} = "$checkout/events";
    local $ENV{RUNNER_TEMP} = "$checkout/runner";
    local $ENV{TMPDIR} = "$checkout/temporary";
    my $targets = join ' ', map { "release-scope-$_" } qw(tools-check lint graphs tests docs archives);
    my @expected = ('workspace', (map { "$_\t$targets" } qw(native text tools build)), 'archives');
    chdir $checkout or die $!;
    for my $failure ('', qw(workspace native text tools build)) {
        local $ENV{WORKFLOW_FAIL} = $failure;
        write_file($ENV{WORKFLOW_EVENTS}, '');
        my $status = system('bash', '-e', '-o', 'pipefail', "$checkout/qualify.sh");
        is($status >> 8, $failure ? 29 : 0, "matched-input workflow propagates $failure status");
        my @events = split /\n/, read_file($ENV{WORKFLOW_EVENTS});
        my %stops = (workspace => 1, native => 2, text => 3, tools => 4, build => 5);
        my $count = $stops{$failure} // scalar @expected;
        is_deeply(\@events, [@expected[0 .. $count-1]], "matched-input workflow stops at $failure boundary");
    }
    chdir $root or die $!;
}

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
run('bash', "$root/scripts/ci/prepare-script-fixture.sh", $root, $repo,
    'scripts/ci/classify-ci-change.pl');
for my $file (qw(check-product-doc-links.sh check-documentation-links.pl)) {
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
is(scope($base, $doc_head, 'workflow_dispatch'), 'full', 'explicit qualification retains the complete gate even for prose');
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
            local @ENV{qw(CLASSIFICATION_RESULT CHANGE_SCOPE NATIVE_RESULT)} = ($result, $scope, "success");
            # Refusal must be explicit: Bash 3.2 does not apply errexit to [[ ]].
            my $passed = system('bash', '-o', 'pipefail', '-c', $command) == 0;
            is($passed, $result eq 'success' && ($scope eq 'docs' || $scope eq 'full'), 'actual job guard fails closed');
        }
    }
}
my ($compiler_guard) = grep { /NATIVE_RESULT/ } @guards;
ok($compiler_guard, 'compiler status admits native qualification results');
for my $native_result ('failure', 'cancelled', 'skipped', '') {
    local @ENV{qw(CLASSIFICATION_RESULT CHANGE_SCOPE NATIVE_RESULT)} = ('success', 'full', $native_result);
    isnt(system('bash', '-o', 'pipefail', '-c', $compiler_guard), 0, 'compiler status rejects absent or failed native proof');
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
