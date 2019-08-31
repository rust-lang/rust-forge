#!/usr/bin/env bash
(cd blacksmith && cargo build --release)

./blacksmith/target/release/mdbook-blacksmith "$@"
