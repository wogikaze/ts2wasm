---
id: 052b
title: "Implement JSON non-integer number representation"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: `JSON.parse` currently rejects JSON numbers that cannot be reduced exactly to the tagged small-int representation, even though Node accepts them as Number values.

## Summary

Extend JSON number parsing and value representation so non-integer JSON numbers produce JavaScript Number-compatible runtime values instead of trapping as unsupported.

## Current failure

Existing issue 052 evidence records Node accepting `JSON.parse("1.5")`, `JSON.parse("[1.5]")`, and `JSON.parse("{\"n\":1.5}")`, while iwasm rejects the first parsed non-integer with `Exception: unreachable`.

## Desired final state

`JSON.parse` accepts integer and non-integer JSON number grammar forms and returns values that behave consistently with the project's JavaScript Number model.

## Scope

In scope:

- [ ] Parse decimal and exponent JSON numbers that produce non-integer numeric values.
- [ ] Preserve existing exact small-int behavior.
- [ ] Add Node differential coverage for top-level, array-value, and object-value non-integer parses.

Out of scope:

- Full BigInt support.
- Full IEEE-754 optimization work outside behavior needed for JSON values.
- UTF-16/string parsing changes.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/ir/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/`
- `issues/open/052-implement-json.md`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] `JSON.parse("1.5")` matches Node for observable output.
- [ ] `JSON.parse("[1.5]")[0]` matches Node for observable output.
- [ ] `JSON.parse("{\"n\":1.5}").n` matches Node for observable output.
- [ ] Existing JSON integer, decimal-to-integer, and invalid-number rejection fixtures still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(json)'
cargo nextest run -p ts2wasm-cli json
cargo nextest run
mise run check issues
```

Impacted commands:

```sh
node fixtures/builtins-and-io/json-parse-unsupported-noninteger-number.ts
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-noninteger-number.ts -o /tmp/ts2wasm-json-noninteger-number.wasm && iwasm /tmp/ts2wasm-json-noninteger-number.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] update `current-state.md` if the supported JSON subset changes

Follow-up issues:

- [ ] update `issues/open/052-implement-json.md`

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
