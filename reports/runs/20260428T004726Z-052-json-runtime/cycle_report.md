# Cycle report: issue 052 JSON runtime builtins

Run ID: `20260428T004726Z-052-json-runtime`
Task ID: `052`
Outcome: PROGRESS

## Summary

Implemented a concrete first JSON runtime slice and added Node differential coverage for the existing JSON fixtures. The issue remains open because full JSON compliance is broader than this slice.

## Evidence

- `fixtures/builtins-and-io/json-stringify.ts`: Node and iwasm both print `{"a":1,"b":2}`.
- `fixtures/builtins-and-io/json-parse.ts`: Node and iwasm both print `1`.
- `cargo nextest run -p ts2wasm-cli json_fixtures_match_node_output_under_iwasm`: pass.
- `cargo nextest run -p ts2wasm-cli json`: 7 passed.
- `cargo fmt --all --check`: pass.
- `scripts/manager update-issue-index --check`: pass.
- `scripts/manager check-issue-health`: pass.
- `scripts/manager check-repo-smoke`: pass.

## Remaining gaps

- Escaped strings.
- Decimal and exponent numbers.
- Nested object/array parsing.
- `JSON.parse` array parsing.
- `JSON.stringify` replacer/space arguments.
- Throw-compatible diagnostics for invalid JSON.
