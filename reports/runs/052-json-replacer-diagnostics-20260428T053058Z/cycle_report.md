# Cycle Report: issue 052 JSON replacer diagnostics

- Run id: `052-json-replacer-diagnostics-20260428T053058Z`
- Branch: `agent/052-json-replacer-diagnostics-20260428T053058Z`
- Issue: `052`
- Outcome: `PROGRESS`

## Scope

Added precise diagnostics and regression coverage for unsupported `JSON.stringify` replacer forms. This slice intentionally does not implement replacer callback or array property-list semantics.

## Changes

- `crates/ir/src/lowered.rs`: issue-052 diagnostics now distinguish function replacer callbacks, declared function replacer identifiers, array replacer property lists, and other unsupported replacer values.
- `fixtures/builtins-and-io/json-stringify-replacer-function-unsupported.ts`: diagnostic fixture for a declared function replacer.
- `fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts`: diagnostic fixture for an array replacer.
- `crates/cli/tests/m2_node_diff.rs`: regression test asserts both fixtures fail with `UnsupportedSyntax`, issue-052 wording, and a source span.

## Evidence

- `cargo fmt --all --check`: pass.
- `cargo nextest run -E 'test(json)'`: pass, 13 tests.
- `cargo nextest run -p ts2wasm-cli json`: pass, 10 tests.
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-function-unsupported.ts -o /tmp/ts2wasm-json-replacer-function.wasm`: expected failure with `[UnsupportedSyntax] issue-052: JSON.stringify function replacer callbacks are not supported yet ... at 59..89`.
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts -o /tmp/ts2wasm-json-replacer-array.wasm`: expected failure with `[UnsupportedSyntax] issue-052: JSON.stringify array replacer property lists are not supported yet ... at 12..49`.
- `scripts/manager check-issue-health`: pass.
- `scripts/manager check-agent-state`: pass.
- `scripts/manager check-repo-smoke`: pass.
- `python -m jsonschema -i reports/runs/052-json-replacer-diagnostics-20260428T053058Z/test_report.json .agents/state/schemas/test_report.schema.json`: pass.

Full `cargo nextest run` was not run for this PROGRESS slice because the assignment required JSON-filtered nextest commands plus direct build evidence, and issue 052 remains open.

## Remaining

- Full replacer semantics remain open for issue 052.
- Arbitrary non-integer JSON number representation, non-ASCII `\uXXXX`/surrogate handling, and throw-compatible parse diagnostics remain open.
