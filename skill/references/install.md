# Installing sysml-check

`skill/scripts/check.sh` (and thus this skill) works best when
`sysml-check` is installed on `$PATH`, instead of relying on a specific
copy of this repo being present relative to the skill. Installing it is
a one-time, per-machine step — after that, the skill works from any
directory, any copy of the skill, regardless of whether the original
repo checkout still exists.

`sysml-check` is a zero-dependency pure-Rust crate (no C libraries, no
OS-specific bindings), so this works identically on any OS or Linux
distro with a Rust toolchain — Fedora, Debian/Ubuntu, Arch, macOS,
Windows (native or WSL), etc.

## 1. Get a Rust toolchain (skip if `cargo --version` already works)

- **Any OS, recommended**: [rustup](https://rustup.rs) —
  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Fedora / RHEL / CentOS**: `sudo dnf install cargo rust`
- **Debian / Ubuntu**: `sudo apt install cargo rustc`
- **Arch**: `sudo pacman -S rust`
- **macOS (Homebrew)**: `brew install rust`

Any of these is sufficient — the crate needs nothing beyond the
standard library (`Cargo.toml` declares no dependencies).

## 2. Install sysml-check

From a copy of this repo:

```sh
skill/scripts/install.sh
```

...which is equivalent to running, from the repo root:

```sh
cargo install --path . --force
```

Either way, this builds a release binary and copies it into cargo's
install-bin directory — `~/.cargo/bin` for a standard rustup install,
already on `$PATH` by default. Distro-package installs of cargo may use
a different location; `cargo install` prints the exact path it used.

## 3. Verify

```sh
sysml-check --version
```

If that prints a version, `skill/scripts/check.sh` will now find and use
this installed binary automatically (it checks `$PATH` before falling
back to building from a local repo checkout). If `sysml-check: command
not found`, cargo's bin directory isn't on `$PATH` — add it (for rustup:
`export PATH="$HOME/.cargo/bin:$PATH"` in your shell profile) and retry.

## Alternative: point at a binary directly

If you already have a `sysml-check` binary somewhere non-standard (a
prebuilt artifact, a different install location), skip installation
entirely and set:

```sh
export SYSML_CHECK_BIN=/path/to/sysml-check
```

`check.sh` uses this in preference to everything else.
