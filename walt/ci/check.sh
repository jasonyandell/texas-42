#!/bin/bash -p
# Portable Walt CI gate. Metal evidence is never skipped into green here; the
# elevated conjunction lives in ci/check_m2_metal.sh.
set -euo pipefail

clean_environment_is_exact() {
    local entry name value
    local saw_home=0 saw_path=0 saw_locale=0 saw_tmp=0 saw_marker=0

    while IFS= read -r -d '' entry; do
        name="${entry%%=*}"
        value="${entry#*=}"
        case "$name" in
            HOME) saw_home=1 ;;
            PATH)
                [[ "$value" == "/usr/bin:/bin:/usr/sbin:/sbin" ]] || return 1
                saw_path=1
                ;;
            LC_ALL)
                [[ "$value" == C ]] || return 1
                saw_locale=1
                ;;
            TMPDIR)
                [[ "$value" == /tmp ]] || return 1
                saw_tmp=1
                ;;
            WALT_M2_CLEAN_ENV)
                [[ "$value" == 1 ]] || return 1
                saw_marker=1
                ;;
            PWD|_) ;;
            SHLVL) [[ "$value" =~ ^[0-9]+$ ]] || return 1 ;;
            *) return 1 ;;
        esac
    done < <(/usr/bin/env -0)
    [[ "$saw_home" -eq 1 && "$saw_path" -eq 1 && "$saw_locale" -eq 1 && \
       "$saw_tmp" -eq 1 && "$saw_marker" -eq 1 ]]
}

if ! clean_environment_is_exact; then
    bootstrap_home="${HOME-}"
    bootstrap_dir="$(CDPATH= builtin cd -- \
        "$(/usr/bin/dirname -- "${BASH_SOURCE[0]}")" && /bin/pwd -P)" || exit 1
    exec /usr/bin/env -i \
        HOME="$bootstrap_home" \
        PATH=/usr/bin:/bin:/usr/sbin:/sbin \
        LC_ALL=C \
        TMPDIR=/tmp \
        WALT_M2_CLEAN_ENV=1 \
        /bin/bash -p "$bootstrap_dir/check.sh" "$@"
fi
unset WALT_M2_CLEAN_ENV

PATH="/usr/bin:/bin:/usr/sbin:/sbin"
export PATH LC_ALL TMPDIR

script_dir="$(CDPATH= builtin cd -- "$(/usr/bin/dirname -- "${BASH_SOURCE[0]}")" && /bin/pwd -P)"
walt_dir="$(CDPATH= builtin cd -- "$script_dir/.." && /bin/pwd -P)"
repo_dir="$(CDPATH= builtin cd -- "$walt_dir/.." && /bin/pwd -P)"
CDPATH= builtin cd -- "$walt_dir"

[[ "$#" -le 1 ]] || {
    echo "check.sh: ERROR: usage: check.sh [FAILURE_OUTPUT]" >&2
    exit 1
}
failure_output="${1-}"
failure_phase=3
failure_code=3
receipt_tmp=""
cargo_build_tmp=""
check_succeeded=0

tool_home="${HOME-}"
cargo_bin="$tool_home/.cargo/bin/cargo"
rustc_bin="$tool_home/.cargo/bin/rustc"
rust_proxy_dir="$tool_home/.cargo/bin"
lake_bin="$tool_home/.elan/bin/lake"
elan_bin_dir="$tool_home/.elan/bin"
rust_target="aarch64-apple-darwin"
cargo_target_dir=""

fail() {
    echo "check.sh: ERROR: $*" >&2
    exit 1
}

environment_has() {
    local name="$1"
    [[ ${!name+x} ]]
}

reject_cargo_configs() {
    local directory="$walt_dir"
    local config_path

    while :; do
        for config_path in \
            "$directory/.cargo/config" \
            "$directory/.cargo/config.toml"
        do
            [[ ! -e "$config_path" && ! -L "$config_path" ]] ||
                fail "ambient Cargo config is forbidden: $config_path"
        done
        [[ "$directory" == "/" ]] && break
        directory="$(/usr/bin/dirname "$directory")"
    done
    for config_path in \
        "$tool_home/.cargo/config" \
        "$tool_home/.cargo/config.toml"
    do
        [[ ! -e "$config_path" && ! -L "$config_path" ]] ||
            fail "fixed Cargo home config is forbidden: $config_path"
    done
}

