#!/usr/bin/env bash
# Real provider qualification, never operator credentials. Linux uses a new local
# bus and temporary Secret Service; macOS uses unique disposable keychain entries.
set -euo pipefail
umask 077
repo=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd -P)
cd "$repo"
: "${SIGNER_TEST_ICP:?select an absolute prepared ICP CLI 1.7.0 path}"
manifest=${SIGNER_TEST_MANIFEST:-$repo/Cargo.toml}
[[ "$manifest" == /* && -f "$manifest" && ! -L "$manifest" ]] || exit 2

if [[ ${1:-} == --linux-session ]]; then
    evidence=$2
    test_binary=$4
    [[ "$evidence" == "$repo"/target/sign-keyring.* && -d "$evidence" && ! -L "$evidence" ]] || exit 2
    locked_test=linux::native_locked_keyring_refuses_without_changing_identity
    case $3 in
        unlocked) selection=(--skip "$locked_test") ;;
        locked) selection=(--exact "$locked_test") ;;
        *) exit 2 ;;
    esac
    evidence="$evidence/$3"
    mkdir "$evidence"
    export XDG_DATA_HOME="$evidence/private-data"
    export XDG_CONFIG_HOME="$evidence/private-config"
    unset GNOME_KEYRING_CONTROL
    unset DISPLAY WAYLAND_DISPLAY XAUTHORITY
    mkdir -p "$XDG_DATA_HOME" "$XDG_CONFIG_HOME" "$evidence/control"
    daemon_pid=
    monitor_pid=
    cleanup() {
        if [[ -n $monitor_pid ]]; then
            kill "$monitor_pid" 2>/dev/null || true
            wait "$monitor_pid" 2>/dev/null || true
        fi
        if [[ -n $daemon_pid ]]; then
            # This fixture deliberately stops its daemon. SIGKILL also settles
            # cleanup if the test is interrupted before its resume guard runs.
            kill -KILL "$daemon_pid" 2>/dev/null || true
            wait "$daemon_pid" 2>/dev/null || true
        fi
        rm -rf -- "$evidence/private-data" "$evidence/private-config" "$evidence/control"
    }
    trap cleanup EXIT
    # Known non-operator fixture password, delivered only over stdin, selects an
    # encrypted temporary collection rather than the unprotected empty-password format.
    printf 'disposable-keyring-fixture-only\n' | gnome-keyring-daemon --foreground --unlock --components=secrets \
        --control-directory "$evidence/control" > "$evidence/keyring.log" 2>&1 &
    daemon_pid=$!
    export SIGNER_TEST_PRIVATE_SESSION=1 SIGNER_TEST_KEYRING_PID="$daemon_pid"
    for ((attempt=0; attempt<100; attempt++)); do
        if gdbus call --session --dest org.freedesktop.DBus \
            --object-path /org/freedesktop/DBus \
            --method org.freedesktop.DBus.NameHasOwner org.freedesktop.secrets 2>/dev/null | grep -q true; then break; fi
        sleep 0.05
    done
    # Profile mode retains message headers/timing only, never message bodies.
    (ulimit -S -f 8192
     exec dbus-monitor --session --profile "interface='org.freedesktop.Secret.Prompt'") \
        > "$evidence/prompt-profile.log" 2> "$evidence/prompt-monitor.log" &
    monitor_pid=$!
    # Only launch/socket metadata is traced: never secret-bearing write/read or
    # message buffers, environment dumps, private files or request payloads.
    (ulimit -S -f 8192
     timeout --kill-after=5s 60s strace -f -e trace=socket,connect,execve -o "$evidence/network.trace" \
         bash -c 'ulimit -S -f unlimited; exec "$@"' sign-fixture \
         "$test_binary" --ignored --test-threads=1 --show-output "${selection[@]}") > "$evidence/test.log" 2>&1
    if [[ $3 == locked ]]; then
        grep -Fxq "test $locked_test ... ok" "$evidence/test.log"
    else
        grep -Fxq 'test native_icp_keyring_update ... ok' "$evidence/test.log"
        grep -Fxq 'test linux::native_keyring_cancellation_retains_custody ... ok' "$evidence/test.log"
    fi
    kill "$monitor_pid"
    wait "$monitor_pid" || true
    monitor_pid=
    if grep -Fq 'org.freedesktop.Secret.Prompt' "$evidence/prompt-profile.log"; then
        echo 'Noninteractive provider emitted an OS prompt request' >&2
        exit 1
    fi
    # Empty/missing tracing cannot establish an offline result.
    grep -Eq 'execve\("[^"]*/ic-sign"' "$evidence/network.trace"
    grep -Eq 'connect\(.*AF_UNIX' "$evidence/network.trace"
    if grep 'socket(' "$evidence/network.trace" | grep -v 'AF_UNIX'; then
        echo 'Unexpected nonlocal socket family during real-provider qualification' >&2
        exit 1
    fi
    perl -e 'open my $f, "<:raw", $ARGV[0] or die "missing disposable keyring\n";
        my $header = ""; my $count = read($f, $header, 16);
        defined($count) && $count == 16 && $header eq "GnomeKeyring\n\r\0\n"
            or die "disposable keyring did not retain the encrypted binary format\n";' \
        "$XDG_DATA_HOME/keyrings/login.keyring"
    echo "Encrypted ICP 1.7.0 keyring tests passed in the $3 session; no prompts or external sockets."
    exit
fi

mkdir -p "$repo/target"
evidence=$(mktemp -d "$repo/target/sign-keyring.XXXXXXXX")
printf 'Evidence: %s\n' "$evidence"
uname -srm > "$evidence/host.txt"
"$SIGNER_TEST_ICP" --version > "$evidence/tool-versions.txt"
if [[ $(uname -s) == Linux ]]; then
    gnome-keyring-daemon --version >> "$evidence/tool-versions.txt"
    dbus-daemon --version >> "$evidence/tool-versions.txt"
else
    /usr/bin/sw_vers >> "$evidence/host.txt"
fi
checkpoint() {
    find crates/ic-host-sign crates/ic-host-process -type f -exec shasum -a 256 {} +
    shasum -a 256 Cargo.toml Cargo.lock scripts/qualification/sign-keyring.sh "$SIGNER_TEST_ICP"
    shasum -a 256 "$manifest" "${manifest%/*}/Cargo.lock"
    if [[ -n ${SIGNER_TEST_HELPER:-} ]]; then shasum -a 256 "$SIGNER_TEST_HELPER"; fi
}
checkpoint | LC_ALL=C sort > "$evidence/source.sha256"
git rev-parse HEAD > "$evidence/source-head"
# Compile before tracing or denying sockets so Cargo can follow caller policy.
cargo test --manifest-path "$manifest" -p ic-host-sign --features provider --test keyring --no-run --locked \
    --message-format=json-render-diagnostics > "$evidence/build.jsonl" 2> "$evidence/build.log"
test_binary=$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "keyring"
    and .target.kind == ["test"] and .profile.test) | .executable] | select(length == 1) | .[0]' "$evidence/build.jsonl")
[[ "$test_binary" == /* && -x "$test_binary" ]]
case $(uname -s) in
    Linux)
        # Locking affects the whole collection. Destroy it before starting the other cases.
        for state in locked unlocked; do
            dbus-run-session -- bash "$repo/scripts/qualification/sign-keyring.sh" --linux-session "$evidence" "$state" "$test_binary"
        done
        ;;
    Darwin)
        [[ ${SIGNER_TEST_DISPOSABLE_KEYCHAIN:-} == 1 ]] || {
            echo 'Native macOS qualification requires an explicitly disposable account (SIGNER_TEST_DISPOSABLE_KEYCHAIN=1).' >&2
            exit 2
        }
        SIGNER_TEST_NETWORK_SANDBOX=1 /usr/bin/sandbox-exec \
            -p '(version 1)(allow default)(deny network*)' \
            "$test_binary" --ignored --test-threads=1 --show-output > "$evidence/test.log" 2>&1
        echo 'Native keychain update/status passed with inherited network denial.'
        ;;
    *) echo 'Only Linux and macOS credential backends are supported' >&2; exit 2 ;;
esac
checkpoint | LC_ALL=C sort > "$evidence/source-after.sha256"
cmp "$evidence/source.sha256" "$evidence/source-after.sha256"
