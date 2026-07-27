#!/usr/bin/env bash
# rob CI gate (BRIEF §9): fmt, clippy denials, no-float grep, tests, and
# byte-for-byte receipt diffs (INV-5).
set -euo pipefail
cd "$(dirname "$0")/.."

export PATH="$HOME/.cargo/bin:$PATH"

echo "== cargo fmt --check"
cargo fmt --check

echo "== cargo clippy (deny warnings, deny float arithmetic)"
cargo clippy --workspace --all-targets -- -D warnings -D clippy::float_arithmetic

echo "== no-float grep (INV-4)"
if grep -rnE '\bf32\b|\bf64\b' crates --include='*.rs'; then
    echo "ERROR: floating-point type mention found (INV-4)"
    exit 1
fi

echo "== vocabulary grep (INV-10)"
if grep -rnE 'OuterCertificate|ReachabilityCertificate|OuterReachabilityCertificate' crates --include='*.rs'; then
    echo "ERROR: forbidden certificate-style identifier (INV-10: necessary outer profile, never certificate)"
    exit 1
fi

echo "== cargo test --workspace --release"
cargo test --workspace --release

echo "== receipt diffs"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT
for expected in receipts/verify_*.txt; do
    stage="$(basename "$expected" .txt)"
    fresh="$tmpdir/$stage.txt"
    cargo run --quiet --release --bin "$stage" > "$fresh"
    diff -u "$expected" "$fresh"
    echo "receipt $stage: byte-identical"
done

echo "rob ci/check.sh: PASS"
