#!/bin/bash -p
# Verify the cumulative freeze-56 source closure.  The manifest itself is
# deliberately excluded: its exact bytes are M2BuildIdentityV1.
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
        /bin/bash -p "$bootstrap_dir/verify_m2_sources.sh" "$@"
fi
unset WALT_M2_CLEAN_ENV

PATH=/usr/bin:/bin:/usr/sbin:/sbin
export PATH LC_ALL TMPDIR

script_dir="$(CDPATH= builtin cd -- "$(/usr/bin/dirname -- "${BASH_SOURCE[0]}")" && /bin/pwd -P)"
walt_dir="$(CDPATH= builtin cd -- "$script_dir/.." && /bin/pwd -P)"
repo_dir="$(CDPATH= builtin cd -- "$walt_dir/.." && /bin/pwd -P)"
manifest_relative="walt/math/gpu_native_trick1_m0_m2_sources_v1.sha256"
manifest="$repo_dir/$manifest_relative"
old_manifest="$walt_dir/math/gpu_native_trick1_m0_m1_sources_v1.sha256"

package_roots=(
    walt/walt-core
    walt/walt-factory
    walt/walt-geom
    walt/walt-gpu-ref
    walt/walt-gpu-spec
    walt/walt-kernel
    walt/walt-m2-runner
    walt/walt-metal
    walt/walt-skeleton
    walt/walt-strat
    rob/crates/core
    rob/crates/player
    rob/crates/verify
)
generated_target_roots=(walt/target rob/target)
for package_root in "${package_roots[@]}"; do
    generated_target_roots+=("$package_root/target")
done

fail() {
    echo "verify_m2_sources.sh: ERROR: $*" >&2
    exit 1
}

