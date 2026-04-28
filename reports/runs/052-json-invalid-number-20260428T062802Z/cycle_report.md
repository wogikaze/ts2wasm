# Cycle Report: 052 JSON invalid number diagnostics

Run ID: `052-json-invalid-number-20260428T062802Z`
Branch: `agent/052-json-invalid-number-20260428T062802Z`
Issue: `issues/open/052-implement-json.md`
Implementation commit: `48c6345`
Status: PROGRESS, merge requested

## Scope

Implemented the assigned narrow `JSON.parse` invalid-number diagnostics slice for leading-zero number tokens. This does not broaden numeric representation beyond the existing integer-valued runtime model.

## Changes

- Added leading-zero validation to `$json_parse_number_value` and `$json_skip_number`.
- Added rejection fixtures for top-level, array-value, and object-value parse paths:
  - `fixtures/builtins-and-io/json-parse-invalid-number-leading-zero.ts`
  - `fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-array.ts`
  - `fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-object.ts`
- Added Node/iwasm rejection coverage in `crates/cli/tests/m2_node_diff.rs`.
- Recorded progress evidence in `issues/open/052-implement-json.md`.

## Evidence

Pre-change leading-zero probes:

- `JSON.parse('01')`: Node rejected with JSON `SyntaxError`; iwasm accepted and printed `accepted`.
- `JSON.parse('[01]')`: Node rejected with JSON `SyntaxError`; iwasm accepted and printed `accepted`.
- `JSON.parse('{"a":01}')`: Node rejected with JSON `SyntaxError`; iwasm accepted and printed `accepted`.

Post-change direct evidence:

- `node fixtures/builtins-and-io/json-parse-invalid-number-leading-zero.ts`: rejected with JSON `SyntaxError`, status 1.
- `node fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-array.ts`: rejected with JSON `SyntaxError`, status 1.
- `node fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-object.ts`: rejected with JSON `SyntaxError`, status 1.
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-invalid-number-leading-zero.ts -o /tmp/ts2wasm-json-invalid-number-leading-zero.wasm && iwasm /tmp/ts2wasm-json-invalid-number-leading-zero.wasm`: rejected with `Exception: unreachable`, status 1.
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-array.ts -o /tmp/ts2wasm-json-invalid-number-leading-zero-array.wasm && iwasm /tmp/ts2wasm-json-invalid-number-leading-zero-array.wasm`: rejected with `Exception: unreachable`, status 1.
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-invalid-number-leading-zero-object.ts -o /tmp/ts2wasm-json-invalid-number-leading-zero-object.wasm && iwasm /tmp/ts2wasm-json-invalid-number-leading-zero-object.wasm`: rejected with `Exception: unreachable`, status 1.

Already-rejected incomplete-number probes:

- `JSON.parse('1.')`: Node rejected; iwasm rejected with `Exception: unreachable`.
- `JSON.parse('1e')`: Node rejected; iwasm rejected with `Exception: unreachable`.
- `JSON.parse('-')`: Node rejected; iwasm rejected with `Exception: unreachable`.

## Validation

- `cargo fmt --all --check`: pass
- `cargo nextest run -E 'test(json)'`: pass, 15 passed
- `cargo nextest run -p ts2wasm-cli json`: pass, 12 passed
- `scripts/manager check-issue-health`: pass
- `scripts/manager check-agent-state`: pass
- `cargo nextest run`: pass, 342 passed, 4 skipped

## Remaining Work

Issue 052 remains open. Remaining umbrella gaps include arbitrary non-integer JSON number representation, non-ASCII `\uXXXX`/surrogate handling, full replacer semantics, and broader throw-compatible parse diagnostics.
