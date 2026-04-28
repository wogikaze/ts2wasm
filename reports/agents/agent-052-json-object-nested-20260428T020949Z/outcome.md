# Outcome: issue 052 JSON object nested values

Status: PROGRESS

Implemented evidence:

- Added `fixtures/builtins-and-io/json-parse-object-nested.ts` for `JSON.parse` object values containing an array and a nested object with its own array.
- Registered the fixture in `crates/cli/tests/m2_node_diff.rs` under the JSON Node/iwasm differential fixture set.
- No backend runtime change was needed: a temporary pre-edit reproduction with the same JSON shape already matched Node and iwasm output.

Expected stdout:

```text
2
2
3
4
```

Validation:

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(json)'`
- `cargo nextest run -p ts2wasm-cli json`
- `node fixtures/builtins-and-io/json-parse-object-nested.ts`
- `cargo run -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-object-nested.ts -o /tmp/ts2wasm-json-parse-object-nested.wasm && iwasm /tmp/ts2wasm-json-parse-object-nested.wasm`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

Full `cargo nextest run` skipped: no runtime parser code changed in this child slice.
