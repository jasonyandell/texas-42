#!/bin/sh -e
# Regenerate pkg/walt2.wasm from source and verify it against the native
# trace. Requires: rustup target add wasm32-unknown-unknown; Node >= 23.6.
cd "$(dirname "$0")"
cargo build --release -p walt2-wasm --target wasm32-unknown-unknown
cp ../target/wasm32-unknown-unknown/release/walt2_wasm.wasm pkg/walt2.wasm
node smoke.mjs
ls -l pkg/walt2.wasm
