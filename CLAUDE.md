# sysml-check

A zero-dependency Rust CLI that lints SysML v2 **textual notation** files
(`.sysml`) — not classic/legacy SysML. Single binary crate, no `[lib]`,
edition 2021, `rust-version = "1.65"`.

## Architecture

Hand-rolled pipeline, no parser-generator or external crate:

```
lexer.rs   → tokens (idents/quoted-names, strings, numbers, punctuation,
             block comments kept as "pending" attached to the next token —
             this is how doc/comment/rep bodies are captured)
parser.rs  → recursive-descent parser → a single uniform ast.rs::Node type
             (definitions/usages/relationships all share one struct;
             specialization is via `keyword` + `is_def`, not separate enum
             variants — see ast.rs::classify())
model.rs   → Model::build(nodes) turns the AST into a flat symbol table
             (Vec<Sym> with parent/children indices) for name resolution
rules.rs   → semantic checks over the Model, driven by Options
             {pedantic, unresolved}; CATALOG is the single source of truth
             for every diagnostic code
diag.rs    → Diagnostic/Span types, human-readable rendering
json.rs    → JSON output
main.rs    → CLI arg parsing, orchestration, exit codes
stdlib.rs  → known standard-library package/type names (ISQ, ScalarValues,
             ...) used to distinguish "unresolved because unknown" (E200)
             from "unresolved because unimported" (W301)
```

Key design choices worth knowing before touching this code:

- **Everything is one `Node`/`Sym` shape.** A `part def`, a `part` usage,
  and a `connect` relationship are all the same struct; `NodeKind` /
  `RelKind` and the `keyword` string (e.g. `"assert satisfy"`,
  `"analysis case def"`) carry the semantics. `ast.rs::classify()` and
  `parser.rs::is_relationship_kw()`/`base_ctx_for()` key off the **last**
  word of a combined keyword string (not the first) — this matters for
  wrapper forms like `assert satisfy`.
- **Expression/action bodies are opaque.** Constraint/calc content and
  action bodies are *not* parsed structurally — `parse_blob` just scans
  tokens, collects any referenced names for resolution, and keeps the raw
  text. Do not try to add expression-grammar parsing without checking
  whether it's actually needed; this is a deliberate scope boundary.
- **Diagnostic code scheme**: `E0xx`/`E1xx` = lexer/parser syntax errors,
  `E2xx` = semantic errors, `W2xx`/`W3xx` = warnings (some gated behind
  `--pedantic`, see `rules::Options` and `--help`). Every code has exactly
  one entry (or a shared entry for near-duplicate messages, e.g. E100 has
  two rule-name variants) in `rules::CATALOG` — that's what
  `--list-rules` prints, and what `main.rs`'s `--pedantic` help text must
  stay in sync with.
- **`QName` conflates `.` and `::`.** Real SysML v2 grammar treats `::`
  (namespace qualification) and `.` (feature chaining) as distinct
  productions; this tool's `QName` (`ast.rs`) treats both as
  interchangeable separators. Documented, not a bug — see the doc comment
  on `QName` and `tests/fixtures/edge/dot_and_double_colon_are_conflated.sysml`.

## Build & test

```bash
cargo build              # debug
cargo build --release    # optimized (lto, strip, panic=abort)
cargo run -- <file.sysml>
cargo test                # unit tests (inline in src/*.rs) + CLI integration tests (tests/cli.rs)
```

No test runner crate is used — `tests/cli.rs` is a black-box integration
test suite that shells out to the compiled binary via Cargo's
`env!("CARGO_BIN_EXE_sysml-check")`, keeping the "zero dependency"
property even for tests.

**Before considering any change done:**
```bash
cargo build --all-targets   # must be warning-free
cargo test                  # must be fully green
```
Then manually sweep `tests/fixtures/{valid,invalid,edge}/*.sysml` with
`./target/debug/sysml-check --format json <file>` for anything you
touched — the automated assertions have occasionally passed while the
actual diagnostic text/position was subtly wrong; eyeball it.

### Test/fixture conventions (follow these exactly — they're consistent
### across ~200 existing tests, don't invent a new shape)

