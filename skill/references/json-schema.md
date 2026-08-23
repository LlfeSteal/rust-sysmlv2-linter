# sysml-check JSON output schema

`skill/scripts/check.sh` always requests `--format json`. This document
describes what comes back, so you can parse it directly instead of
screen-scraping human-readable output. Source of truth:
`emit_json`/`diag_json`/`emit_sym_json`/`emit_gitlab` in `src/main.rs`.

## Default shape (`--emit diagnostics`, the default)

```json
{
  "tool": "sysml-check",
  "version": "0.1.0",
  "summary": {
    "files": 1,
    "errors": 0,
    "warnings": 1,
    "infos": 0,
    "ok": true
  },
  "diagnostics": [
    {
      "file": "path/to/model.sysml",
      "code": "W301",
      "rule": "unresolved-import",
      "severity": "warning",
      "message": "human-readable description",
      "hint": "optional suggestion, or null",
      "line": 12,
      "column": 5,
      "endLine": 12,
      "endColumn": 18,
      "snippet": "the source line the diagnostic points at"
    }
  ]
}
```

Key points:
- **`summary.ok`** is `true` iff `errors == 0`. It does **not** account
  for `--deny-warnings` — if you passed that flag, also check the
  process exit code (or treat `warnings > 0` as blocking yourself) since
  `ok` alone won't reflect it.
- **`severity`** is one of `"error" | "warning" | "info"`.
- **`hint`** is `null` when the diagnostic has no suggested fix.
- Line/column are 1-based. `endLine`/`endColumn` mark the end of the
  offending span (useful for highlighting the exact range, not just the
  start point).
- This is the loop an authoring agent should run: fix every entry with
  `severity: "error"` first (these block `ok`), then decide per-project
  whether `"warning"` entries need fixing too.

## `--emit ast` or `--emit both`

Adds a top-level `"ast"` key: an array of recursive symbol nodes (the
model's top-level elements, each with nested `children`):

```json
{
  "id": 3,
  "kind": "part-def",
  "keyword": "part def",
  "name": "Engine",
  "shortName": null,
  "qualifiedName": "VehicleModel::Engine",
  "isDefinition": true,
  "modifiers": ["abstract"],
  "relationships": [
    {"kind": "specialization", "token": ":>", "target": "Component", "resolved": "VehicleModel::Component"}
  ],
  "references": [
    {"context": "typed-by", "target": "ISQ::Mass", "resolved": "ISQ::Mass"}
  ],
  "multiplicity": {"lower": "1", "upper": "1"},
  "file": "path/to/model.sysml",
  "line": 10,
  "column": 1,
  "children": []
}
```

`multiplicity` is `null` when the element has none. `relationships[].resolved`
and `references[].resolved` are `null` when the target name didn't
resolve — this is a cheaper way to check "does X reference something
that exists" than cross-referencing diagnostics yourself.

With `--emit diagnostics` (the default), `"diagnostics"` is populated and
`"ast"` is absent entirely. With `--emit ast`, `"ast"` is populated and
`"diagnostics"` is present but forced empty (`[]`) — pass `--emit both`
if you need both at once.

## `--format gitlab` (instead of json)

A different top-level shape entirely — a bare array in GitLab Code
Quality report format, for CI pipelines rather than agent consumption:

```json
[
  {
    "description": "message — hint (if any)",
    "check_name": "E200",
    "fingerprint": "stable hash of file+code+line+col+message",
    "severity": "major" | "minor" | "info",
    "location": {"path": "path/to/model.sysml", "lines": {"begin": 12}}
  }
]
```

## `--list-rules` (independent of `--format`)

Ignores `--format`/`--emit` entirely and prints its own top-level array,
the full rule catalog:

```json
[
  {"code": "E200", "rule": "unresolved-reference", "description": "..."}
]
```
