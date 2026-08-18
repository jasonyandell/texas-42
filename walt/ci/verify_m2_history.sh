#!/bin/bash -p
# Freeze-56 historical verifier: inspect freeze-55 bytes at their producing
# commit, never through current-path reinterpretation.
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
        /bin/bash -p "$bootstrap_dir/verify_m2_history.sh" "$@"
fi
unset WALT_M2_CLEAN_ENV

PATH=/usr/bin:/bin:/usr/sbin:/sbin
export PATH LC_ALL TMPDIR

script_dir="$(CDPATH= builtin cd -- "$(/usr/bin/dirname -- "${BASH_SOURCE[0]}")" && /bin/pwd -P)"
walt_dir="$(CDPATH= builtin cd -- "$script_dir/.." && /bin/pwd -P)"
repo_dir="$(CDPATH= builtin cd -- "$walt_dir/.." && /bin/pwd -P)"
parent_commit="3b4c6d60fef371e3050de151ccf9eaefbc2d2da7"
parent_census_bytes=921481
parent_census_sha256="518ab540358f8d74ea091a5e0d9dd269d6e64ec55a4b54c7b2a10f2d3d203e45"
old_manifest_sha256="eccf0a3742e2cfc50cad158292db7ad8c6145da8aa7958b7aa2ed07a1566f2ad"

fail() {
    echo "verify_m2_history.sh: ERROR: $*" >&2
    exit 1
}

sha256_file() {
    /usr/bin/shasum -a 256 "$1" | /usr/bin/awk '{print $1}'
}

require_file_hash() {
    local relative_path="$1"
    local expected="$2"
    local actual
    actual="$(sha256_file "$walt_dir/$relative_path")"
    [[ "$actual" == "$expected" ]] ||
        fail "$relative_path: expected $expected, found $actual"
}

temporary_dir="$(/usr/bin/mktemp -d /tmp/walt-m2-history.XXXXXX)" ||
    fail "cannot allocate isolated historical-verifier directory"
[[ -d "$temporary_dir" && ! -L "$temporary_dir" ]] ||
    fail "isolated historical-verifier path is not a real directory"
git_home="$temporary_dir/git-home"
/bin/mkdir -m 700 "$git_home" ||
    fail "cannot allocate isolated Git configuration home"
trap '/bin/rm -rf -- "$temporary_dir"' EXIT

history_git() {
    /usr/bin/env -i \
        HOME="$git_home" \
        XDG_CONFIG_HOME="$git_home" \
        PATH=/usr/bin:/bin \
        LC_ALL=C \
        TMPDIR=/tmp \
        GIT_CONFIG_NOSYSTEM=1 \
        GIT_NO_REPLACE_OBJECTS=1 \
        /usr/bin/git --no-replace-objects \
        -c core.fsmonitor=false \
        -c core.untrackedCache=false \
        -c color.ui=false \
        "$@"
}

history_git -C "$repo_dir" cat-file -e "$parent_commit^{commit}" ||
    fail "missing parent commit $parent_commit"

manifest="$walt_dir/math/gpu_native_trick1_m0_m1_sources_v1.sha256"
[[ "$(sha256_file "$manifest")" == "$old_manifest_sha256" ]] ||
    fail "historical M0/M1 manifest bytes changed"

entry_count=0
while IFS= read -r line; do
    case "$line" in
        ""|\#*) continue ;;
    esac
    [[ "$line" == *"  "* ]] || fail "malformed old-manifest line"
    expected="${line%%  *}"
    relative_path="${line#*  }"
    [[ "$expected" =~ ^[0-9a-f]{64}$ ]] || fail "invalid digest for $relative_path"
    [[ -n "$relative_path" && "$relative_path" != /* ]] ||
        fail "invalid historical path $relative_path"
    case "$relative_path" in
        ../*) repository_path="${relative_path#../}" ;;
        *..*) fail "unsupported historical path $relative_path" ;;
        *) repository_path="walt/$relative_path" ;;
    esac
    [[ "$repository_path" != *..* ]] || fail "unnormalized path $repository_path"
    actual="$(
        history_git -C "$repo_dir" show \
            "$parent_commit:$repository_path" |
            /usr/bin/shasum -a 256 |
            /usr/bin/awk '{print $1}'
    )"
    [[ "$actual" == "$expected" ]] ||
        fail "$repository_path at parent: expected $expected, found $actual"
    entry_count=$((entry_count + 1))
done < "$manifest"
[[ "$entry_count" -eq 184 ]] || fail "expected 184 historical entries, found $entry_count"

history_git -C "$repo_dir" show \
    "$parent_commit:walt/CENSUS-RULINGS.md" > "$temporary_dir/parent-census"
[[ "$(/usr/bin/wc -c < "$temporary_dir/parent-census" | /usr/bin/tr -d ' ')" -eq "$parent_census_bytes" ]] ||
    fail "parent CENSUS byte length changed"
[[ "$(sha256_file "$temporary_dir/parent-census")" == "$parent_census_sha256" ]] ||
    fail "parent CENSUS digest changed"
current_prefix_sha256="$(
    /usr/bin/head -c "$parent_census_bytes" "$walt_dir/CENSUS-RULINGS.md" |
        /usr/bin/shasum -a 256 |
        /usr/bin/awk '{print $1}'
)"
[[ "$current_prefix_sha256" == "$parent_census_sha256" ]] ||
    fail "current CENSUS parent prefix changed"

require_file_hash "GPU-NATIVE-TRICK1.md" \
    "6190e740a0579b6b5196e086e52c8022d4cddcd0f746ecbd9226f87bbc0e4790"
require_file_hash "math/gpu_native_trick1_implementers_guide_v0.2.md" \
    "ee2e78da20eb7d087fb121f467a56bafc0179a45fb692ca0b938f4c4210b6a44"
require_file_hash "receipts/gpu_native_trick1_gate0_2026-08-16.txt" \
    "b57f7077e5aa0aa1d8030a76a3399076810b71b1623ad83e001aee2b4aaeb215"
require_file_hash "receipts/gpu_native_trick1_m0_m1_v1/opening_max_cell_envelope_v1.bin" \
    "1127d3868d7da07c26a7b8bc031ac8a63ba84a9068df786b67a413ea6af5f517"
require_file_hash "receipts/gpu_native_trick1_m0_m1_v1/grade5_declared_stop_v1.bin" \
    "7e8dfecf1cac314ae6e71b406eb268b29d4157206ce5e64d1c50d1aa94d43bdf"
require_file_hash "receipts/gpu_native_trick1_m0_m1_v1/receipt_summary_v1.txt" \
    "51a162ea933801f05b852ec2a454c48a31c7d292ee8273ba683d0a7fec340b12"

echo "verify_m2_history.sh: PASS ($entry_count parent blobs; prefix intact)"
