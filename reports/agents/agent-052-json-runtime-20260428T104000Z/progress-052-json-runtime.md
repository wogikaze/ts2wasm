# Progress report: issue 052 JSON runtime builtins

Date: 2026-04-28
Branch: `agent/052-json-runtime-20260428T104000Z`
Outcome: PROGRESS

## Slice completed

- Reproduced the existing semantic gap:
  - `JSON.stringify({ a: 1, b: 2 })` printed `263` under iwasm instead of `{"a":1,"b":2}`.
  - `JSON.parse('{"a":1,"b":2}').a` printed `undefined` under iwasm instead of `1`.
- Implemented the first runtime slice:
  - `JSON.stringify`: small-int primitives, ASCII strings, arrays, and flat object serialization.
  - `JSON.parse`: whitespace-trimmed primitives, ASCII strings, and flat objects with string keys and small-int/string/boolean/null values.
- Added Node differential regression coverage for the existing JSON fixtures.

## Validation

Passed:

```text
cargo run -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify.ts -o /tmp/ts2wasm-json-stringify.wasm && node fixtures/builtins-and-io/json-stringify.ts && iwasm /tmp/ts2wasm-json-stringify.wasm
stdout: {"a":1,"b":2} from Node and iwasm

cargo run -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse.ts -o /tmp/ts2wasm-json-parse.wasm && node fixtures/builtins-and-io/json-parse.ts && iwasm /tmp/ts2wasm-json-parse.wasm
stdout: 1 from Node and iwasm

cargo nextest run -p ts2wasm-cli json_fixtures_match_node_output_under_iwasm
1 test passed

cargo nextest run -p ts2wasm-cli json
7 tests passed

cargo fmt --all --check
pass

scripts/manager update-issue-index --check
pass

scripts/manager check-issue-health
pass

scripts/manager check-repo-smoke
pass
```

## Remaining work

Issue 052 remains open. This is not full JSON compliance yet. Known gaps include escaped strings, decimal/exponent numbers, nested parsed values, parsed arrays, replacer/space arguments, and JavaScript-compatible parse error throwing.
