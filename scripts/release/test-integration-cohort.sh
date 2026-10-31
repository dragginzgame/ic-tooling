#!/usr/bin/env bash
set -euo pipefail

# Opt-in archive consumer proof against the current published foundation.
# Candidate versions exist only in a disposable workspace; no release/upload.
[[ $# == 1 && -x "$1" ]] || {
    echo 'usage: test-integration-cohort.sh BUILT_RELEASE_EDITOR' >&2
    exit 2
}
editor="$1"
[[ "$editor" == /* ]] || editor="$PWD/$editor"
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/integration-cohort.XXXXXX")"
fixture_complete=false
# Retain successful candidates too: these are a reviewable publication plan.
trap 'status=$?; fixture_completion_status "$status" "$fixture_complete" || status=$?; printf "Integration cohort evidence: %s\n" "$fixture" >&2; exit "$status"' EXIT
export RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/target}"
parser="${YQ:-yq}"
"$parser" -p toml -o json '.' "$root/Cargo.toml" > "$fixture/source.json"
foundation="$("$parser" -p toml -o json '.package.version' "$root/crates/ic-build/Cargo.toml" | jq -er '.')"
previous="$("$parser" -p toml -o json '.package.version' "$root/crates/ic-testkit/Cargo.toml" | jq -er '.')"
next="$(bash "$root/scripts/ci/next-release-version.sh" "$previous" minor)"
(cd "$root"; RELEASE_VERSION="$next" bash scripts/ci/cargo-release-group.sh plan integration) \
    > "$fixture/group-plan.json"

mkdir -p "$fixture/crates" "$fixture/consumer/src" "$fixture/before"
mkdir -p "$fixture/serializer/src"
# Construct fixtures with the existing TOML serializer; the Build editor below
# remains the owner of selected release edits. No ad-hoc TOML writer is used.
"$parser" -p toml -o json '.' "$root/Cargo.lock" > "$fixture/source-lock.json"
toml_version="$(jq -er '.package[] | select(.name == "toml" and (.version | startswith("1."))) | .version | split("+")[0]' "$fixture/source-lock.json")"
json_version="$(jq -er '.package[] | select(.name == "serde_json") | .version' "$fixture/source-lock.json")"
types_version="$(jq -er '[.package[] | select(.name == "ic-platform-types")] |
    if length == 1 then .[0].version else error("expected one canonical Types supplier") end' "$fixture/source-lock.json")"
cat > "$fixture/serializer/Cargo.toml" <<EOF
[package]
name = "cohort-fixture-serializer"
version = "0.0.0"
edition = "2024"
publish = false
[workspace]
[dependencies]
toml = "=$toml_version"
serde_json = "=$json_version"
EOF
cat > "$fixture/serializer/src/main.rs" <<'RUST'
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value: serde_json::Value = serde_json::from_reader(std::io::stdin().lock())?;
    print!("{}", toml::to_string_pretty(&value)?);
    Ok(())
}
RUST
cargo +1.91.0 build --offline --manifest-path "$fixture/serializer/Cargo.toml" \
    > "$fixture/serializer.log" 2>&1
serializer="$CARGO_TARGET_DIR/debug/cohort-fixture-serializer"
cp -a "$root/crates/ic-host-tools" "$root/crates/ic-testkit" "$root/crates/ic-testkit-native" "$fixture/crates/"
cp "$root/README.md" "$root/LICENSE" "$fixture/"
# An archive consumer uses registry foundation packages, as Cargo's normalized
# publication manifests do. Preserve their actual requirement literals.
jq '
    .workspace.members = ["crates/ic-host-tools", "crates/ic-testkit", "crates/ic-testkit-native", "consumer"]
    | del(.workspace["default-members"])
    | reduce ["ic-host-artifacts", "ic-host-fs", "ic-host-process", "ic-host-text", "ic-build"][] as $name
        (. ; del(.workspace.dependencies[$name].path))
' "$fixture/source.json" | "$serializer" > "$fixture/Cargo.toml"
cp "$fixture/Cargo.toml" "$fixture/before/Cargo.toml"
jq '.manifests[] | select(.path == "Cargo.toml") | {version,dependencies}' \
    "$fixture/group-plan.json" > "$fixture/root-selection.json"
"$editor" --max-input-bytes 8388608 group-manifest "$fixture/Cargo.toml" \
    "$previous" "$next" "$fixture/root-selection.json" > "$fixture/root.candidate"
mv "$fixture/root.candidate" "$fixture/Cargo.toml"
printf '%s\n' '{"version":"package","dependencies":[]}' > "$fixture/member-selection.json"
for package in ic-host-tools ic-testkit ic-testkit-native; do
    manifest="$fixture/crates/$package/Cargo.toml"
    # Both real groups already own literal package versions.
    cp "$manifest" "$fixture/before/$package.toml"
    "$editor" --max-input-bytes 8388608 group-manifest "$manifest" \
        "$previous" "$next" "$fixture/member-selection.json" > "$manifest.candidate"
    mv "$manifest.candidate" "$manifest"
    "$parser" -p toml -o json '.' "$fixture/before/$package.toml" > "$fixture/before/member.json"
    "$parser" -p toml -o json '.' "$manifest" |
        jq -e --slurpfile before "$fixture/before/member.json" \
            'del(.package.version) == ($before[0] | del(.package.version))' > /dev/null
done
cat > "$fixture/consumer/Cargo.toml" <<EOF
[package]
name = "integration-cohort-consumer"
version = "$foundation"
edition = "2024"
publish = false
[dependencies]
ic-build = "=$foundation"
ic-host-artifacts = "=$foundation"
ic-host-process = "=$foundation"
ic-host-tools = { path = "../crates/ic-host-tools", version = "=$next", features = ["registry"] }
ic-testkit = { path = "../crates/ic-testkit", version = "=$next", features = ["wire"] }
ic-testkit-native = { path = "../crates/ic-testkit-native", version = "=$next", features = ["stable-memory"] }
ic-platform = { workspace = true, features = ["principal"] }
ic-platform-types = { version = "=$types_version", default-features = false }
ic-wire = { workspace = true }
EOF
cat > "$fixture/consumer/src/lib.rs" <<'RUST'
#[cfg(test)]
mod tests {
    #[test]
    fn published_build_keeps_its_public_foundation_error_identities() {
        use ic_build::artifacts::candid::ExtractionError;
        let _: fn(ic_host_artifacts::artifact::ArtifactError) -> ExtractionError =
            ExtractionError::Input;
        let _: fn(ic_host_process::tool::ToolError) -> ExtractionError = ExtractionError::Tool;
    }

    #[test]
    fn integration_callers_share_canonical_identity_and_core_wire_limits() {
        let owner: ic_platform_types::Principal =
            ic_host_tools::registry::routing_shards::canister_range_start_from_key(
                "prefix/", "prefix/00000000000000000101",
            ).unwrap();
        let facade: ic_platform::Principal = owner;
        let _: ic_platform_types::Principal = facade;
        let limits = ic_wire::Limits {
            bytes: 1024, allocation: 16384, nodes: 1024, depth: 16,
        };
        let call = ic_testkit::pic::WireCallSpec {
            method: "probe", method_id: 1,
            request_limits: limits, response_limits: limits,
        };
        let _: ic_wire::Limits = call.request_limits;
        let _: ic_wire::Limits = call.response_limits;
        let request = ic_wire::native::encode_request(call.method_id, &facade, limits).unwrap();
        let decoded: ic_platform::Principal =
            ic_wire::native::decode_request(call.method_id, &request, limits).unwrap();
        assert_eq!(decoded, facade);
        let account = ic_platform::Account { owner: facade, subaccount: None };
        let response = ic_wire::native::encode_response(call.method_id, &account, limits).unwrap();
        let decoded: ic_platform::Account =
            ic_wire::native::decode_response(call.method_id, &response, limits).unwrap();
        assert_eq!(decoded, account);
        assert!(decoded.subaccount.is_none());
        let memory: ic_testkit_native::memory::MeteredMemory = Default::default();
        assert_eq!(ic_platform::stable_structures::Memory::size(&memory), 0);
    }
}
RUST

# Resolve the proposed consumer, not the real workspace or its release state.
cargo +1.91.0 generate-lockfile --offline --manifest-path "$fixture/Cargo.toml" \
    > "$fixture/lock.log" 2>&1
cargo +1.91.0 metadata --locked --offline --format-version 1 --manifest-path "$fixture/Cargo.toml" \
    > "$fixture/metadata.json" 2> "$fixture/metadata.stderr"
jq -e --arg next "$next" '
    . as $m
    | [.packages[] | select(.id as $id | $m.workspace_members | index($id)) | select(.publish != [])]
        | (map(.name) | sort) == ["ic-host-tools", "ic-testkit", "ic-testkit-native"] and all(.[]; .version == $next)
' "$fixture/metadata.json" > /dev/null
jq -e --arg types "$types_version" '
    [.packages[] | select(.name == "ic-platform-types")] |
    length == 1 and all(.[]; .version == $types)
' "$fixture/metadata.json" > /dev/null
jq -e --arg foundation "$foundation" '
    .packages as $packages
    | all(["ic-build", "ic-host-artifacts", "ic-host-process", "ic-host-fs"][];
        . as $name | [$packages[] | select(.name == $name)]
        | length == 1 and all(.[]; .version == $foundation and (.source | startswith("registry+"))))
' "$fixture/metadata.json" > /dev/null
"$parser" -p toml -o json '.' "$fixture/Cargo.toml" > "$fixture/candidate.json"
"$parser" -p toml -o json '.' "$fixture/before/Cargo.toml" > "$fixture/before.json"
jq -e --slurpfile before "$fixture/before.json" '
    (.workspace.package | has("version") | not)
    and ([.workspace.dependencies | del(.["ic-testkit"].version)]
        == [$before[0].workspace.dependencies | del(.["ic-testkit"].version)])
' "$fixture/candidate.json" > /dev/null
cargo +1.91.0 test --locked --offline --manifest-path "$fixture/Cargo.toml" \
    -p integration-cohort-consumer > "$fixture/consumer.log" 2>&1
# Package each real candidate, then compile the same public callers against
# the extracted normalized manifests, without any workspace source patches.
archive_consumer="$fixture/archive-consumer"
mkdir -p "$archive_consumer/consumer/src" "$archive_consumer/extracted"
for package in ic-host-tools ic-testkit ic-testkit-native; do
    cargo +1.91.0 package --locked --offline --allow-dirty --no-verify \
        --manifest-path "$fixture/Cargo.toml" --target-dir "$fixture/archives" -p "$package" \
        >> "$fixture/package.log" 2>&1
    tar -xzf "$fixture/archives/package/$package-$next.crate" -C "$archive_consumer/extracted"
done
jq --arg next "$next" '
    .workspace.members = (["consumer"] + (["ic-host-tools","ic-testkit","ic-testkit-native"] | map("extracted/" + . + "-" + $next)))
    | del(.workspace.dependencies["ic-testkit"].path, .workspace.dependencies["ic-testkit-perf-probe"])
' "$fixture/candidate.json" | "$serializer" > "$archive_consumer/Cargo.toml"
sed -e "s|../crates/ic-host-tools|../extracted/ic-host-tools-$next|g" \
    -e "s|../crates/ic-testkit-native|../extracted/ic-testkit-native-$next|g" \
    -e "s|../crates/ic-testkit|../extracted/ic-testkit-$next|g" \
    "$fixture/consumer/Cargo.toml" > "$archive_consumer/consumer/Cargo.toml"
cp "$fixture/consumer/src/lib.rs" "$archive_consumer/consumer/src/"
cargo +1.91.0 generate-lockfile --offline --manifest-path "$archive_consumer/Cargo.toml" \
    > "$fixture/archive-lock.log" 2>&1
cargo +1.91.0 test --locked --offline --manifest-path "$archive_consumer/Cargo.toml" \
    -p integration-cohort-consumer > "$fixture/archive-consumer.log" 2>&1
jq --arg foundation "$foundation" --arg integration "$next" '
    {foundation:$foundation, integration:$integration,
     packages:[.packages[] | select(.source == null and .publish != []) | {name,version}],
     status:"disposable source and archive consumer proof; no release or publication"}
' "$fixture/metadata.json" > "$fixture/publication-plan.json"
cat "$fixture/publication-plan.json"
echo 'Selected integration manifests, packaged consumers and published foundation passed'
fixture_complete=true
