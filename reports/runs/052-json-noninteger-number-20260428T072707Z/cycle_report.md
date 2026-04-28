# Cycle Report: 052 JSON non-integer number diagnostics

Run ID: `052-json-noninteger-number-20260428T072707Z`
Issue: `052`
Branch: `agent/052-json-noninteger-number-20260428T072707Z`
Implementation commit: `5cbb6487d18b21ab367fcbc1ba12e2d3e74e06b0`
Status: `PROGRESS`

## Scope

Implemented the assigned narrow coverage slice for `JSON.parse` non-integer JSON numbers that the current tagged small-int runtime cannot represent. No backend runtime change was needed: the existing `$json_parse_number_value` path already traps when a decimal/exponent form cannot be reduced to an integer-valued small-int.

## Changes

- Added top-level coverage for `JSON.parse("1.5")`.
- Added array-value coverage for `JSON.parse("[1.5]")`.
- Added object-value coverage for `JSON.parse("{\"n\":1.5}")`.
- Added `m2_node_diff` coverage that classifies these cases as accepted by Node and rejected by iwasm with an `unreachable` trap.
- Recorded progress evidence in `issues/open/052-implement-json.md`.

## Evidence

Pre-change probe using `/tmp/ts2wasm-json-noninteger-probe.ts`:

```text
node_status=0
build_status=0
iwasm_status=1
Exception: unreachable
```

Direct fixture evidence:

```text
node fixtures/builtins-and-io/json-parse-unsupported-noninteger-number.ts
status=0 stdout=accepted

node fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-array.ts
status=0 stdout=accepted

node fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-object.ts
status=0 stdout=accepted

cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-noninteger-number.ts -o /tmp/ts2wasm-json-noninteger-number.wasm
status=0
iwasm /tmp/ts2wasm-json-noninteger-number.wasm
status=1 stderr=Exception: unreachable

cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-array.ts -o /tmp/ts2wasm-json-noninteger-number-array.wasm
status=0
iwasm /tmp/ts2wasm-json-noninteger-number-array.wasm
status=1 stderr=Exception: unreachable

cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-object.ts -o /tmp/ts2wasm-json-noninteger-number-object.wasm
status=0
iwasm /tmp/ts2wasm-json-noninteger-number-object.wasm
status=1 stderr=Exception: unreachable
```

Validation passed:

```text
cargo fmt --all --check
cargo nextest run -E 'test(json)'
cargo nextest run -p ts2wasm-cli json
node fixtures/builtins-and-io/json-parse-unsupported-noninteger-number.ts
node fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-array.ts
node fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-object.ts
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-noninteger-number.ts -o /tmp/ts2wasm-json-noninteger-number.wasm
iwasm /tmp/ts2wasm-json-noninteger-number.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-array.ts -o /tmp/ts2wasm-json-noninteger-number-array.wasm
iwasm /tmp/ts2wasm-json-noninteger-number-array.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-object.ts -o /tmp/ts2wasm-json-noninteger-number-object.wasm
iwasm /tmp/ts2wasm-json-noninteger-number-object.wasm
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager check-repo-smoke
cargo nextest run
```

## Outcome

Issue 052 remains open because full JSON support is broader than this assigned child slice. This branch is a validated PROGRESS slice with regression coverage for the non-integer number unsupported path.

Remaining known gaps from issue 052 include arbitrary non-integer JSON number representation, full UTF-16/non-ASCII string representation, full surrogate-pair support, full replacer semantics, and broader throw-compatible parse diagnostics.
