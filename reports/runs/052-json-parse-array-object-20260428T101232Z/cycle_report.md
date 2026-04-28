# Cycle Report: 052-json-parse-array-object-20260428T101232Z

## Outcome

PROGRESS for issue 052. Added focused regression coverage for `JSON.parse` arrays whose object elements contain nested object and array/object values.

No backend/runtime code changed. A direct pre-change probe showed the current runtime already matches Node for this continuation slice.

## Scope

- Added `fixtures/builtins-and-io/json-parse-array-object-nested.ts`
- Added the fixture to the JSON Node differential list in `crates/cli/tests/m2_node_diff.rs`
- Recorded progress evidence in `issues/open/052-implement-json.md`

## Evidence

Pre-change probe source:

```ts
let arr = JSON.parse('[{"a":{"b":1}},{"c":[2,{"d":3}]}]');
console.log(arr.length);
console.log(arr[0].a.b);
console.log(arr[1].c.length);
console.log(arr[1].c[0]);
console.log(arr[1].c[1].d);
```

Node and iwasm both printed:

```text
2
1
2
2
3
```

## Validation

Passed:

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(json)'`
- `cargo nextest run -p ts2wasm-cli json`
- `node fixtures/builtins-and-io/json-parse-array-object.ts`
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-array-object.ts -o /tmp/ts2wasm-052-json-array-object.wasm && iwasm /tmp/ts2wasm-052-json-array-object.wasm`
- `node fixtures/builtins-and-io/json-parse-array-object-nested.ts`
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-array-object-nested.ts -o /tmp/ts2wasm-052-json-array-object-nested.wasm && iwasm /tmp/ts2wasm-052-json-array-object-nested.wasm`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

Skipped:

- `cargo nextest run`: not required for this PROGRESS slice because no backend runtime code changed and issue 052 remains open.

## Remaining Issue 052 Gaps

- Arbitrary non-integer JSON number representation
- Full UTF-16/non-ASCII string representation
- Full surrogate-pair support
- Broader replacer semantics beyond the current string-literal object-literal subset
- Non-stringify `space` ignored-value parity requiring IR validation work
- Broader throw-compatible parse diagnostics