reject_variable_prefix() {
    local prefix="$1"
    local name

    while IFS= read -r name; do
        [[ -z "$name" ]] || fail "ambient $name must be absent"
    done < <(compgen -A variable "$prefix")
}

cleanup_check() {
    local status=$?
    local cleanup_status=0
    local render_status
    trap - EXIT
    set +e
    if [[ "$status" -eq 0 && "$check_succeeded" -ne 1 ]]; then
        status=1
        failure_phase=3
        failure_code=3
        /usr/bin/printf '%s\n' \
            "check.sh: ERROR: exited before the final conjunction" >&2
    fi
    if [[ -n "$receipt_tmp" ]]; then
        /bin/rm -rf -- "$receipt_tmp" || cleanup_status=1
    fi
    if [[ -n "$cargo_build_tmp" ]]; then
        /bin/rm -rf -- "$cargo_build_tmp" || cleanup_status=1
    fi
    if [[ "$cleanup_status" -ne 0 ]]; then
        status=1
        failure_phase=3
        failure_code=3
        /usr/bin/printf '%s\n' \
            "check.sh: ERROR: could not remove all portable gate artifacts" >&2
    fi
    if [[ "$status" -eq 0 ]]; then
        /usr/bin/printf '%s\n' "walt ci/check.sh: PASS"
        exit 0
    fi
    if [[ "$status" -ne 0 && -n "$failure_output" ]]; then
        if [[ -f "$walt_dir/ci/render_m2_failure.py" && \
           ! -L "$walt_dir/ci/render_m2_failure.py" && \
           ! -e "$failure_output" && ! -L "$failure_output" ]]; then
            /usr/bin/env -i \
                HOME="$tool_home" \
                PATH="/usr/bin:/bin" \
                LC_ALL=C \
                TMPDIR=/tmp \
                /usr/bin/python3 -I -B "$walt_dir/ci/render_m2_failure.py" \
                "$repo_dir" "$failure_output" "$failure_phase" "$failure_code"
            render_status=$?
            if [[ "$render_status" -eq 0 ]]; then
                /usr/bin/printf '%s\n' \
                    "check.sh: retained typed failure: $failure_output" >&2
            else
                /usr/bin/printf '%s\n' \
                    "check.sh: ERROR: typed failure renderer exited $render_status" >&2
            fi
        else
            /usr/bin/printf '%s\n' \
                "check.sh: ERROR: typed failure output is unavailable or already occupied" >&2
        fi
    fi
    exit "$status"
}

trap cleanup_check EXIT

