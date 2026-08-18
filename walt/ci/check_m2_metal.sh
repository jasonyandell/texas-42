#!/bin/bash -p
# Elevated freeze-56 conjunction.  Success exists only after the discarded
# smoke, two fresh complete official children, and exact receipt regeneration.
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
    bootstrap_script_dir="$(CDPATH= builtin cd -- \
        "$(/usr/bin/dirname -- "${BASH_SOURCE[0]}")" && /bin/pwd -P)" || exit 1
    exec /usr/bin/env -i \
        HOME="$bootstrap_home" \
        PATH=/usr/bin:/bin:/usr/sbin:/sbin \
        LC_ALL=C \
        TMPDIR=/tmp \
        WALT_M2_CLEAN_ENV=1 \
        /bin/bash -p "$bootstrap_script_dir/check_m2_metal.sh" "$@"
fi
unset WALT_M2_CLEAN_ENV

PATH="/usr/bin:/bin:/usr/sbin:/sbin"
export PATH LC_ALL TMPDIR

script_dir="$(CDPATH= builtin cd -- "$(/usr/bin/dirname -- "${BASH_SOURCE[0]}")" && /bin/pwd -P)"
walt_dir="$(CDPATH= builtin cd -- "$script_dir/.." && /bin/pwd -P)"
repo_dir="$(CDPATH= builtin cd -- "$walt_dir/.." && /bin/pwd -P)"
tool_home="${HOME-}"
cargo_bin="$tool_home/.cargo/bin/cargo"
rustc_bin="$tool_home/.cargo/bin/rustc"
rust_proxy_dir="$tool_home/.cargo/bin"
lake_bin="$tool_home/.elan/bin/lake"
elan_bin_dir="$tool_home/.elan/bin"
rust_target="aarch64-apple-darwin"
receipt_dir="$walt_dir/receipts/gpu_native_trick1_m2_v1"
committed_receipt="$receipt_dir/m2_metal_parity_v1.bin"
committed_checksum="$receipt_dir/m2_metal_parity_v1.sha256"
receipt_relative="walt/receipts/gpu_native_trick1_m2_v1/m2_metal_parity_v1.bin"
checksum_relative="walt/receipts/gpu_native_trick1_m2_v1/m2_metal_parity_v1.sha256"
receipt_basename="m2_metal_parity_v1.bin"
diagnostic_root="$walt_dir/.gate0-diagnostic"
temporary_dir=""
bootstrap_temporary_dir=""
gate_succeeded=0
head_commit=""
failure_phase=3
failure_code=3
typed_failure_path=""
failure_repository_root="$repo_dir"
fallback_script="$walt_dir/ci/render_m2_failure.py"

fail() {
    echo "check_m2_metal.sh: ERROR: $*" >&2
    exit 1
}

cleanup_gate_artifacts() {
    local status=$?
    local cleanup_status
    local fallback_status
    trap - EXIT
    set +e
    if [[ "$status" -eq 0 && "$gate_succeeded" -ne 1 ]]; then
        /usr/bin/printf '%s\n' \
            "check_m2_metal.sh: ERROR: exited before the final conjunction" >&2
        status=1
        failure_phase=16
        failure_code=3
    fi
    if [[ "$status" -eq 0 && "$gate_succeeded" -eq 1 ]]; then
        /bin/rm -rf -- "$temporary_dir"
        cleanup_status=$?
        if [[ "$cleanup_status" -eq 0 ]]; then
            /bin/rm -rf -- "$bootstrap_temporary_dir"
            cleanup_status=$?
        fi
        if [[ "$cleanup_status" -eq 0 ]]; then
            /usr/bin/printf '%s\n' "walt ci/check_m2_metal.sh: PASS"
            exit 0
        else
            /usr/bin/printf '%s\n' \
                "check_m2_metal.sh: ERROR: could not remove all successful gate artifacts" >&2
            status=1
            failure_phase=16
            failure_code=3
        fi
    fi
    if [[ -z "$typed_failure_path" ]]; then
        if [[ -f "$fallback_script" && ! -L "$fallback_script" && \
           ! -e "$fallback_failure" && ! -L "$fallback_failure" ]]; then
            /usr/bin/env -i \
                HOME="$tool_home" \
                PATH="/usr/bin:/bin" \
                LC_ALL=C \
                TMPDIR=/tmp \
                /usr/bin/python3 -I -B "$fallback_script" \
                "$failure_repository_root" "$fallback_failure" \
                "$failure_phase" "$failure_code"
            fallback_status=$?
            if [[ "$fallback_status" -eq 0 ]]; then
                typed_failure_path="$fallback_failure"
            else
                /usr/bin/printf '%s\n' \
                    "check_m2_metal.sh: ERROR: typed fallback renderer failed with status $fallback_status" >&2
            fi
        else
            /usr/bin/printf '%s\n' \
                "check_m2_metal.sh: ERROR: typed fallback path is unavailable or already occupied" >&2
        fi
    fi
    if [[ -n "$typed_failure_path" ]]; then
        /usr/bin/printf '%s\n' \
            "check_m2_metal.sh: retained typed failure: $typed_failure_path" >&2
    fi
    if [[ -n "$temporary_dir" ]]; then
        /usr/bin/printf '%s\n' \
            "check_m2_metal.sh: retained failure artifacts: $temporary_dir" >&2
    fi
    exit "$status"
}

