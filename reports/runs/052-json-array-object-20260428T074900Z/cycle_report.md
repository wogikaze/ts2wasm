# Cycle Report: 052-json-array-object-20260428T074900Z

- Issue: `issues/open/052-implement-json.md`
- Branch: `agent/052-json-array-object-20260428T074900Z`
- Outcome: PROGRESS
- Scope: focused `JSON.parse` regression coverage for reading object properties from object elements inside parsed arrays.

## Summary

The exact assigned probe, `JSON.parse('[{"n":1},{"n":2}]')` with `arr[0].n` and `arr[1].n` reads, already matched Node under iwasm before any backend/runtime changes. This run added a focused regression fixture and registered it in the JSON Node differential test set. No backend files were changed.

## Evidence

Direct pre-change probe:

```text
node /tmp/ts2wasm-json-array-object-probe.ts
2
1
2

cargo run -q -p ts2wasm-cli -- build /tmp/ts2wasm-json-array-object-probe.ts -o /tmp/ts2wasm-json-array-object-probe.wasm && iwasm /tmp/ts2wasm-json-array-object-probe.wasm
2
1
2
```

New fixture:

```text
fixtures/builtins-and-io/json-parse-array-object-properties.ts
```

Direct fixture evidence:

```text
node fixtures/builtins-and-io/json-parse-array-object-properties.ts
2
1
2

cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-array-object-properties.ts -o /tmp/ts2wasm-json-parse-array-object-properties.wasm && iwasm /tmp/ts2wasm-json-parse-array-object-properties.wasm
2
1
2
```

## Validation

- PASS: `cargo fmt --all --check`
- PASS: `cargo nextest run -E 'test(json)'` (`17 passed, 336 skipped`)
- PASS: `cargo nextest run -p ts2wasm-cli json` (`14 passed, 211 skipped`)
- PASS: `scripts/manager check-issue-health`
- PASS: `scripts/manager check-agent-state`
- PASS: `cargo nextest run` (`349 passed, 4 skipped`)

## Files Changed

- `fixtures/builtins-and-io/json-parse-array-object-properties.ts`
- `crates/cli/tests/m2_node_diff.rs`
- `issues/open/052-implement-json.md`
- `reports/agents/052-json-array-object-20260428T074900Z/assignment.md`
- `reports/runs/052-json-array-object-20260428T074900Z/cycle_report.md`
- `reports/runs/052-json-array-object-20260428T074900Z/test_report.json`

## Status

Issue 052 remains open. This run records PROGRESS only because the full JSON issue still has broader gaps outside this assigned slice.
