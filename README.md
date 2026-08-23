# sysml-check

**A fast, zero-dependency linter for SysML v2 textual notation.**

`sysml-check` parses and semantically checks `.sysml` files — the
textual notation defined by the OMG SysML v2 specification — and
reports precise, actionable diagnostics: syntax errors, unresolved
names, semantic rule violations, and optional style warnings. It's a
single static binary with no runtime dependencies, built to be equally
useful at a terminal, in CI, and as a tool an AI coding agent can call
directly.

> **Scope note**: this checks the SysML v2 **textual notation**
> (`.sysml` files) defined in the OMG SysML v2 / KerML specifications.
> It does not understand classic/legacy graphical SysML.

## Features

- **Zero dependencies.** `Cargo.toml` has no `[dependencies]` at all —
  lexer, parser, and semantic model are all hand-rolled. Fast, small,
  and nothing to audit downstream.
- **Three output formats**: readable terminal output (`human`),
  machine-readable `json` for tooling/agents, and `gitlab` for GitLab's
  Code Quality report format in CI pipelines.
- **A real diagnostic catalog, not ad-hoc messages.** 39 rules across
  syntax errors (`E0xx`/`E1xx`), semantic errors (`E2xx`), and warnings
  (`W2xx`/`W3xx`, some gated behind `--pedantic`) — enumerable at any
  time with `--list-rules`.
- **AST emission** (`--emit ast`) for tooling that needs the resolved
  symbol tree — names, relationships, and whether each reference
  resolved — not just diagnostics.
- **Built for agents as much as humans**: JSON output is designed to be
  parsed directly, and the project ships an [Agent Skill](skill/) so
  Claude Code, opencode, or any compatible tool can invoke it as part of
  writing or verifying a model.

## Installation

Requires a Rust toolchain (`rust-version = "1.65"` or newer) — nothing
else. Any way of getting `cargo` works identically on any OS or distro,
since the crate has no system dependencies:

```sh
cd rust-sysmlv2-linter   # from a clone/copy of this repository
cargo install --path .
```

This puts `sysml-check` on your `$PATH` (via cargo's install-bin
directory, `~/.cargo/bin` by default). Alternatively, build a binary
without installing it globally:

```sh
cargo build --release
./target/release/sysml-check --version
```

## Quick start

The repo ships a clean example model at `examples/drone.sysml`:

```sh
$ sysml-check examples/drone.sysml
✔ 1 fichier(s) analysé(s) — aucun problème
```

(The CLI's human-facing output is in French; diagnostic codes,
`--format json`, and `--format gitlab` output are all
locale-independent.)

Turn on style rules with `--pedantic`:

```sh
$ sysml-check --pedantic examples/drone.sysml
avertissement[W306]: le nom d'usage `AutonomieMinimale` devrait commencer par une minuscule
   --> examples/drone.sysml:109:29
    |
109 |         satisfy requirement AutonomieMinimale by droneProduction;
    |                             ^^^^^^^^^^^^^^^^^
    = aide: convention SysML v2 : lowerCamelCase pour les usages
    = règle: naming-convention

0 erreur(s), 1 avertissement(s), 0 info(s) dans 1 fichier(s)
```

Get machine-readable output for scripting, CI, or an agent loop:

```sh
$ sysml-check --format json examples/drone.sysml
{
  "tool": "sysml-check",
  "version": "0.1.0",
  "summary": { "files": 1, "errors": 0, "warnings": 0, "infos": 0, "ok": true },
  "diagnostics": []
}
```

Every diagnostic in the `json`/`gitlab` formats carries `file`, `code`,
`rule`, `severity`, `message`, an optional `hint`, and a precise
`line`/`column`/`endLine`/`endColumn` span plus a source `snippet` —
enough to render or auto-fix without re-parsing the file yourself.

See the full rule catalog at any time:

```sh
sysml-check --list-rules
```

## CLI reference

```
USAGE:
    sysml-check [OPTIONS] <FILE>...
    sysml-check [OPTIONS] --stdin

OPTIONS:
    -f, --format <human|json|gitlab>   Output format (default: human)
        --emit <diagnostics|ast|both>  What to emit (default: diagnostics)
        --stdin                        Read the model from stdin
        --name <NAME>                  Filename shown when using --stdin
        --pedantic                     Enable style rules (W302/W306/W307/W309/W311/W312/W313)
        --unresolved <error|warn|off>  Severity for unresolved names (default: error)
        --deny-warnings                Warnings become blocking (exit 1)
        --max-diags <N>                Max diagnostics emitted (default: 500)
        --color                        ANSI color in human output
    -q, --quiet                        Only print the summary line
        --list-rules                   Print the rule catalog (JSON array) then exit
    -h, --help                         Print help
    -V, --version                      Print version
```

