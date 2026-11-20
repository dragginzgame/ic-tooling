#!/bin/sh
# Report the actual Cargo child environment and each argument without using a registry.
printf '%s\n' "${CARGO_NET_OFFLINE-unset}" "$@" >&2
exit 7
