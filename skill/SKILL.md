---
name: sysml-lint
description: Write, build, verify, lint, or validate SysML v2 textual-notation (.sysml) models. Use whenever the user asks to author, edit, check, or fix a SysML v2 file, or mentions SysML v2 syntax/semantic errors — not for classic/legacy graphical SysML. Wraps the sysml-check CLI to run compiler-style checks with machine-readable diagnostics.
---

# SysML v2 lint & verify

This skill wraps `sysml-check`, a zero-dependency Rust CLI in this repo
that parses and semantically checks SysML v2 **textual notation**
(`.sysml`) files and reports diagnostics (syntax errors, unresolved
names, semantic rule violations, optional style warnings).

## When to use this

- **Authoring**: after writing or editing any `.sysml` file, run the
  checker before telling the user the model is done. Treat it like a
  compiler: write/edit → run the checker → fix every reported error →
  re-run → repeat until `summary.ok` is `true`.
- **Verification**: whenever explicitly asked to lint, check, validate,
  or find problems in an existing SysML v2 model.
- **Debugging**: when the user reports something "doesn't work" in a
  `.sysml` file and hasn't already named the specific error — run the
  checker first instead of guessing from the source text.

Do not use this for classic/legacy (graphical) SysML — this tool only
understands the SysML v2 textual notation grammar.

## How to invoke it

```
skill/scripts/check.sh <file1.sysml> [file2.sysml ...] [flags]
```

Run it from anywhere, on any copy of this skill/repo. It resolves which
`sysml-check` binary to run in order: an explicit `$SYSML_CHECK_BIN`
override, then a `sysml-check` already on `$PATH` (the normal case once
it's installed — see `references/install.md`), then finally a
best-effort local build from this repo's source if it's still reachable
above the script. It always requests JSON output, so you can parse the
result directly instead of reading terminal text. If it exits with "not
installed" (exit code 2), follow `references/install.md` or run
`skill/scripts/install.sh`.

Common flags to forward (full list in `references/cli-reference.md`):
- `--pedantic` — also enable style/best-practice rules, not just
  correctness rules. Turn on when the user cares about idiomatic style.
- `--unresolved warn` or `--unresolved off` — relax unresolved-name
  errors to warnings/off when checking a deliberately incomplete
  fragment (e.g. one file that references not-yet-written elements).
- `--list-rules` (no file args) — print the full rule catalog, useful
  for explaining *why* a code fired or confirming a construct is
  actually unsupported rather than just unrecognized.
- `--stdin --name foo.sysml` — check content before it's even written to
  disk, by piping it to the script's stdin.

## Reading the output

The script always emits:

```json
{
  "summary": {"files": 1, "errors": 0, "warnings": 0, "infos": 0, "ok": true},
  "diagnostics": [
    {"file": "...", "code": "E200", "rule": "...", "severity": "error",
     "message": "...", "hint": "..." | null,
     "line": N, "column": N, "endLine": N, "endColumn": N, "snippet": "..."}
  ]
}
```

Fix every `"severity": "error"` entry — these are what block
`summary.ok`. `"warning"`/`"info"` entries are worth surfacing to the
user but don't mean the model is broken. Full schema, including
`--emit ast` (structural symbol tree with resolved references) and
`--format gitlab` (CI report shape), is in `references/json-schema.md`.

## Exit codes

`0` = clean, `1` = at least one blocking diagnostic (errors, or warnings
too if `--deny-warnings` was passed), `2` = usage/I-O error (bad flag,
missing file). Don't rely on the exit code alone to decide `ok` under
`--deny-warnings` — check `summary` fields directly.
