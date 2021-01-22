#!/usr/bin/env bash

# compile
cargo build 2>/dev/null
# run
./target/debug/password-philosophy
# clean
cargo clean 2>/dev/null
