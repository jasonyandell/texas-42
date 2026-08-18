#!/bin/bash -p
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
            WALT_M3_CLEAN_ENV)
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
        WALT_M3_CLEAN_ENV=1 \
        /bin/bash -p "$bootstrap_dir/build_m3_metallib.sh" "$@"
fi
unset WALT_M3_CLEAN_ENV
PATH=/usr/bin:/bin:/usr/sbin:/sbin
export PATH LC_ALL TMPDIR

toolchain="com.apple.dt.toolchain.Metal.32023.883"
script_dir="$(CDPATH= builtin cd -- "$(/usr/bin/dirname -- "${BASH_SOURCE[0]}")" && /bin/pwd -P)"
temporary_root=$(/usr/bin/mktemp -d /tmp/walt-m3-metallib.XXXXXX)
trap '/bin/rm -rf -- "$temporary_root"' EXIT HUP INT TERM

mode=${1:-verify}
case "$mode" in
    verify|--replace) ;;
    *)
        echo "usage: $0 [verify|--replace]" >&2
        exit 2
        ;;
esac

build_one() {
    run_name=$1
    source_dir="$temporary_root/$run_name/source"
    air_dir="$temporary_root/$run_name/air"
    output="$temporary_root/$run_name/walt_m3.metallib"
    /bin/mkdir -p "$source_dir" "$air_dir"
    /bin/cp "$script_dir/00_u256.metal" "$source_dir/00_u256.metal"
    /bin/cp "$script_dir/02_m3_wavefront.metal" "$source_dir/02_m3_wavefront.metal"

    /usr/bin/xcrun --toolchain "$toolchain" metal \
        -std=metal3.2 \
        -mmacosx-version-min=26.0 \
        -fmetal-math-mode=safe \
        -fno-fast-math \
        -Wall \
        -Wextra \
        -Werror \
        -c "$source_dir/00_u256.metal" \
        -o "$air_dir/00_u256.air"
    /usr/bin/xcrun --toolchain "$toolchain" metal \
        -std=metal3.2 \
        -mmacosx-version-min=26.0 \
        -fmetal-math-mode=safe \
        -fno-fast-math \
        -Wall \
        -Wextra \
        -Werror \
        -c "$source_dir/02_m3_wavefront.metal" \
        -o "$air_dir/02_m3_wavefront.air"
    /usr/bin/xcrun --toolchain "$toolchain" metallib \
        "$air_dir/00_u256.air" \
        "$air_dir/02_m3_wavefront.air" \
        -o "$output"
}

build_one first
build_one second

/usr/bin/cmp "$temporary_root/first/walt_m3.metallib" \
    "$temporary_root/second/walt_m3.metallib"
if [ "$mode" = "--replace" ]; then
    /bin/cp "$temporary_root/first/walt_m3.metallib" "$script_dir/walt_m3.metallib"
else
    /usr/bin/cmp "$temporary_root/first/walt_m3.metallib" \
        "$script_dir/walt_m3.metallib"
fi
/usr/bin/shasum -a 256 "$script_dir/walt_m3.metallib"
