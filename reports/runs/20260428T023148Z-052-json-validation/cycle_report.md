# Issue 052 JSON Validation Progress

Run ID: `20260428T023148Z-052-json-validation`
Branch: `agent/052-json-validation-20260428T022516Z`
Base: `8988f17`

## Outcome

PROGRESS. Implemented strict top-level trailing-token validation for `JSON.parse`.

The previous runtime accepted `JSON.parse('{"a":1} trailing')`, printed `unreachable`, and exited 0. The new runtime parses the valid prefix, skips trailing JSON whitespace, and traps if any non-whitespace token remains.

## Changed Files

- `crates/backend-wasm/src/runtime_builtins_host.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/builtins-and-io/json-parse-trailing-invalid.ts`
- `issues/open/052-implement-json.md`

## Validation

- `cargo fmt --all --check`: passed
- `cargo nextest run -E 'test(json)'`: passed, 11 passed
- `cargo nextest run -p ts2wasm-cli json`: passed, 8 passed
- `node fixtures/builtins-and-io/json-parse-trailing-invalid.ts`: expected rejection, status 1, JSON `SyntaxError`
- `cargo run -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-trailing-invalid.ts -o /tmp/ts2wasm-json-parse-trailing-invalid.wasm && iwasm /tmp/ts2wasm-json-parse-trailing-invalid.wasm`: expected rejection at iwasm, `Exception: unreachable`, status 1
- `scripts/manager check-issue-health`: passed
- `scripts/manager check-agent-state`: passed

Full `cargo nextest run` was not run because the parent check-in requested focused JSON gates now.

## Remaining Gaps

- Escaped strings
- Decimals and exponents
- Stricter incomplete-token validation
- Explicit object-element-inside-array regression coverage
- `JSON.stringify` replacer and space arguments
- Throw-compatible parse diagnostics instead of runtime trap
