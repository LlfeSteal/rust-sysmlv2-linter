# sysml-check CLI reference

Full flag list for the `sysml-check` binary this skill wraps (source of
truth: `src/main.rs`'s `--help` text and arg parser). The wrapper script
`skill/scripts/check.sh` always adds `--format json` before forwarding
whatever arguments you pass it, so you don't need to pass `--format`
yourself unless you deliberately want to override it (e.g. `--format
human` for a one-off readable dump, or `--format gitlab` for CI-style
output — see `json-schema.md` for both shapes).

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

Multiple `<FILE>` arguments are accepted in one invocation (checked as
one combined model). `--stdin` reads a single in-memory model instead of
/ in addition to file args — useful for checking content an agent is
about to write, before it hits disk (pair with `--name` so diagnostics
show a sensible filename).

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | No errors — model is syntactically and semantically consistent |
| 1 | At least one error (or a warning, if `--deny-warnings` was passed) |
| 2 | Usage error or I/O error (bad flag, missing file, unreadable file) |

## When to reach for non-default flags

- **`--pedantic`**: turn on when the user cares about style/best-practice
  conformance, not just correctness. Off by default because these rules
  are opinionated, not required by the grammar.
- **`--unresolved warn`** or **`--unresolved off`**: use when checking a
  fragment of a larger model (e.g. one file out of many, or content
  that intentionally references not-yet-imported/not-yet-written
  elements) so unresolved-name errors don't drown out real problems.
  Default is `error`, which is right when checking a complete model.
- **`--list-rules`**: call with no file arguments to get the full rule
  catalog (`[{"code","rule","description"}, ...]`) — useful for an agent
  that wants to explain *why* a diagnostic code matters, or to check
  whether a rule exists before assuming a construct is unsupported.
- **`--emit ast`** / **`--emit both`**: use when you need structural
  information about the model (symbol tree, resolved references) rather
  than just diagnostics — e.g. to answer "does this model already define
  a part named X" programmatically instead of re-parsing text. See
  `json-schema.md` for the AST node shape.
