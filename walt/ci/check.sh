#!/usr/bin/env bash
# walt CI gate: fmt, clippy denials, no-float grep, tests.
# Mirrors rob/ci/check.sh; walt grows its own receipt diffs when it has
# receipts to diff.
set -euo pipefail
cd "$(dirname "$0")/.."

export PATH="$HOME/.cargo/bin:$PATH"

echo "== cargo fmt --check"
cargo fmt --check

echo "== cargo clippy (deny warnings, deny float arithmetic)"
cargo clippy --workspace --all-targets -- -D warnings -D clippy::float_arithmetic

echo "== no-float grep"
# Exact integers and rationals only, everywhere: no float type may even be
# named. This script names them, so it excludes itself.
if grep -rnE '\bf32\b|\bf64\b' . \
    --include='*.rs' --include='*.toml' \
    --exclude-dir=target; then
    echo "ERROR: floating-point type mention found"
    exit 1
fi

echo "== cargo test --workspace --release"
cargo test --workspace --release

echo "walt ci/check.sh: PASS"