require_regular_relative() {
    local relative_path="$1"
    local current="$repo_dir"
    local component
    local -a components
    local index

    IFS=/ read -r -a components <<< "$relative_path"
    [[ "${#components[@]}" -gt 0 ]] ||
        fail "empty repository-relative path"
    for ((index = 0; index < ${#components[@]}; index++)); do
        component="${components[$index]}"
        [[ -n "$component" && "$component" != "." && "$component" != ".." ]] ||
            fail "unnormalized repository-relative path '$relative_path'"
        current="$current/$component"
        [[ ! -L "$current" ]] ||
            fail "symlink is forbidden in source path: $relative_path"
        if ((index + 1 == ${#components[@]})); then
            [[ -f "$current" ]] ||
                fail "source path is not a regular file: $relative_path"
        else
            [[ -d "$current" ]] ||
                fail "source path has a non-directory component: $relative_path"
        fi
    done
}

require_directory_relative() {
    local relative_path="$1"
    local current="$repo_dir"
    local component
    local -a components
    local index

    IFS=/ read -r -a components <<< "$relative_path"
    [[ "${#components[@]}" -gt 0 ]] ||
        fail "empty repository-relative directory"
    for ((index = 0; index < ${#components[@]}; index++)); do
        component="${components[$index]}"
        [[ -n "$component" && "$component" != "." && "$component" != ".." ]] ||
            fail "unnormalized repository-relative directory '$relative_path'"
        current="$current/$component"
        [[ ! -L "$current" && -d "$current" ]] ||
            fail "source-tree component is not a real directory: $relative_path"
    done
}

require_regular_relative "$manifest_relative"
require_regular_relative "walt/math/gpu_native_trick1_m0_m1_sources_v1.sha256"
/usr/bin/grep -n $'\r' "$manifest" >/dev/null 2>&1 &&
    fail "source manifest contains CR bytes"
/usr/bin/iconv -f UTF-8 -t UTF-8 "$manifest" >/dev/null ||
    fail "source manifest is not valid UTF-8"

temporary_dir="$(/usr/bin/mktemp -d /tmp/walt-m2-sources.XXXXXX)"
trap '/bin/rm -rf -- "$temporary_dir"' EXIT
paths_file="$temporary_dir/paths"
: > "$paths_file"

manifest_contains() {
    /usr/bin/grep -Fqx -- "$1" "$paths_file"
}

is_generated_target_path() {
    local relative_path="$1"
    local target_root

    for target_root in "${generated_target_roots[@]}"; do
        case "$relative_path" in
            "$target_root"|"$target_root"/*) return 0 ;;
        esac
    done
    return 1
}

is_generated_target_path "walt/walt-metal/target/generated.o" ||
    fail "generated-target classifier rejected an exact package target root"
if is_generated_target_path "walt/walt-metal/src/target/hidden.rs"; then
    fail "generated-target classifier would omit a nested source target directory"
fi

previous=""
entry_count=0
while IFS= read -r line || [[ -n "$line" ]]; do
    [[ ${#line} -ge 67 ]] || fail "short or empty manifest line"
    expected="${line:0:64}"
    separator="${line:64:2}"
    relative_path="${line:66}"
    [[ "$expected" =~ ^[0-9a-f]{64}$ ]] ||
        fail "invalid digest on manifest line $((entry_count + 1))"
    [[ "$separator" == "  " ]] ||
        fail "manifest line $((entry_count + 1)) does not use two spaces"
    [[ -n "$relative_path" && "$relative_path" != /* ]] ||
        fail "invalid absolute or empty path '$relative_path'"
    case "/$relative_path/" in
        *//*|*/./*|*/../*) fail "unnormalized path '$relative_path'" ;;
    esac
    is_generated_target_path "$relative_path" &&
        fail "generated target path appears in source manifest: $relative_path"
    case "$relative_path" in
        "$manifest_relative"|*/.gate0-diagnostic/*|*.air|*/__pycache__/*|*/module-cache/*)
            fail "excluded path appears in source manifest: $relative_path"
            ;;
        walt/receipts/gpu_native_trick1_m2_v1/*)
            fail "M2 generated receipt/checksum appears in source manifest: $relative_path"
            ;;
    esac
    if [[ -n "$previous" ]]; then
        [[ "$previous" < "$relative_path" ]] ||
            fail "paths are duplicate or not in strict bytewise order: '$previous' then '$relative_path'"
    fi
    require_regular_relative "$relative_path"
    file="$repo_dir/$relative_path"
    actual="$(/usr/bin/shasum -a 256 "$file" | /usr/bin/awk '{print $1}')"
    [[ "$actual" == "$expected" ]] ||
        fail "$relative_path: expected $expected, found $actual"
    printf '%s\n' "$relative_path" >> "$paths_file"
    previous="$relative_path"
    entry_count=$((entry_count + 1))
done < "$manifest"
[[ "$entry_count" -gt 184 ]] ||
    fail "cumulative source closure has only $entry_count entries"

# Every path in the immutable M0/M1 source closure must remain represented,
# translated from its historical walt-relative grammar to repository-relative.
old_count=0
while IFS= read -r line || [[ -n "$line" ]]; do
    case "$line" in
        ""|\#*) continue ;;
    esac
    old_path="${line#*  }"
    case "$old_path" in
        ../*) current_path="${old_path#../}" ;;
        *) current_path="walt/$old_path" ;;
    esac
    manifest_contains "$current_path" ||
        fail "historical source path omitted from cumulative closure: $current_path"
    old_count=$((old_count + 1))
done < "$old_manifest"
[[ "$old_count" -eq 184 ]] ||
    fail "historical source manifest no longer has 184 entries"

required_paths=(
    lean/Texas42.lean
    lean/Texas42/Trick1MetalFoundation.lean
    lean/trick1_metal_foundation_axioms_v1.txt
    walt/CENSUS-RULINGS.md
    walt/Cargo.lock
    walt/Cargo.toml
    walt/GPU-NATIVE-TRICK1-M2.md
    walt/GPU-NATIVE-TRICK1.md
    walt/ci/check.sh
    walt/ci/check_msl_no_float.awk
    walt/ci/check_m2_metal.sh
    walt/ci/check_rust_no_float.py
    walt/ci/render_m2_failure.py
    walt/ci/verify_m2_history.sh
    walt/ci/verify_m2_sources.sh
    walt/math/gpu_native_trick1_implementers_guide_v0.2.md
    walt/math/gpu_native_trick1_m0_m1_sources_v1.sha256
    walt/math/gpu_native_trick1_m2_rebrief_v0.1.md
    walt/receipts/gpu_native_trick1_gate0_2026-08-16.txt
    walt/receipts/gpu_native_trick1_m0_m1_v1/grade5_declared_stop_v1.bin
    walt/receipts/gpu_native_trick1_m0_m1_v1/opening_max_cell_envelope_v1.bin
    walt/receipts/gpu_native_trick1_m0_m1_v1/receipt_summary_v1.txt
    walt/rust-toolchain.toml
    walt/walt-gpu-ref/Cargo.toml
    walt/walt-gpu-ref/src/lib.rs
    walt/walt-gpu-ref/src/m2.rs
    walt/walt-gpu-ref/src/m2_receipt/mod.rs
    walt/walt-gpu-ref/src/m2_receipt/receipt.rs
    walt/walt-gpu-ref/src/m2_receipt/records.rs
    walt/walt-gpu-ref/src/m2_receipt/transport.rs
    walt/walt-gpu-ref/src/m2_receipt/wire.rs
    walt/walt-gpu-ref/tests/m2_receipt.rs
    walt/walt-metal/Cargo.toml
    walt/walt-metal/shaders/00_u256.metal
    walt/walt-metal/shaders/01_opening_projector.metal
    walt/walt-metal/shaders/build_metallib.sh
    walt/walt-metal/shaders/walt_m2.metallib
    walt/walt-metal/toolchain/m2_host_tool_descriptor_v1.txt
    walt/walt-m2-runner/Cargo.toml
    walt/walt-m2-runner/src/assembly.rs
    walt/walt-m2-runner/src/child.rs
    walt/walt-m2-runner/src/lib.rs
    walt/walt-m2-runner/src/main.rs
    walt/walt-m2-runner/src/observation.rs
    walt/walt-m2-runner/src/protocol.rs
)
for required_path in "${required_paths[@]}"; do
    manifest_contains "$required_path" ||
        fail "required M2 path omitted: $required_path"
done

# Close every current Cargo package plus the complete Lean and CI trees.  Every
# regular package file is conservatively treated as a build input; only each
# package's exact root-level generated `target` directory is pruned.  A nested
# source directory merely named `target` remains closed by the manifest.  This
# deliberately
# over-includes documentation, fixtures and custom targets rather than allowing
# an unmanifested file to become reachable through include/build/test machinery.
closure_file_list="$temporary_dir/closure-files"
: > "$closure_file_list"

register_closure_tree() {
    local relative_root="$1"
    local absolute_root="$repo_dir/$relative_root"
    local bad_nodes="$temporary_dir/bad-nodes"

    require_directory_relative "$relative_root"
    : > "$bad_nodes"
    /usr/bin/find "$absolute_root" \
        \( -type l -o \( ! -type f ! -type d \) \) -print0 > "$bad_nodes"
    [[ ! -s "$bad_nodes" ]] ||
        fail "symlink or nonregular node exists in build-input tree: $relative_root"
    /usr/bin/find "$absolute_root" -type f -print0 >> "$closure_file_list"
}

register_package_tree() {
    local relative_root="$1"
    local absolute_root="$repo_dir/$relative_root"
    local bad_nodes="$temporary_dir/bad-package-nodes"
    local generated_target="$absolute_root/target"

    require_directory_relative "$relative_root"
    if [[ -e "$generated_target" || -L "$generated_target" ]]; then
        [[ -d "$generated_target" && ! -L "$generated_target" ]] ||
            fail "generated target root is not a real directory: $relative_root/target"
    fi
    : > "$bad_nodes"
    /usr/bin/find "$absolute_root" \
        -path "$absolute_root/target" -prune -o \
        \( -type l -o \( ! -type f ! -type d \) \) -print0 > "$bad_nodes"
    [[ ! -s "$bad_nodes" ]] ||
        fail "symlink or nonregular node exists in local package: $relative_root"
    /usr/bin/find "$absolute_root" \
        -path "$absolute_root/target" -prune -o \
        -type f -print0 >> "$closure_file_list"
}

register_closure_tree "walt/ci"
register_closure_tree "lean/Texas42"

for package_root in "${package_roots[@]}"; do
    nested_lock="$repo_dir/$package_root/Cargo.lock"
    [[ ! -e "$nested_lock" && ! -L "$nested_lock" ]] ||
        fail "nested generated lockfile must not survive workspace integration: $nested_lock"
    register_package_tree "$package_root"
done

build_input_files=(
    lean/Texas42.lean
    lean/lake-manifest.json
    lean/lakefile.toml
    lean/lean-toolchain
    lean/trick1_metal_foundation_axioms_v1.txt
    rob/Cargo.lock
    rob/Cargo.toml
    walt/Cargo.lock
    walt/Cargo.toml
    walt/rust-toolchain.toml
)
for build_input_file in "${build_input_files[@]}"; do
    require_regular_relative "$build_input_file"
    printf '%s\0' "$repo_dir/$build_input_file" >> "$closure_file_list"
done

while IFS= read -r -d '' source_file; do
    relative_path="${source_file#"$repo_dir/"}"
    [[ "$relative_path" != *$'\n'* && "$relative_path" != *$'\r'* ]] ||
        fail "newline is forbidden in build-input path"
    manifest_contains "$relative_path" ||
        fail "build-input source omitted from M2 closure: $relative_path"
done < "$closure_file_list"

build_identity="$(/usr/bin/shasum -a 256 "$manifest" | /usr/bin/awk '{print $1}')"
echo "verify_m2_sources.sh: PASS ($entry_count files; M2BuildIdentityV1=$build_identity)"