- **Unit tests**: inline `#[cfg(test)] mod tests` at the bottom of each
  `src/*.rs` file. `parser.rs` tests parse source and assert on the AST
  shape / diagnostic codes; `rules.rs` tests call `analyze()`/
  `analyze_opts()` (parse → `Model::build` → `check`) and assert on
  diagnostic codes via the `has()`/`codes()` helpers already defined in
  that file's test module.
- **Fixtures**: one `.sysml` file per rule/construct under
  `tests/fixtures/valid/` (must produce zero diagnostics), `invalid/`
  (named `<code>_<slug>.sysml`, e.g. `e230_satisfy_target_not_requirement.sysml`,
  must trigger exactly that code — pedantic-gated ones follow the same
  naming, e.g. `w311_non_standard_keyword.sysml`), or `edge/` (edge cases:
  empty file, CRLF, Unicode, documented limitations). Every fixture opens
  with a `//` comment stating the expected diagnostic.
- **CLI tests**: one test per fixture in `tests/cli.rs`, using the
  existing helpers (`assert_ok`, `assert_has`, `assert_has_pedantic`,
  `assert_absent_without_pedantic`, `json`/`json_with` for anything
  custom) — don't hand-roll new `Command::new` calls.
- Run the exact repro (`sysml-check --format json <file>`) for anything
  new before trusting the automated assertion; several real bugs this
  project has had were parser ordering issues that only showed up when
  actually running the CLI, not from reasoning about the code.

## Known scope boundaries (don't re-litigate without reading first)

A systematic audit against the real grammar/validator (see links below)
found and this project has since closed most High/Medium gaps. What's
**still** deliberately not implemented, and why, is listed in two places —
read them before assuming something is a bug:
- `.` vs `::` conflation, `$::` global qualification, rare KerML
  relationship keywords (`chains`, `inverse of`, `featured by`, ...),
  `ExtendedDefinition`/`ExtendedUsage` — see the doc comments at their
  respective fixtures in `tests/fixtures/edge/`.
- 7 Low-severity gaps from the coverage audit (locale on doc/comment,
  filter-package imports `import P[cond];`, top-level import visibility,
  `expose`'s diagnostic context mislabeling, `succession flow`
  misclassification, empty-multiplicity `[]` cosmetic difference,
  duplicate-name/E201 severity mismatch vs the real validator) — not yet
  implemented, no dedicated fixtures.

## SysML v2 / KerML standard references

Use these when checking whether the tool's behavior matches the real
grammar/validator, not just its own rule catalog:

- **OMG SysML v2 specification** (formal, v2.0, adopted Sept 2025):
  https://www.omg.org/spec/SysML/2.0/
- **OMG KerML specification** (Kernel Modeling Language — the metamodel
  SysML v2 is built on; formal, v1.0, adopted Sept 2025):
  https://www.omg.org/spec/KerML/1.0/
- **SysML v2 Pilot Implementation** (reference Java/Xtext tooling —
  authoritative machine-checked grammar and validator, `master` branch):
  https://github.com/Systems-Modeling/SysML-v2-Pilot-Implementation
  - Concrete grammar: `org.omg.sysml.xtext/src/org/omg/sysml/xtext/SysML.xtext`
  - KerML base grammar: `org.omg.kerml.xtext/src/org/omg/kerml/xtext/KerML.xtext`
  - Expression grammar: `org.omg.kerml.expressions.xtext/src/org/omg/kerml/expressions/xtext/KerMLExpressions.xtext`
  - Reference well-formedness rules: `org.omg.sysml.xtext/src/org/omg/sysml/xtext/validation/SysMLValidator.xtend`
    and `org.omg.kerml.xtext/src/org/omg/kerml/xtext/validation/KerMLValidator.xtend`
  - Metamodel (class hierarchy, needed for `instanceof`-based validator
    rules like subject/actor/stakeholder scoping): `org.omg.sysml/model/SysML.ecore`
- **SysML v2 Release** (spec artifacts, examples, incremental releases —
  start here for what's new in a given release): https://github.com/Systems-Modeling/SysML-v2-Release
