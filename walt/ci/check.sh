#!/usr/bin/env bash
# walt CI gate: fmt, clippy denials, no-float grep, tests.
# Mirrors rob/ci/check.sh; walt grows its own receipt diffs when it has
# receipts to diff.
set -euo pipefail
cd "$(dirname "$0")/.."

export PATH="$HOME/.cargo/bin:$PATH"

echo "== frozen GPU-native trick-1 source identity"
shasum -a 256 -c math/gpu_native_trick1_implementers_guide_v0.2.sha256
shasum -a 256 -c math/gpu_native_trick1_m0_m1_sources_v1.sha256

echo "== deterministic M0/M1 persisted receipts"
receipt_tmp="$(mktemp -d "${TMPDIR:-/tmp}/walt-m0-m1-receipts.XXXXXX")"
trap 'rm -rf "$receipt_tmp"' EXIT
cargo run --quiet --release -p walt-gpu-ref --example generate_m0_m1_receipts -- "$receipt_tmp"
diff -r receipts/gpu_native_trick1_m0_m1_v1 "$receipt_tmp"

echo "== cargo fmt --check"
cargo fmt --check

echo "== cargo clippy (deny warnings, deny float arithmetic)"
cargo clippy --workspace --all-targets -- -D warnings -D clippy::float_arithmetic

echo "== no-float gates"
# Guarantees, without pretending grep is a Rust lexer:
# - clippy above rejects floating-point arithmetic in every compiled target;
# - this token scan rejects explicit Rust f32/f64 types and suffixes throughout
#   Walt plus the complete Rob source set bound as the independent oracle;
# - the MSL scan rejects the scalar, vector, matrix, packed, and simdgroup
#   half/float/double/bfloat type families;
# - a numeric-token scan additionally rejects ordinary inferred float literals
#   in the M0/M1 production source and generator. It is intentionally scoped:
#   the older workspace contains prose strings such as section 12.6, so a
#   lexer-free whole-workspace decimal grep would be a false claim, not a gate.
# - the AWK gate rejects bare decimal/exponent/inf/nan tokens on every stripped
#   line of the exact M0/M1 TOML/JSON manifests, including continuation lines
#   in arrays and inline tables. It conservatively rejects numeric dotted keys
#   and fractional datetimes too; multiline/unterminated strings fail closed.
if grep -rnE '(^|[^[:alnum:]_])(f32|f64)([^[:alnum:]_]|$)|[0-9][0-9_]*(f32|f64)\b' \
    . ../rob/crates/core ../rob/crates/player ../rob/crates/verify \
    --include='*.rs' --include='*.toml' --exclude-dir=target; then
    echo "ERROR: explicit Rust floating-point type or suffix found"
    exit 1
fi
if grep -rnE '(^|[^[:alnum:]_])((packed|simdgroup)_)?(half|float|double|bfloat)([234]|[234]x[234]|8x8)?([^[:alnum:]_]|$)' . \
    --include='*.metal' --exclude-dir=target; then
    echo "ERROR: explicit MSL floating-point type found"
    exit 1
fi
if grep -rnE '(^|[^[:alnum:]_.])([0-9][0-9_]*\.[0-9_]+([eE][+-]?[0-9_]+)?|[0-9][0-9_]*\.([^[:digit:]_.]|$)|[0-9][0-9_]*[eE][+-]?[0-9_]+)(f32|f64)?([^[:alnum:]_]|$)' \
    walt-gpu-spec/src walt-gpu-ref/src walt-gpu-ref/examples \
    --include='*.rs' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//'; then
    echo "ERROR: inferred floating-point literal found in M0/M1 production source"
    exit 1
fi
awk -f ci/check_toml_no_float.awk \
    Cargo.toml Cargo.lock rust-toolchain.toml \
    walt-core/Cargo.toml walt-kernel/Cargo.toml \
    walt-gpu-spec/Cargo.toml walt-gpu-ref/Cargo.toml \
    ../rob/Cargo.toml ../rob/Cargo.lock \
    ../rob/crates/core/Cargo.toml ../rob/crates/player/Cargo.toml \
    ../rob/crates/verify/Cargo.toml \
    ../lean/lakefile.toml ../lean/lake-manifest.json

echo "== cargo test --workspace --release"
cargo test --workspace --release

echo "== Lean trick-1 foundation"
(
    cd ../lean
    lake build Texas42.Trick1Foundation
)

echo "walt ci/check.sh: PASS"
