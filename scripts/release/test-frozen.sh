#!/usr/bin/env bash
# Installed consumer qualification. Actual metadata/Git/publication-dry-run hooks;
# product gate bodies are substituted here and remain owned by the live gate.
set -euo pipefail
[[ $# == 0 ]] || exit 2
root=$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES TOKO_TOOLING_ROOT
unset RELEASE_GROUP RELEASE_GROUPS RELEASE_PUBLISH_TARGET RELEASE_SETUP_TARGET
unset RELEASE_SETUP_MODE RELEASE_BATCH_ACTIVE
unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE GIT_COMMON_DIR GIT_OBJECT_DIRECTORY GIT_ALTERNATE_OBJECT_DIRECTORIES
unset CARGO_TARGET_DIR CARGO_HOME
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
# shellcheck source=scripts/ci/product-fixture.sh
source "$root/scripts/ci/product-fixture.sh"
fixture=$(tooling_fixture_create "$root" frozen-release)
# Keep installed archives, durable candidate Git and every recovery observation.
trap 'status=$?; printf "Frozen hook evidence retained: %s\n" "$fixture" >&2; exit "$status"' EXIT
normal="$fixture/normal"
mkdir "$normal"
git -C "$root" archive HEAD | tar -xf - -C "$normal"
# Retired managed files must not survive from the committed fixture baseline.
git -C "$root" diff HEAD --name-only --diff-filter=D -z > "$fixture/consumer-deletions"
while IFS= read -r -d '' path; do
    rm -- "$normal/$path"
done < "$fixture/consumer-deletions"
# Adopted managed payloads may be pending delivery. Qualify those exact verified
# bytes rather than accidentally testing the preceding committed supplier.
bash "$root/scripts/ci/verify-toko-tooling-snapshot.sh" --consumer "$root"
bash "$root/scripts/ci/verify-toko-tooling-snapshot.sh" --read-manifest "$root/.toko-tooling.snapshot" > "$fixture/snapshot-records"
while IFS=$'\t' read -r kind _digest _mode path; do
    [[ $kind == file ]] || continue
    mkdir -p "$normal/$(dirname "$path")"
    cp -p "$root/$path" "$normal/$path"
done < "$fixture/snapshot-records"
cp "$root/.toko-tooling.snapshot" "$normal/.toko-tooling.snapshot"
# Include current consumer inputs in this disposable fixture. Its own commit
# selects the captured graph; deferred normal-tree edits are injected below.
git -C "$root" diff HEAD --name-only --diff-filter=d -z > "$fixture/consumer-changes"
git -C "$root" ls-files --others --exclude-standard -z >> "$fixture/consumer-changes"
while IFS= read -r -d '' path; do
    [[ $path != Cargo.lock ]] || continue
    [[ -f "$root/$path" && ! -L "$root/$path" ]]
    mkdir -p "$normal/$(dirname "$path")"
    cp -p "$root/$path" "$normal/$path"
done < "$fixture/consumer-changes"
cp "$root/Cargo.lock" "$normal/Cargo.lock"
bash "$root/scripts/release/prepare-validation-fixture.sh" "$root" "$normal" --compact
cp "$root/Cargo.lock" "$fixture/deferred-Cargo.lock"
export FROZEN_TEST_NORMAL="$normal" FROZEN_TEST_RECORD="$fixture"
export FROZEN_TEST_PLATFORM
FROZEN_TEST_PLATFORM=$("$root/.tools/host/bin/yq" -p toml -o json -r '.package[] | select(.name == "ic-platform") | .version' "$normal/Cargo.lock")
cat > "$normal/scripts/release/validation-stage.sh" <<'STAGE'
#!/usr/bin/env bash
set -euo pipefail
platform=$("$YQ" -p toml -o json -r '.package[] | select(.name == "ic-platform") | .version' Cargo.lock)
[[ $platform == "$FROZEN_TEST_PLATFORM" ]]
printf '%s %s %s\n' "$RELEASE_GROUP" "$1" "$platform" >> "$FROZEN_TEST_RECORD/stages"
if [[ $RELEASE_GROUP == text && $1 == toko-tooling-check && -f "$FROZEN_TEST_RECORD/fail-once" ]]; then
    cp "$FROZEN_TEST_RECORD/deferred-Cargo.lock" "$FROZEN_TEST_NORMAL/Cargo.lock"
    printf '\n# frozen qualification deferred edit\n' >> "$FROZEN_TEST_NORMAL/Cargo.lock"
    rm "$FROZEN_TEST_RECORD/fail-once"
    exit 143
fi
if [[ $1 == tooling-command-check ]]; then
    while IFS=$'\t' read -r stage _check; do
        bash scripts/release/reuse-fixture.sh "$RELEASE_GROUP" "$stage"
    done < <(bash scripts/ci/select-tooling-checks.sh reusable)
fi
STAGE
cat >> "$normal/Makefile" <<'MAKE'

# This fixture isolates metadata/recovery/publication hooks. The independent
# installed release qualifies product gates and server setup without substitutes.
LOCAL_TOOL_INSTALL_TARGETS := install-release-editor
LOCAL_TOOL_CHECK_TARGETS := release-editor-check
# Workspace lint is a substituted product gate; archive/provenance hooks stay real.
clippy-workspace:
	+@printf '%s\n' '$(RELEASE_GROUP) publication-lint' >> "$$FROZEN_TEST_RECORD/stages"
publish:
	+CARGO_NET_OFFLINE=false bash scripts/publish/workspace.sh check
	+@printf '%s\n' '$(RELEASE_GROUP)' >> "$$FROZEN_TEST_RECORD/publications"
	+@if [ '$(RELEASE_GROUP)' = text ] && [ -f "$$FROZEN_TEST_RECORD/publication-stop-once" ]; then rm "$$FROZEN_TEST_RECORD/publication-stop-once"; exit 143; fi
MAKE
for group in text testkit; do
    ledger="$normal/releases/$group/CHANGELOG.md"
    version=$(cd "$normal" && bash scripts/release/group.sh ci/cargo-release-groups.json "$group" version)
    next=$(perl "$root/scripts/ci/validate-release-inputs.pl" next-version "$version" patch)
    if ! grep -Fxq "## [$next] - Unreleased" "$ledger" && ! grep -Fxq "## [$next]" "$ledger"; then
        { printf '# Frozen consumer fixture\n\n## [%s] - Unreleased\n\n- Qualify captured hooks.\n\n' "$next"; cat "$ledger"; } > "$fixture/ledger"
        cp "$fixture/ledger" "$ledger"
    fi
done
git init -q -b main "$normal"
git -C "$normal" config user.name 'Tooling qualification'
git -C "$normal" config user.email 'qualification@example.invalid'
git -C "$normal" add -- .
git -C "$normal" commit -qm 'Reproducible Tooling frozen-hook fixture'
git init -q --bare "$fixture/remote.git"
git -C "$normal" remote add origin "$fixture/remote.git"
git -C "$normal" push -q origin main
source=$(git -C "$normal" rev-parse HEAD)
[[ ! -e "$normal/.tools" ]]
all_groups=$(bash "$normal/scripts/release/group.sh" "$normal/ci/cargo-release-groups.json" --list)
for group in $all_groups; do
    make --no-print-directory -s -C "$normal" release-version "RELEASE_GROUP=$group" > "$fixture/version-$group"
done
if (cd "$normal" && bash scripts/release/group.sh ci/cargo-release-groups.json build,foundation batch) > "$fixture/order-refusal.log" 2>&1; then exit 1; fi
: > "$fixture/fail-once"
: > "$fixture/publication-stop-once"
if make --no-print-directory -C "$normal" release-patch RELEASE_GROUPS=text,testkit > "$fixture/interrupted.log" 2>&1; then exit 1; fi
intents=("$normal"/.git/release-state/frozen/batch-*/intent.json)
[[ ${#intents[@]} == 1 && -f ${intents[0]} ]]
jq -e '.phase == "releasing" and .index == 0' "${intents[0]}" > /dev/null
batch=$(basename "${intents[0]%/*}")
[[ ! -e "$fixture/fail-once" ]]
[[ $(git -C "$normal" rev-parse HEAD) == "$source" ]]
cp "$normal/Cargo.lock" "$fixture/expected-deferred-Cargo.lock"
if make --no-print-directory -C "$normal" release-batch-resume "BATCH=$batch" > "$fixture/resumed.log" 2>&1; then exit 1; fi
jq -e '.schema == 2 and .phase == "publishing" and .index == 0 and
    (.release_commit | test("^[0-9a-f]{40}$"))' "${intents[0]}" > /dev/null
printf 'text\n' > "$fixture/expected-publications"
cmp "$fixture/expected-publications" "$fixture/publications"
if make --no-print-directory -C "$normal" release-batch-resume "BATCH=$batch" > "$fixture/uncertain-publication.log" 2>&1; then exit 1; fi
cmp "$fixture/expected-publications" "$fixture/publications"
make --no-print-directory -C "$normal" release-batch-resume "BATCH=$batch" PUBLICATION_RESULT=published > "$fixture/reconciled.log" 2>&1
jq -e '.phase == "complete" and .index == 2' "${intents[0]}" > /dev/null
work="$normal/target/release-frozen/$batch"
gitdir="$normal/.git/release-state/frozen/$batch/repository.git"
[[ $(git -C "$normal" rev-parse HEAD) == "$source" ]]
cmp "$normal/Cargo.lock" "$fixture/expected-deferred-Cargo.lock"
printf 'text\ntestkit\n' > "$fixture/expected-publications"
cmp "$fixture/expected-publications" "$fixture/publications"
grep -q '^testkit ' "$fixture/stages"
[[ $(git --git-dir "$fixture/remote.git" rev-parse main) == "$(git --git-dir "$gitdir" rev-parse HEAD)" ]]
for group in text testkit; do
    package=ic-testkit
    [[ $group != text ]] || package=ic-host-text
    version=$(make --no-print-directory -s -C "$work" release-version "RELEASE_GROUP=$group")
    # Cargo retains publication dry-run archives in tmp-crate.
    archive="$work/target/package/tmp-crate/$package-$version.crate"
    sha=$(git --git-dir "$gitdir" rev-parse "$group-v$version^{}")
    tar -xOf "$archive" "$package-$version/.cargo_vcs_info.json" |
        jq -e --arg sha "$sha" '.git.sha1 == $sha and (.git.dirty // false) == false' > /dev/null
done
make --no-print-directory -C "$normal" release-batch-resume "BATCH=$batch" > "$fixture/complete-resume.log" 2>&1
cmp "$fixture/expected-publications" "$fixture/publications"
# A stale normal source must not overwrite tags delivered by the earlier batch.
git -C "$normal" restore --source=HEAD -- Cargo.lock
before=$(git --git-dir "$fixture/remote.git" rev-parse main)
if make --no-print-directory -C "$normal" release-patch RELEASE_GROUP=text > "$fixture/remote-conflict.log" 2>&1; then exit 1; fi
[[ $(git --git-dir "$fixture/remote.git" rev-parse main) == "$before" ]]
cmp "$fixture/expected-publications" "$fixture/publications"
echo 'Frozen Tooling hooks passed: five group queries, order refusal, deferred lock, interrupted/uncertain recovery, two archive identities, completed resume and remote conflict'