[[ "$tool_home" == /* && -d "$tool_home" && ! -L "$tool_home" && \
   "$(CDPATH= builtin cd -- "$tool_home" && /bin/pwd -P)" == "$tool_home" ]] ||
    fail "HOME must be one physical absolute tool-owner directory"
[[ -d "$tool_home/.cargo" && ! -L "$tool_home/.cargo" && \
   -d "$rust_proxy_dir" && ! -L "$rust_proxy_dir" ]] ||
    fail "fixed Rust proxy directory is unavailable or a symlink: $rust_proxy_dir"
[[ -x "$cargo_bin" && -L "$cargo_bin" && \
   "$(/usr/bin/readlink "$cargo_bin")" == "rustup" ]] ||
    fail "fixed Cargo proxy is not the adjacent rustup"
[[ -x "$rustc_bin" && -L "$rustc_bin" && \
   "$(/usr/bin/readlink "$rustc_bin")" == "rustup" ]] ||
    fail "fixed rustc proxy is not the adjacent rustup"
for rust_proxy_name in cargo-clippy cargo-fmt clippy-driver rustdoc rustfmt; do
    rust_proxy="$rust_proxy_dir/$rust_proxy_name"
    [[ -x "$rust_proxy" && -L "$rust_proxy" && \
       "$(/usr/bin/readlink "$rust_proxy")" == "rustup" ]] ||
        fail "fixed $rust_proxy_name proxy is not the adjacent rustup"
done
[[ -x "$rust_proxy_dir/rustup" && ! -L "$rust_proxy_dir/rustup" ]] ||
    fail "fixed rustup executable is unavailable or a symlink"
[[ -d "$tool_home/.elan" && ! -L "$tool_home/.elan" && \
   -d "$elan_bin_dir" && ! -L "$elan_bin_dir" && \
   -x "$lake_bin" && ! -L "$lake_bin" ]] ||
    fail "fixed Elan/Lake executable path is unavailable or a symlink: $lake_bin"

for ambient_name in \
    RUSTFLAGS RUSTDOCFLAGS CARGO_ENCODED_RUSTFLAGS \
    RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER MACOSX_DEPLOYMENT_TARGET \
    SDKROOT DEVELOPER_DIR TOOLCHAINS CARGO_TARGET_DIR CARGO_HOME \
    RUSTUP_HOME RUSTUP_TOOLCHAIN RUSTC RUSTDOC RUSTC_BOOTSTRAP \
    ELAN_HOME ELAN_TOOLCHAIN LEAN_PATH LEAN_SRC_PATH LEAN_SYSROOT LAKE_HOME \
    CARGO_INCREMENTAL \
    CARGO_BUILD_RUSTC CARGO_BUILD_RUSTC_WRAPPER CARGO_BUILD_RUSTDOC \
    CARGO_BUILD_RUSTFLAGS CARGO_BUILD_RUSTDOCFLAGS CARGO_BUILD_TARGET \
    CARGO_BUILD_TARGET_DIR CARGO_TARGET_AARCH64_APPLE_DARWIN_RUSTFLAGS \
    CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER \
    GIT_DIR GIT_WORK_TREE GIT_COMMON_DIR GIT_INDEX_FILE \
    GIT_OBJECT_DIRECTORY GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_QUARANTINE_PATH \
    GIT_REPLACE_REF_BASE GIT_CONFIG_SYSTEM GIT_CONFIG_GLOBAL \
    GIT_CONFIG_NOSYSTEM GIT_CONFIG_COUNT GIT_CONFIG_PARAMETERS
do
    if environment_has "$ambient_name"; then
        fail "ambient $ambient_name must be absent"
    fi
done
reject_variable_prefix CARGO_BUILD_
reject_variable_prefix CARGO_PROFILE_
reject_variable_prefix CARGO_SOURCE_
reject_variable_prefix CARGO_TARGET_
reject_cargo_configs

PATH="/usr/bin:/bin:/usr/sbin:/sbin:$rust_proxy_dir:$elan_bin_dir"
export PATH

cargo_build_tmp="$(/usr/bin/mktemp -d /tmp/walt-portable-cargo.XXXXXX)" ||
    fail "cannot allocate fresh portable Cargo build directory"
[[ -d "$cargo_build_tmp" && ! -L "$cargo_build_tmp" ]] ||
    fail "fresh portable Cargo build path is not a real directory"
cargo_target_dir="$cargo_build_tmp/target"

echo "== immutable M0/M1 history at its producing commit"
failure_phase=1
failure_code=1
/bin/bash -p ci/verify_m2_history.sh

echo "== frozen GPU-native trick-1 guide identity"
failure_phase=1
failure_code=2
/usr/bin/shasum -a 256 -c math/gpu_native_trick1_implementers_guide_v0.2.sha256

echo "== cumulative M0/M1/M2 source identity"
failure_phase=2
failure_code=2
/bin/bash -p ci/verify_m2_sources.sh

echo "== deterministic M0/M1 compatibility replay"
failure_phase=1
failure_code=23
receipt_tmp="$(/usr/bin/mktemp -d /tmp/walt-m0-m1-receipts.XXXXXX)"
"$cargo_bin" --locked run --quiet --release --target "$rust_target" -p walt-gpu-ref \
    --target-dir "$cargo_target_dir" \
    --example generate_m0_m1_receipts -- "$receipt_tmp"
/usr/bin/diff -r receipts/gpu_native_trick1_m0_m1_v1 "$receipt_tmp"

echo "== cargo fmt --check"
failure_phase=3
failure_code=24
"$cargo_bin" --locked fmt --all --check

echo "== cargo clippy (deny warnings, deny float arithmetic)"
"$cargo_bin" --locked clippy --workspace --all-targets --target "$rust_target" \
    --target-dir "$cargo_target_dir" -- \
    -D warnings -D clippy::float_arithmetic

echo "== no-float gates"
# Guarantees, without pretending grep is a Rust lexer:
# - clippy above rejects floating-point arithmetic in every compiled target;
# - this token scan rejects explicit Rust f32/f64 types and suffixes throughout
#   Walt plus the complete Rob source set bound as the independent oracle;
# - the MSL scan rejects the scalar, vector, matrix, packed, and simdgroup
#   half/float/double/bfloat type families;
# - a small fail-closed lexical scanner additionally rejects ordinary inferred
#   float literals in the M0/M1/M2 proof source after removing Rust comments,
#   strings, raw strings, byte strings, and character literals;
# - the AWK gate rejects bare decimal/exponent/inf/nan tokens on every stripped
#   line of the exact M0/M1 TOML/JSON manifests, including continuation lines
#   in arrays and inline tables. It conservatively rejects numeric dotted keys
#   and fractional datetimes too; multiline/unterminated strings fail closed.
if /usr/bin/grep -rnE '(^|[^[:alnum:]_])(f32|f64)([^[:alnum:]_]|$)|[0-9][0-9_]*(f32|f64)([^[:alnum:]_]|$)' \
    . ../rob/crates/core ../rob/crates/player ../rob/crates/verify \
    --include='*.rs' --include='*.toml' \
    --exclude-dir=.gate0-diagnostic; then
    echo "ERROR: explicit Rust floating-point type or suffix found"
    exit 1
fi
if /usr/bin/grep -rnE '(^|[^[:alnum:]_])((packed|simdgroup)_)?(half|float|double|bfloat)([234]|[234]x[234]|8x8)?([^[:alnum:]_]|$)' . \
    --include='*.metal' --exclude-dir=.gate0-diagnostic; then
    echo "ERROR: explicit MSL floating-point type found"
    exit 1
fi
/usr/bin/awk -f ci/check_msl_no_float.awk \
    walt-metal/shaders/00_u256.metal \
    walt-metal/shaders/01_opening_projector.metal
/usr/bin/python3 -I -B ci/check_rust_no_float.py \
    walt-core walt-factory walt-geom walt-gpu-ref walt-gpu-spec \
    walt-kernel walt-m2-runner walt-metal walt-skeleton walt-strat \
    ../rob/crates/core ../rob/crates/player ../rob/crates/verify
/usr/bin/awk -f ci/check_toml_no_float.awk \
    Cargo.toml Cargo.lock rust-toolchain.toml \
    walt-core/Cargo.toml walt-kernel/Cargo.toml \
    walt-gpu-spec/Cargo.toml walt-gpu-ref/Cargo.toml \
    walt-metal/Cargo.toml walt-m2-runner/Cargo.toml \
    ../rob/Cargo.toml ../rob/Cargo.lock \
    ../rob/crates/core/Cargo.toml ../rob/crates/player/Cargo.toml \
    ../rob/crates/verify/Cargo.toml \
    ../lean/lakefile.toml ../lean/lake-manifest.json

echo "== cargo test --workspace --release"
"$cargo_bin" --locked test --workspace --release --target "$rust_target" \
    --target-dir "$cargo_target_dir"

echo "== Lean trick-1 foundations and exact axiom audit"
failure_phase=1
failure_code=1
(
    CDPATH= builtin cd -- ../lean
    "$lake_bin" build Texas42.Trick1Foundation Texas42.Trick1MetalFoundation
    axiom_output="$(/usr/bin/mktemp /tmp/walt-m2-axioms.XXXXXX)"
    trap '/bin/rm -f "$axiom_output"' EXIT
    "$lake_bin" env lean Texas42/Trick1MetalFoundation.lean > "$axiom_output"
    /usr/bin/diff -u trick1_metal_foundation_axioms_v1.txt "$axiom_output"
)

echo "== final cumulative source recheck"
failure_phase=2
failure_code=2
/bin/bash -p ci/verify_m2_sources.sh
check_succeeded=1
