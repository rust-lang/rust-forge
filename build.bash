#!/usr/bin/env bash
set -euo pipefail

TARGET=${CARGO_TARGET_DIR:-target}
# https://stackoverflow.com/a/3572105
realpath() {
    [[ $1 = /* ]] && echo "$1" || echo "$PWD/${1#./}"
}

cd blacksmith
cargo build
BLACKSMITH="$(realpath "$TARGET/debug/mdbook-blacksmith")"
cd "$OLDPWD"
"$BLACKSMITH" "$@"