environment_has() {
    local name="$1"
    [[ ${!name+x} ]]
}

reject_cargo_configs() {
    local directory="$1"
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

bootstrap_temporary_dir="$(/usr/bin/mktemp -d /tmp/walt-m2-gate-bootstrap.XXXXXX)" || {
    /usr/bin/printf '%s\n' \
        "check_m2_metal.sh: ERROR: cannot establish typed-failure bootstrap directory" >&2
    exit 1
}
fallback_failure="$bootstrap_temporary_dir/outer-gate-failure.bin"
trap cleanup_gate_artifacts EXIT
[[ -d "$bootstrap_temporary_dir" && ! -L "$bootstrap_temporary_dir" ]] ||
    fail "typed-failure bootstrap path is not a real directory"

if [[ -e "$diagnostic_root" || -L "$diagnostic_root" ]]; then
    [[ -d "$diagnostic_root" && ! -L "$diagnostic_root" ]] ||
        fail "diagnostic root is not a real directory: $diagnostic_root"
else
    /bin/mkdir -m 700 "$diagnostic_root" ||
        fail "cannot create diagnostic root: $diagnostic_root"
fi
temporary_dir="$(/usr/bin/mktemp -d "$diagnostic_root/m2-gate.XXXXXX")" ||
    fail "cannot allocate a unique gate diagnostic directory"
[[ -d "$temporary_dir" && ! -L "$temporary_dir" ]] ||
    fail "unique gate diagnostic path is not a real directory"

cargo_target_dir="$temporary_dir/target"
runner="$cargo_target_dir/$rust_target/release/walt-m2-runner"
git_home="$temporary_dir/git-home"
source_archive="$temporary_dir/source.tar"
source_snapshot="$temporary_dir/source"
snapshot_walt_dir="$source_snapshot/walt"
snapshot_receipt="$temporary_dir/head-receipt.bin"
snapshot_checksum="$temporary_dir/head-receipt.sha256"
portable_failure="$temporary_dir/portable-gate-failure.bin"
adjudication_failure="$temporary_dir/receipt-adjudication-failure.bin"
[[ ! -e "$cargo_target_dir" && ! -L "$cargo_target_dir" && \
   ! -e "$git_home" && ! -L "$git_home" && \
   ! -e "$source_archive" && ! -L "$source_archive" && \
   ! -e "$source_snapshot" && ! -L "$source_snapshot" && \
   ! -e "$snapshot_receipt" && ! -L "$snapshot_receipt" && \
   ! -e "$snapshot_checksum" && ! -L "$snapshot_checksum" && \
   ! -e "$portable_failure" && ! -L "$portable_failure" && \
   ! -e "$adjudication_failure" && ! -L "$adjudication_failure" && \
   ! -e "$fallback_failure" && ! -L "$fallback_failure" ]] ||
    fail "fresh gate paths were not absent"

[[ "$tool_home" == /* && -d "$tool_home" && ! -L "$tool_home" && \
   "$(CDPATH= builtin cd -- "$tool_home" && /bin/pwd -P)" == "$tool_home" ]] ||
    fail "HOME must be one physical absolute tool-owner directory"
[[ -d "$tool_home/.cargo" && ! -L "$tool_home/.cargo" && \
   -d "$rust_proxy_dir" && ! -L "$rust_proxy_dir" ]] ||
    fail "fixed Rust proxy directory is unavailable or a symlink: $rust_proxy_dir"
[[ -x "$cargo_bin" && -L "$cargo_bin" && \
   "$(/usr/bin/readlink "$cargo_bin")" == "rustup" ]] ||
    fail "fixed Cargo proxy no longer resolves to the adjacent rustup"
[[ -x "$rustc_bin" && -L "$rustc_bin" && \
   "$(/usr/bin/readlink "$rustc_bin")" == "rustup" ]] ||
    fail "fixed rustc proxy no longer resolves to the adjacent rustup"
for rust_proxy_name in cargo-clippy cargo-fmt clippy-driver rustdoc rustfmt; do
    rust_proxy="$rust_proxy_dir/$rust_proxy_name"
    [[ -x "$rust_proxy" && -L "$rust_proxy" && \
       "$(/usr/bin/readlink "$rust_proxy")" == "rustup" ]] ||
        fail "fixed $rust_proxy_name proxy no longer resolves to the adjacent rustup"
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
reject_cargo_configs "$walt_dir"

PATH="/usr/bin:/bin:/usr/sbin:/sbin:$rust_proxy_dir:$elan_bin_dir"
export PATH

/bin/mkdir -m 700 "$git_home" ||
    fail "cannot create isolated Git configuration home"
[[ -d "$git_home" && ! -L "$git_home" ]] ||
    fail "isolated Git configuration home is not a real directory"

gate_git() {
    /usr/bin/env -i \
        HOME="$git_home" \
        XDG_CONFIG_HOME="$git_home" \
        PATH="/usr/bin:/bin" \
        LC_ALL=C \
        TMPDIR=/tmp \
        GIT_CONFIG_NOSYSTEM=1 \
        /usr/bin/git --no-replace-objects \
        -c core.fsmonitor=false \
        -c core.untrackedCache=false \
        -c color.ui=false \
        "$@"
}

create_immutable_source_snapshot() {
    local repository_top
    local archive_commit

    failure_phase=2
    failure_code=1
    repository_top="$(gate_git -C "$repo_dir" rev-parse --show-toplevel)"
    [[ "$repository_top" == "$repo_dir" ]] ||
        fail "repository root mismatch: $repository_top"
    [[ -z "$head_commit" ]] || fail "HEAD snapshot was already resolved"
    head_commit="$(gate_git -C "$repo_dir" rev-parse --verify 'HEAD^{commit}')"
    [[ "$head_commit" =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ ]] ||
        fail "HEAD did not resolve to one exact lowercase commit object"

    /bin/mkdir -m 700 "$source_snapshot" ||
        fail "cannot create immutable source snapshot directory"
    gate_git -C "$repo_dir" archive --format=tar \
        --output="$source_archive" "$head_commit" ||
        fail "cannot archive immutable HEAD source"
    [[ -f "$source_archive" && ! -L "$source_archive" ]] ||
        fail "immutable source archive is not a regular file"
    archive_commit="$(gate_git get-tar-commit-id < "$source_archive")"
    [[ "$archive_commit" == "$head_commit" ]] ||
        fail "source archive commit identity mismatch"
    /usr/bin/tar -xpf "$source_archive" -C "$source_snapshot" ||
        fail "cannot extract immutable source archive"
    [[ -d "$snapshot_walt_dir" && ! -L "$snapshot_walt_dir" && \
       -f "$snapshot_walt_dir/ci/verify_m2_sources.sh" && \
       ! -L "$snapshot_walt_dir/ci/verify_m2_sources.sh" ]] ||
        fail "immutable source snapshot has no real Walt source verifier"

    failure_phase=2
    failure_code=2
    failure_repository_root="$source_snapshot"
    /bin/bash -p "$snapshot_walt_dir/ci/verify_m2_sources.sh"
    [[ -f "$snapshot_walt_dir/ci/render_m2_failure.py" && \
       ! -L "$snapshot_walt_dir/ci/render_m2_failure.py" ]] ||
        fail "immutable source snapshot has no regular fallback renderer"
    fallback_script="$snapshot_walt_dir/ci/render_m2_failure.py"
    failure_phase=3
    failure_code=3
    reject_cargo_configs "$snapshot_walt_dir"
}

verify_live_source_matches_snapshot() {
    local live_manifest="$repo_dir/walt/math/gpu_native_trick1_m0_m2_sources_v1.sha256"
    local snapshot_manifest="$source_snapshot/walt/math/gpu_native_trick1_m0_m2_sources_v1.sha256"

    [[ -f "$live_manifest" && ! -L "$live_manifest" && \
       -f "$snapshot_manifest" && ! -L "$snapshot_manifest" ]] ||
        fail "live/snapshot source manifest is not two regular files"
    /usr/bin/cmp "$snapshot_manifest" "$live_manifest" ||
        fail "live source manifest differs from immutable HEAD snapshot"
    /bin/bash -p "$walt_dir/ci/verify_m2_sources.sh"
    /usr/bin/cmp "$snapshot_manifest" "$live_manifest" ||
        fail "live source manifest changed during source verification"
}

require_regular_relative() {
    local relative_path="$1"
    local current="$repo_dir"
    local component
    local -a components
    local index

    IFS=/ read -r -a components <<< "$relative_path"
    for ((index = 0; index < ${#components[@]}; index++)); do
        component="${components[$index]}"
        [[ -n "$component" && "$component" != "." && "$component" != ".." ]] ||
            fail "unnormalized committed path: $relative_path"
        current="$current/$component"
        [[ ! -L "$current" ]] || fail "symlink in committed path: $relative_path"
        if ((index + 1 == ${#components[@]})); then
            [[ -f "$current" ]] || fail "committed path is not a regular file: $relative_path"
        else
            [[ -d "$current" ]] ||
                fail "non-directory component in committed path: $relative_path"
        fi
    done
}

verify_immutable_committed_comparands() {
    local repository_top
    local current_head
    local dirty
    local receipt_entry
    local checksum_entry
    local receipt_metadata
    local checksum_metadata
    local receipt_mode
    local checksum_mode
    local receipt_oid
    local checksum_oid
    local receipt_index
    local checksum_index
    local checksum_bytes
    local expected_checksum_bytes
    local checksum_line
    local checksum_digest
    local computed_digest

    repository_top="$(gate_git -C "$repo_dir" rev-parse --show-toplevel)"
    [[ "$repository_top" == "$repo_dir" ]] ||
        fail "repository root mismatch: $repository_top"
    current_head="$(gate_git -C "$repo_dir" rev-parse --verify 'HEAD^{commit}')"
    [[ -n "$head_commit" && "$current_head" == "$head_commit" ]] ||
        fail "HEAD moved after immutable source snapshot"
    require_regular_relative "$receipt_relative"
    require_regular_relative "$checksum_relative"

    receipt_entry="$(gate_git -C "$repo_dir" \
        ls-tree --full-tree "$head_commit" -- "$receipt_relative")"
    checksum_entry="$(gate_git -C "$repo_dir" \
        ls-tree --full-tree "$head_commit" -- "$checksum_relative")"
    case "$receipt_entry" in
        "100644 blob "*$'\t'"$receipt_relative"|"100755 blob "*$'\t'"$receipt_relative") ;;
        *) fail "committed receipt is not one regular blob at HEAD" ;;
    esac
    case "$checksum_entry" in
        "100644 blob "*$'\t'"$checksum_relative"|"100755 blob "*$'\t'"$checksum_relative") ;;
        *) fail "committed checksum is not one regular blob at HEAD" ;;
    esac
    [[ "$receipt_entry" != *$'\n'* && "$checksum_entry" != *$'\n'* ]] ||
        fail "duplicate committed comparand entry at HEAD"

    receipt_metadata="${receipt_entry%%$'\t'*}"
    checksum_metadata="${checksum_entry%%$'\t'*}"
    receipt_mode="${receipt_metadata%% *}"
    checksum_mode="${checksum_metadata%% *}"
    receipt_oid="${receipt_metadata##* }"
    checksum_oid="${checksum_metadata##* }"
    [[ "$receipt_oid" =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ && \
       "$checksum_oid" =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ ]] ||
        fail "comparand tree entry has a noncanonical blob object id"
    receipt_index="$(gate_git -C "$repo_dir" ls-files --stage -- "$receipt_relative")"
    checksum_index="$(gate_git -C "$repo_dir" ls-files --stage -- "$checksum_relative")"
    [[ "$receipt_index" == "$receipt_mode $receipt_oid 0"$'\t'"$receipt_relative" ]] ||
        fail "receipt index entry differs from immutable HEAD"
    [[ "$checksum_index" == "$checksum_mode $checksum_oid 0"$'\t'"$checksum_relative" ]] ||
        fail "checksum index entry differs from immutable HEAD"
    case "$receipt_mode" in
        100644) [[ ! -x "$committed_receipt" ]] || fail "receipt worktree mode is dirty" ;;
        100755) [[ -x "$committed_receipt" ]] || fail "receipt worktree mode is dirty" ;;
    esac
    case "$checksum_mode" in
        100644) [[ ! -x "$committed_checksum" ]] || fail "checksum worktree mode is dirty" ;;
        100755) [[ -x "$committed_checksum" ]] || fail "checksum worktree mode is dirty" ;;
    esac

    dirty="$(gate_git -C "$repo_dir" status --porcelain=v1 --untracked-files=all -- \
        "$receipt_relative" "$checksum_relative")"
    [[ -z "$dirty" ]] ||
        fail "committed receipt/checksum has an index or worktree change: $dirty"

    [[ ! -e "$snapshot_receipt" && ! -L "$snapshot_receipt" && \
       ! -e "$snapshot_checksum" && ! -L "$snapshot_checksum" ]] ||
        fail "immutable HEAD snapshot output already exists"
    gate_git -C "$repo_dir" \
        cat-file blob "$head_commit:$receipt_relative" > "$snapshot_receipt"
    gate_git -C "$repo_dir" \
        cat-file blob "$head_commit:$checksum_relative" > "$snapshot_checksum"
    [[ -f "$snapshot_receipt" && ! -L "$snapshot_receipt" && \
       -f "$snapshot_checksum" && ! -L "$snapshot_checksum" ]] ||
        fail "immutable HEAD snapshot is not two regular files"
    /usr/bin/cmp "$snapshot_receipt" "$committed_receipt" ||
        fail "current receipt bytes differ from immutable HEAD"
    /usr/bin/cmp "$snapshot_checksum" "$committed_checksum" ||
        fail "current checksum bytes differ from immutable HEAD"

    checksum_bytes="$(/usr/bin/wc -c < "$snapshot_checksum" | /usr/bin/tr -d ' ')"
    expected_checksum_bytes=$((64 + 2 + ${#receipt_basename} + 1))
    [[ "$checksum_bytes" == "$expected_checksum_bytes" ]] ||
        fail "committed checksum has $checksum_bytes bytes, expected $expected_checksum_bytes"
    IFS= read -r checksum_line < "$snapshot_checksum" ||
        fail "committed checksum lacks its one terminating LF"
    [[ "${#checksum_line}" -eq $((64 + 2 + ${#receipt_basename})) ]] ||
        fail "committed checksum line has the wrong length"
    checksum_digest="${checksum_line:0:64}"
    [[ "$checksum_digest" =~ ^[0-9a-f]{64}$ ]] ||
        fail "committed checksum digest is not 64 lowercase hexadecimal bytes"
    [[ "${checksum_line:64:2}" == "  " && \
       "${checksum_line:66}" == "$receipt_basename" ]] ||
        fail "committed checksum must name exactly $receipt_basename"
    computed_digest="$(/usr/bin/shasum -a 256 "$snapshot_receipt" | \
        /usr/bin/awk '{print $1}')"
    [[ "$computed_digest" =~ ^[0-9a-f]{64}$ && \
       "$computed_digest" == "$checksum_digest" ]] ||
        fail "independent committed receipt digest comparison failed"
}

verify_comparand_snapshot_still_current() {
    local current_head
    local dirty

    current_head="$(gate_git -C "$repo_dir" rev-parse --verify 'HEAD^{commit}')"
    [[ "$current_head" == "$head_commit" ]] ||
        fail "HEAD moved during receipt adjudication"
    dirty="$(gate_git -C "$repo_dir" status --porcelain=v1 --untracked-files=all -- \
        "$receipt_relative" "$checksum_relative")"
    [[ -z "$dirty" ]] ||
        fail "receipt/checksum changed during adjudication: $dirty"
}

run_profile_or_retain_failure() {
    local command="$1"
    local output="$2"
    local label="$3"
    local run_status
    local operation_phase="$failure_phase"
    local operation_code="$failure_code"

    failure_phase=3
    failure_code=3
    reject_cargo_configs "$walt_dir"
    failure_phase="$operation_phase"
    failure_code="$operation_code"
    [[ ! -e "$output" && ! -L "$output" ]] ||
        fail "$label output already exists: $output"
    if "$runner" "$command" "$repo_dir" "$output"; then
        return 0
    else
        run_status=$?
    fi
    if "$runner" validate-failure "$output"; then
        typed_failure_path="$output"
        fail "$label failed with status $run_status; typed failure retained at $output"
    fi
    fail "$label failed with status $run_status without a canonical typed failure; diagnostics retained at $temporary_dir"
}

adjudicate_or_retain_failure() {
    local adjudication_status
    local operation_phase="$failure_phase"
    local operation_code="$failure_code"

    failure_phase=3
    failure_code=3
    reject_cargo_configs "$walt_dir"
    failure_phase="$operation_phase"
    failure_code="$operation_code"
    [[ ! -e "$adjudication_failure" && ! -L "$adjudication_failure" ]] ||
        fail "receipt-adjudication failure output already exists"
    if "$runner" adjudicate-receipts \
        "$repo_dir" \
        "$temporary_dir/official-1.bin" \
        "$temporary_dir/official-2.bin" \
        "$snapshot_receipt" \
        "$snapshot_checksum" \
        "$adjudication_failure"
    then
        [[ ! -e "$adjudication_failure" && ! -L "$adjudication_failure" ]] ||
            fail "successful receipt adjudication unexpectedly wrote a failure artifact"
        return 0
    else
        adjudication_status=$?
    fi
    if "$runner" validate-failure "$adjudication_failure"; then
        typed_failure_path="$adjudication_failure"
        fail "receipt adjudication failed with status $adjudication_status; typed failure retained at $adjudication_failure"
    fi
    fail "receipt adjudication failed with status $adjudication_status without a canonical typed failure; diagnostics retained at $temporary_dir"
}

CDPATH= builtin cd -- "$walt_dir"

echo "== portable freeze-56 conjunction"
failure_phase=3
failure_code=24
if /bin/bash -p ci/check.sh "$portable_failure"; then
    [[ ! -e "$portable_failure" && ! -L "$portable_failure" ]] ||
        fail "successful portable gate unexpectedly wrote a failure artifact"
else
    portable_status=$?
    if [[ -f "$portable_failure" && ! -L "$portable_failure" ]]; then
        typed_failure_path="$portable_failure"
    fi
    fail "portable gate failed with status $portable_status"
fi

echo "== immutable committed source snapshot"
create_immutable_source_snapshot

echo "== release M2 runner"
failure_phase=3
failure_code=3
reject_cargo_configs "$snapshot_walt_dir"
failure_code=24
(
    CDPATH= builtin cd -- "$snapshot_walt_dir"
    "$cargo_bin" --locked build --release --target "$rust_target" \
        --target-dir "$cargo_target_dir" -p walt-m2-runner
)

echo "== checked host/tool/device descriptor and two-build metallib"
failure_phase=2
failure_code=2
verify_live_source_matches_snapshot
failure_phase=3
failure_code=3
reject_cargo_configs "$walt_dir"
failure_phase=4
failure_code=3
"$runner" descriptor-verify "$repo_dir"

echo "== canonical Rust Gate 0, U256 parity and Metal negative controls"
failure_phase=3
failure_code=3
reject_cargo_configs "$snapshot_walt_dir"
failure_phase=6
failure_code=24
(
    CDPATH= builtin cd -- "$snapshot_walt_dir"
    "$cargo_bin" --locked test --release --target "$rust_target" \
        --target-dir "$cargo_target_dir" \
        -p walt-metal --test metal_device \
        canonical_arithmetic_and_negative_controls_device_gate -- --ignored --exact
)

echo "== timeout, malformed-protocol and no-partial controls"
failure_phase=3
failure_code=3
reject_cargo_configs "$snapshot_walt_dir"
failure_phase=15
failure_code=21
(
    CDPATH= builtin cd -- "$snapshot_walt_dir"
    "$cargo_bin" --locked test --release --target "$rust_target" \
        --target-dir "$cargo_target_dir" \
        -p walt-m2-runner protocol::tests
)

echo "== discarded maximum-projector smoke child"
failure_phase=10
failure_code=24
run_profile_or_retain_failure run-smoke "$temporary_dir/smoke.bin" \
    "maximum-projector smoke child"
"$runner" validate-smoke "$temporary_dir/smoke.bin"

echo "== fresh official M2 child 1"
failure_phase=12
failure_code=24
run_profile_or_retain_failure run-official "$temporary_dir/official-1.bin" \
    "first official M2 child"
"$runner" validate-receipt "$repo_dir" "$temporary_dir/official-1.bin"

echo "== fresh official M2 child 2"
failure_phase=12
failure_code=24
run_profile_or_retain_failure run-official "$temporary_dir/official-2.bin" \
    "second official M2 child"
"$runner" validate-receipt "$repo_dir" "$temporary_dir/official-2.bin"

echo "== immutable HEAD comparands and typed receipt adjudication"
failure_phase=16
failure_code=23
verify_immutable_committed_comparands
adjudicate_or_retain_failure
verify_comparand_snapshot_still_current
"$runner" validate-receipt "$repo_dir" "$snapshot_receipt"

echo "== final Lean foundation and exact axiom audit"
failure_phase=1
failure_code=1
live_lean_packages="$repo_dir/lean/.lake/packages"
snapshot_lean_lake="$source_snapshot/lean/.lake"
[[ -d "$live_lean_packages" && ! -L "$live_lean_packages" ]] ||
    fail "fixed Lean package cache is unavailable or a symlink"
[[ ! -e "$snapshot_lean_lake" && ! -L "$snapshot_lean_lake" ]] ||
    fail "immutable source snapshot unexpectedly contains a Lean build cache"
/bin/mkdir -m 700 "$snapshot_lean_lake" ||
    fail "cannot allocate fresh snapshot Lean build directory"
/bin/ln -s "$live_lean_packages" "$snapshot_lean_lake/packages" ||
    fail "cannot bind the fixed Lean package cache into the fresh snapshot build"
(
    CDPATH= builtin cd -- "$source_snapshot/lean"
    "$lake_bin" build Texas42.Trick1Foundation Texas42.Trick1MetalFoundation
    axiom_output="$(/usr/bin/mktemp /tmp/walt-m2-final-axioms.XXXXXX)"
    trap '/bin/rm -f "$axiom_output"' EXIT
    "$lake_bin" env lean Texas42/Trick1MetalFoundation.lean > "$axiom_output"
    /usr/bin/diff -u trick1_metal_foundation_axioms_v1.txt "$axiom_output"
)

echo "== final cumulative source identity"
failure_phase=2
failure_code=2
verify_live_source_matches_snapshot
/bin/bash -p "$snapshot_walt_dir/ci/verify_m2_sources.sh"

failure_phase=16
failure_code=23
verify_comparand_snapshot_still_current
gate_succeeded=1
