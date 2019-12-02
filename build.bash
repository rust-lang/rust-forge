#!/usr/bin/env bash
set -e
set -u
set -o pipefail

CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-"$PWD"/target}
export CARGO_TARGET_DIR

cargo run --locked --manifest-path=blacksmith/Cargo.toml -- "$@"
