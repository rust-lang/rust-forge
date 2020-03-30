#!/usr/bin/env bash
set -euo pipefail

CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-"$PWD"/target}
export CARGO_TARGET_DIR

cargo run --locked --manifest-path=blacksmith/Cargo.toml -- "$@"
