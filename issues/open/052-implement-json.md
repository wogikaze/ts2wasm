---
id: 052
title: "Implement JSON"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement JSON.parse and JSON.stringify.

## Problem

JSON is not implemented. It is essential for data serialization.

## Desired final state

`JSON.parse()` and `JSON.stringify()` work correctly.

## Scope

In scope:

- [ ] Implement JSON.parse
- [ ] Implement JSON.stringify
- [ ] Add fixtures for JSON behavior

Out of scope:

- Full JSON spec compliance (start with common cases)

## Affected paths

Expected:

- `crates/backend-wasm/src/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] JSON.parse works correctly
- [ ] JSON.stringify works correctly
- [ ] Fixtures cover JSON behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/json-test.ts -o /tmp/test.wasm
iwasm /tmp/test.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Note: fixtures/builtins-and-io/json-*.ts exist but may not be fully implemented.

## Progress evidence

2026-04-28:

- Implemented a first runtime slice for `JSON.stringify` covering small-int primitives, ASCII strings, arrays, and flat object serialization.
- Implemented a first runtime slice for `JSON.parse` covering whitespace-trimmed primitives, ASCII strings, and flat objects with string keys and small-int/string/boolean/null values.
- Added Node differential coverage for the existing JSON fixtures in `crates/cli/tests/m2_node_diff.rs`.
- Existing fixture evidence:
  - `fixtures/builtins-and-io/json-stringify.ts`: Node and iwasm both print `{"a":1,"b":2}`.
  - `fixtures/builtins-and-io/json-parse.ts`: Node and iwasm both print `1`.
- Remaining gaps before close: escaped strings, decimals/exponents, nested parse values, arrays in `JSON.parse`, replacer/space arguments, and throw-compatible parse diagnostics remain outside this first slice.

2026-04-28:

- Implemented a next runtime slice for `JSON.parse` covering top-level arrays with small-int, ASCII string, boolean, and null elements.
- Added Node differential coverage in `fixtures/builtins-and-io/json-parse-array.ts`; Node and iwasm both print:

```text
5
1
two
true
false
null
```

- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -p ts2wasm-cli json`
  - `cargo nextest run -E 'test(json)'`
  - `cargo run -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-array.ts -o /tmp/ts2wasm-json-parse-array.wasm && iwasm /tmp/ts2wasm-json-parse-array.wasm`
  - `node fixtures/builtins-and-io/json-parse-array.ts`
  - `cargo nextest run`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
  - `python -m jsonschema -i reports/runs/052-json-next-20260428T013435Z/test_report.json .agents/state/schemas/test_report.schema.json`
  - `scripts/manager check-repo-smoke`
- Remaining gaps before close: escaped strings, decimals/exponents, nested arrays/objects in parsed values, arrays inside parsed object values, object elements inside parsed arrays, replacer/space arguments, and throw-compatible parse diagnostics remain outside this slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
