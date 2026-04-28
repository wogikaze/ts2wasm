# Issue 052 JSON Unicode Diagnostics Progress

Run ID: `052-json-unicode-diagnostics-20260428T065856Z`
Branch: `agent/052-json-unicode-diagnostics-20260428T065856Z`
Implementation commit: `b034261`

## Outcome

PROGRESS. Added regression coverage for the existing narrow `JSON.parse` unicode escape diagnostic behavior without implementing broader UTF-16 or surrogate support.

The runtime already rejects invalid unicode escape hex and `\uXXXX` code points outside the current ASCII string representation through `$json_parse_unicode_escape_byte`. This slice pins that behavior for top-level string, array string value, and object string value parse paths.

## Changed Files

- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/builtins-and-io/json-parse-invalid-unicode-escape.ts`
- `fixtures/builtins-and-io/json-parse-unsupported-unicode-array.ts`
- `fixtures/builtins-and-io/json-parse-unsupported-unicode-object.ts`
- `issues/open/052-implement-json.md`
- `reports/runs/052-json-unicode-diagnostics-20260428T065856Z/cycle_report.md`
- `reports/runs/052-json-unicode-diagnostics-20260428T065856Z/test_report.json`

## Evidence

- `JSON.parse('"\\u00G0"')`: Node rejects with JSON `SyntaxError`; iwasm rejects with `Exception: unreachable`.
- `JSON.parse('["\\u00e9"]')`: Node accepts; iwasm rejects with `Exception: unreachable` because non-ASCII string representation is unsupported in this runtime slice.
- `JSON.parse('{"s":"\\ud800"}')`: Node accepts; iwasm rejects with `Exception: unreachable` because surrogate representation is unsupported in this runtime slice.

## Validation

- `cargo fmt --all --check`: passed
- `cargo nextest run -E 'test(json)'`: passed, 16 passed
- `cargo nextest run -p ts2wasm-cli json`: passed, 13 passed
- `cargo nextest run`: passed, 344 passed, 4 skipped
- `scripts/manager check-agent-state`: passed
- `scripts/manager check-issue-health`: passed
- Direct Node/iwasm fixture evidence: passed with expected accept/reject outcomes listed above

Additional gate note: `cargo clippy --all-targets --all-features -- -D warnings` failed on existing `clippy::assertions_on_constants` diagnostics in `crates/runtime-abi/src/layout.rs`. That file is outside the allowed paths for this child and was not changed.

## Remaining Gaps

- Arbitrary non-integer JSON number representation
- Full UTF-16/non-ASCII runtime string representation
- Full surrogate-pair support
- Full `JSON.stringify` replacer semantics
- Throw-compatible `JSON.parse` diagnostics instead of runtime traps
