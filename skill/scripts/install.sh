#!/usr/bin/env bash
# Installs sysml-check onto $PATH via `cargo install`, so the sysml-lint
# skill works from any directory / any copy of this repo, on any OS or
# Linux distro with a Rust toolchain (Fedora, Debian, Arch, macOS, ...).
# Run this once per machine; skill/scripts/check.sh then picks it up
# automatically (it prefers a PATH-installed binary over building locally).
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
    echo "error: could not locate the sysml-check source tree above $(dirname "${BASH_SOURCE[0]}")" >&2
    echo "       this script must run from within a copy of the rust-sysmlv2-linter repo" >&2
    return 1
}

if ! command -v cargo >/dev/null 2>&1; then
    echo "error: cargo not found. Install a Rust toolchain first — see skill/references/install.md" >&2
    exit 2
fi

REPO_ROOT="$(resolve_repo_root)"
echo "Installing sysml-check from $REPO_ROOT ..." >&2
cargo install --path "$REPO_ROOT" --force

echo >&2
echo "Done. Verifying:" >&2
if command -v sysml-check >/dev/null 2>&1; then
    sysml-check --version
else
    echo "warning: install succeeded but 'sysml-check' isn't on \$PATH yet." >&2
    echo "         cargo installs to the bin dir it prints above (usually ~/.cargo/bin) — add that to \$PATH." >&2
fi
