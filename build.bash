#!/usr/bin/env bash
(cd blacksmith && cargo build)

./blacksmith/target/debug/mdbook-blacksmith "$@"
