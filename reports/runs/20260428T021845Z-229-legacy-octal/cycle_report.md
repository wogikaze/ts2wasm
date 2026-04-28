# Cycle Report: Issue 229 Legacy Octal Escape Handling

Status: PROGRESS
Date: 2026-04-28
Branch: `agent/229-legacy-octal-20260428T020949Z`

## Scope

Implemented one reference-backed legacy octal slice for template literals:

- Support non-strict legacy octal escapes in string literals parsed inside template interpolation.
- Reject strict legacy octal escapes with an issue-linked diagnostic.
- Reject legacy octal escapes in untagged template literal text with an issue-linked diagnostic.

## Validation

```text
cargo test -p ts2wasm-frontend legacy_octal -- --nocapture
PASS: 4 passed

cargo nextest run -p ts2wasm-frontend legacy_octal
PASS: 4 passed

cargo nextest run -p ts2wasm-cli template_literal
PASS: 3 passed

node fixtures/core-semantics/template-literal-legacy-octal.ts
PASS: stdout true / 0

cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/template-literal-legacy-octal.ts -o /tmp/ts2wasm-template-legacy-octal.wasm && iwasm /tmp/ts2wasm-template-legacy-octal.wasm
PASS: stdout true / 0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --path-filter legacy-octal-escape-sequence --limit 750 --detail
PASS: unsupported_features=name-resolution:4,template-literal:3,function:1; no legacy-octal-escape remains

cargo fmt --all --check
PASS

scripts/manager check-agent-state
PASS

scripts/manager check-issue-health
PASS

cargo nextest run
PASS: 289 passed, 4 skipped
```

## Acceptance Evidence

- Classified test262 legacy-octal-escape cases no longer report `legacy-octal-escape`: verified by filtered reference coverage.
- Strict and non-strict template literal legacy octal cases have regression coverage:
  - `fixtures/core-semantics/template-literal-legacy-octal.ts`
  - `fixtures/core-semantics/template-literal-legacy-octal-strict-unsupported.ts`
- Existing template literal interpolation fixtures still pass: `cargo nextest run -p ts2wasm-cli template_literal`.
- Full required validation passed: `cargo fmt --all --check`, `cargo nextest run`.
