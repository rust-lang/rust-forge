#!/usr/bin/env bash
set -euo pipefail

if [ "${RUN_BLACKSMITH:-0}" = "1" ]; then
  cargo run --locked --manifest-path=blacksmith/Cargo.toml -- "$@"
else
  # This actually tells mdbook that we don't support the given renderer
  exit 1
fi
