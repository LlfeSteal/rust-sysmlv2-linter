#!/usr/bin/env bash
# Portable wrapper around the sysml-check binary for the sysml-lint skill.
# Resolves which binary to run in order, so this works regardless of
# environment or where any particular copy of this repo/skill lives:
#   1. $SYSML_CHECK_BIN, if set          — explicit override
#   2. `sysml-check` on $PATH            — proper install (see install.sh /
#                                           references/install.md), works
#                                           from anywhere, any OS/distro
#   3. this repo's target/release build  — zero-setup fallback, only works
#                                           if the source tree is still
#                                           reachable above this script
set -euo pipefail

resolve_repo_root() {
    local dir
    dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    while [ "$dir" != "/" ]; do
        if [ -f "$dir/Cargo.toml" ] && grep -q '^name = "sysml-check"' "$dir/Cargo.toml"; then
            printf '%s\n' "$dir"
            return 0
        fi
        dir="$(dirname "$dir")"
    done
    return 1
}

BIN=""

if [ -n "${SYSML_CHECK_BIN:-}" ]; then
    if [ -x "$SYSML_CHECK_BIN" ]; then
        BIN="$SYSML_CHECK_BIN"
    else
        echo "warning: \$SYSML_CHECK_BIN is set to '$SYSML_CHECK_BIN' but it's not an executable file, ignoring it" >&2
    fi
fi

if [ -z "$BIN" ] && command -v sysml-check >/dev/null 2>&1; then
    BIN="$(command -v sysml-check)"
fi

if [ -z "$BIN" ]; then
    if REPO_ROOT="$(resolve_repo_root)"; then
        CANDIDATE="$REPO_ROOT/target/release/sysml-check"
        if [ ! -x "$CANDIDATE" ]; then
            if command -v cargo >/dev/null 2>&1; then
                echo "sysml-check: not on PATH, building from source at $REPO_ROOT (cargo build --release)..." >&2
                (cd "$REPO_ROOT" && cargo build --release --quiet)
            else
                echo "error: sysml-check is not installed, not on PATH, and cargo isn't available to build it from $REPO_ROOT" >&2
                echo "       see skill/references/install.md for how to install it" >&2
                exit 2
            fi
        fi
        BIN="$CANDIDATE"
    fi
fi

if [ -z "$BIN" ]; then
    echo "error: could not find or build sysml-check:" >&2
    echo "       - \$SYSML_CHECK_BIN is not set" >&2
    echo "       - 'sysml-check' is not on \$PATH" >&2
    echo "       - no sysml-check source tree found above this script" >&2
    echo "       run skill/scripts/install.sh (or see skill/references/install.md) to install it" >&2
    exit 2
fi

exec "$BIN" --format json "$@"
