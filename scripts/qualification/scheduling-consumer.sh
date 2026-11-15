#!/usr/bin/env bash
# Run the real pinned Scheduling host consumer in disposable build scratch.
# No sibling writes, registry updates, server installation or release admission.
set -euo pipefail

root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${1:-$root/../ic-scheduling}"
[[ $# -le 1 ]] || { echo 'usage: scheduling-consumer.sh [scheduling-checkout]' >&2; exit 2; }
revision=0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e
source_checkout="$(cd -- "$source_checkout" && pwd -P)"
[[ "$(git -C "$source_checkout" rev-parse "$revision^{commit}")" == "$revision" ]]
case "$root$source_checkout" in *$'\n'*|*$'\r'*) echo 'checkout paths must not contain CR/LF' >&2; exit 2 ;; esac
[[ ! -L "$root/target" && ! -L "$root/target/qualification" ]] || {
    echo 'qualification scratch must not be symlinked' >&2; exit 1;
}
mkdir -p "$root/target/qualification"
attempt="$(mktemp -d "$root/target/qualification/scheduling.XXXXXX")"
echo "Scheduling qualification evidence: $attempt"
mkdir "$attempt/source"
git -C "$source_checkout" archive "$revision" | tar -x -C "$attempt/source"
git -C "$attempt/source" apply --check "$root/qualification/scheduling/streaming-selection.patch"
git -C "$attempt/source" apply "$root/qualification/scheduling/streaming-selection.patch"

# Cargo owns resolution. Patches apply to the full graph, including transitive
# public Host identities; the consumer's manifests and unrelated pins stay intact.
perl -MJSON::PP=encode_json -e '
    my ($root, $destination) = @ARGV;
    open my $out, ">", $destination or die $!;
    print {$out} "[patch.crates-io]\n";
    for my $name (qw(ic-build ic-host-artifacts ic-host-fs ic-host-process)) {
        print {$out} "$name = { path = ", encode_json("$root/crates/$name"), " }\n";
    }
' "$root" "$attempt/patches.toml"
manifest="$attempt/source/qualification/Cargo.toml"
export CARGO_TARGET_DIR="$attempt/target"
export CARGO_NET_OFFLINE=true
export RUSTUP_AUTO_INSTALL=0
# Keep the exact source lock for comparison; Cargo only resolves the local patches.
cp "$attempt/source/qualification/Cargo.lock" "$attempt/original-Cargo.lock"
cargo metadata --manifest-path "$manifest" --config "$attempt/patches.toml" \
    --format-version 1 --offline > "$attempt/metadata.json"
perl -MJSON::PP=decode_json -e '
    my ($root, $metadata) = @ARGV;
    open my $in, "<", $metadata or die $!;
    my $data = decode_json(do { local $/; <$in> });
    for my $name (qw(ic-build ic-host-artifacts ic-host-fs ic-host-process)) {
        my @found = grep { $_->{name} eq $name } @{$data->{packages}};
        @found == 1 && !defined($found[0]{source}) &&
            $found[0]{manifest_path} eq "$root/crates/$name/Cargo.toml"
            or die "replacement supplier identity mismatch: $name\n";
    }
' "$root" "$attempt/metadata.json"
printf 'Scheduling source: %s\nReplacement checkout: %s\n' "$revision" "$root" > "$attempt/source.txt"

# Build compiles the actual migrated run() callback. Selected library tests cover
# source drift, process refusal and publication without launching PocketIC.
cargo test --manifest-path "$manifest" --config "$attempt/patches.toml" \
    -p ic-timers-pocketic --lib build::tests --locked --offline \
    2>&1 | tee "$attempt/build-tests.log"
cargo test --manifest-path "$manifest" --config "$attempt/patches.toml" \
    -p ic-timers-pocketic --lib report::tests --locked --offline \
    2>&1 | tee "$attempt/report-tests.log"
echo 'Scheduling host consumer checks passed; installed simulator/archive qualification remains separate.'
