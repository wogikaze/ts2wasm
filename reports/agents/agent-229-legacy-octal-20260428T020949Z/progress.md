# Issue 229 Progress Report

Run: `20260428T021845Z-229-legacy-octal`
Branch: `agent/229-legacy-octal-20260428T020949Z`

## Outcome

Status: PROGRESS

Implemented the selected legacy octal slice:

- Non-strict string legacy octal escapes are cooked during lexing, including template interpolation source such as `` `${'\07'}` ``.
- Strict source rejects legacy octal string escapes with an issue-linked `UnsupportedSyntax` diagnostic.
- Untagged template literal text rejects legacy octal escapes with an issue-linked `UnsupportedSyntax` diagnostic.
- Existing template interpolation behavior is preserved by passing strict context into interpolation parsing.

The issue remains open because the assignment requested a `PROGRESS` parent event, not issue closure.

## Evidence

```text
cargo test -p ts2wasm-frontend legacy_octal -- --nocapture
result: PASS, 4 passed

cargo nextest run -p ts2wasm-cli template_literal_legacy_octal
result: PASS, 2 passed

node fixtures/core-semantics/template-literal-legacy-octal.ts
result: PASS, stdout true / 0

cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/template-literal-legacy-octal.ts -o /tmp/ts2wasm-template-legacy-octal.wasm && iwasm /tmp/ts2wasm-template-legacy-octal.wasm
result: PASS, stdout true / 0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --path-filter legacy-octal-escape-sequence --limit 750 --detail
result: PASS, unsupported_features no longer includes legacy-octal-escape

cargo fmt --all --check
result: PASS

scripts/manager check-agent-state
result: PASS

scripts/manager check-issue-health
result: PASS

cargo nextest run
result: PASS, 289 passed, 4 skipped
```

## Remaining Work

The filtered reference run still reports unrelated unsupported outcomes after the legacy octal classification is removed:

- Annex B template legacy-octal files now reach `UnresolvedName` for test262 harness globals (`assert`, `$DONOTEVALUATE`).
- Invalid template-octal files classify as `template-literal` diagnostics.
- String literal legacy-octal files outside this assignment classify under `function` or `name-resolution`.