Multiple `<FILE>` arguments are checked together as one combined model.

### Exit codes

| Code | Meaning |
|------|---------|
| `0` | No errors — the model is syntactically and semantically consistent |
| `1` | At least one error (or a warning, if `--deny-warnings` was passed) |
| `2` | Usage error or I/O error |

## Diagnostic codes

Codes follow a fixed scheme, always safe to pattern-match on in tooling:

| Prefix | Meaning |
|--------|---------|
| `E0xx` / `E1xx` | Lexer / parser syntax errors |
| `E2xx` | Semantic errors |
| `W2xx` / `W3xx` | Warnings (some require `--pedantic`) |

Run `sysml-check --list-rules` for the live, authoritative list of all
39 codes with their descriptions — that catalog is generated from the
same source of truth the checker itself uses, so it never drifts from
actual behavior.

## Using it as an agent skill

This repo ships a self-contained [Agent Skill](skill/) (`skill/SKILL.md`)
that wraps this CLI so an AI coding agent can invoke it directly while
writing or reviewing a `.sysml` model, instead of guessing whether a
model is valid. It follows the open Agent Skills format, so it works
with Claude Code, [opencode](https://opencode.ai/docs/skills/), or any
other tool that reads `SKILL.md`.

```sh
skill/scripts/install.sh          # once per machine: puts sysml-check on $PATH
skill/scripts/check.sh model.sysml [flags]   # what the skill actually calls
```

`check.sh` resolves the binary via an explicit `$SYSML_CHECK_BIN`
override, then `$PATH`, then a local build from this repo as a last
resort — so the skill folder keeps working even if it's copied out to a
different location than the rest of this repo. See
[`skill/references/install.md`](skill/references/install.md) for
distro-by-distro install notes and
[`skill/references/json-schema.md`](skill/references/json-schema.md)
for the full output schema.

## Architecture

Hand-rolled pipeline, no parser-generator or external crate:

```
lexer.rs   → tokens
parser.rs  → recursive-descent parser → a single uniform ast.rs::Node type
model.rs   → Model::build(nodes) → a flat symbol table for name resolution
rules.rs   → semantic checks over the Model; CATALOG is the single
             source of truth for every diagnostic code
diag.rs    → Diagnostic/Span types, human-readable rendering
json.rs    → JSON output
main.rs    → CLI parsing, orchestration, exit codes
stdlib.rs  → known standard-library names (ISQ, ScalarValues, ...)
```

The key design choice: a `part def`, a `part` usage, and a `connect`
relationship are all the *same* struct — `NodeKind`/`RelKind` and a
`keyword` string carry the semantics, rather than a large enum of node
variants. Expression/action bodies are treated as opaque (scanned for
referenced names, not structurally parsed) — a deliberate scope
boundary, not an oversight. See [`CLAUDE.md`](CLAUDE.md) for the full
architecture writeup, including why `QName` deliberately conflates `.`
and `::`.

## Development

```sh
cargo build --all-targets   # must be warning-free
cargo test                  # 140 unit tests (inline in src/*.rs) +
                             # 82 CLI integration tests (tests/cli.rs)
```

Tests are backed by fixture corpora under `tests/fixtures/`: 15 files
that must produce zero diagnostics (`valid/`), 39 that must trigger one
specific code (`invalid/`), and 12 documenting edge cases and known
limitations (`edge/`). See [`CLAUDE.md`](CLAUDE.md) for fixture/test
conventions before contributing — they're consistent across the whole
suite and new tests are expected to follow the same shape.

## Known limitations

A systematic audit against the reference grammar/validator closed most
gaps, but some are deliberately out of scope for now: `.` vs `::`
conflation, `$::` global qualification, a handful of rare KerML
relationship keywords (`chains`, `inverse of`, `featured by`, ...),
`ExtendedDefinition`/`ExtendedUsage`, and 7 low-severity gaps from the
coverage audit (documented in [`CLAUDE.md`](CLAUDE.md), each with either
a dedicated fixture explaining the boundary or a note on why it isn't
implemented yet).

## References

- [OMG SysML v2 specification](https://www.omg.org/spec/SysML/2.0/) (formal, v2.0)
- [OMG KerML specification](https://www.omg.org/spec/KerML/1.0/) (formal, v1.0)
- [SysML v2 Pilot Implementation](https://github.com/Systems-Modeling/SysML-v2-Pilot-Implementation) — reference grammar/validator this tool is checked against
- [SysML v2 Release](https://github.com/Systems-Modeling/SysML-v2-Release) — spec artifacts and examples

## License

GPL-3.0-only — see [`LICENSE`](LICENSE).
