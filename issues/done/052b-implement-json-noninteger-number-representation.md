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
completed: 2026-04-29
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

- [x] Parse decimal and exponent JSON numbers that produce non-integer numeric values.
- [x] Preserve existing exact small-int behavior.
- [x] Add Node differential coverage for top-level, array-value, and object-value non-integer parses.

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

- [x] `JSON.parse("1.5")` matches Node for observable output.
- [x] `JSON.parse("[1.5]")[0]` matches Node for observable output.
- [x] `JSON.parse("{\"n\":1.5}").n` matches Node for observable output.
- [x] Existing JSON integer, decimal-to-integer, and invalid-number rejection fixtures still pass.

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

- [x] not affected

Current state:

- [x] updated `current-state.md` for the supported JSON number subset

Follow-up issues:

- [x] updated `issues/open/052-implement-json.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `2d5677c` (`issue-052b: support json noninteger numbers`)

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -E 'test(json)'
result: pass; 18 tests run, 18 passed, 399 skipped
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli json
result: pass; 15 tests run, 15 passed, 241 skipped
date: 2026-04-29

command: node fixtures/builtins-and-io/json-parse-unsupported-noninteger-number.ts
result: pass; printed 1.5
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-noninteger-number.ts -o /tmp/ts2wasm-json-noninteger-number.wasm && iwasm /tmp/ts2wasm-json-noninteger-number.wasm
result: pass; printed 1.5
date: 2026-04-29

command: node fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-array.ts && cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-array.ts -o /tmp/ts2wasm-json-noninteger-number-array.wasm && iwasm /tmp/ts2wasm-json-noninteger-number-array.wasm
result: pass; Node and iwasm both printed 1.5
date: 2026-04-29

command: node fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-object.ts && cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-noninteger-number-object.ts -o /tmp/ts2wasm-json-noninteger-number-object.wasm && iwasm /tmp/ts2wasm-json-noninteger-number-object.wasm
result: pass; Node and iwasm both printed 1.5
date: 2026-04-29

command: node fixtures/builtins-and-io/json-parse-number-decimal-exponent.ts && cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-number-decimal-exponent.ts -o /tmp/ts2wasm-json-decimal-exponent.wasm && iwasm /tmp/ts2wasm-json-decimal-exponent.wasm
result: pass; Node and iwasm both printed 1, 100, -25, 12
date: 2026-04-29

command: cargo nextest run
result: pass; 413 tests run, 413 passed, 4 skipped
date: 2026-04-29

command: mise run update-issue-index
result: pass; issues/index.md regenerated
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass; issues/index.md OK
date: 2026-04-29

command: mise run check issue-index
result: pass; issues/index.md queue OK and issue health OK after recreating gitignored local report placeholders referenced by pre-existing issue evidence
date: 2026-04-29

command: mise run check issues
result: pass; issue health OK after recreating gitignored local report placeholders referenced by pre-existing issue evidence
date: 2026-04-29

command: mise run check agent-state
result: pass; agent state files validated
date: 2026-04-29
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/052b-implement-json-noninteger-number-representation.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
