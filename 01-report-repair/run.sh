#!/usr/bin/env bash

# compile
cargo build 2>/dev/null
# run
./target/debug/report-repair
# clean
cargo clean 2>/dev/null
