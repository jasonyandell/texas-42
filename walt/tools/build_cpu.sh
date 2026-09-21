#!/usr/bin/env bash
# Build the production native experiment entry points with the validated CPU bundle.
set -euo pipefail
walt_dir="$(cd "$(dirname "$0")/.." && pwd)"
export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C target-cpu=native"
export CARGO_PROFILE_RELEASE_LTO=thin
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
cargo build --locked --offline --release --manifest-path "$walt_dir/Cargo.toml" \
  -p walt-player --bins
cargo build --locked --offline --release --manifest-path "$walt_dir/Cargo.toml" \
  -p walt --bin partnership --bin partner_rollout --bin partnership_gym
cargo build --locked --offline --release --manifest-path "$walt_dir/Cargo.toml" \
  -p walt-cpu-bench "$@"
