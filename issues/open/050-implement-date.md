---
id: 050
title: "Implement Date"
type: feature
area: runtime/builtins
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement Date object for date/time operations.

Problem: Date support is currently tracked as a broad epic; direct selection leaves live-time policy, frontend recognition, runtime helpers, and Annex B behavior mixed in one work item.

Queue design note:

- This is an epic-level issue and must not be selected directly from the Ready queue.
- Use child slices for deterministic Date behavior, host time policy, timezone formatting policy, and Annex B legacy methods.
- Keep live host time work out of implementation slices until a capability policy child is complete.

## Problem

Date is not implemented. It is a common built-in for date/time operations.

## Desired final state

`new Date()` and Date methods work for basic operations.

## Scope

In scope:

- [ ] Implement Date constructor
- [ ] Implement Date.now()
- [ ] Implement Date.prototype.getTime
- [ ] Implement Date.prototype.toString
- [ ] Add fixtures for Date behavior

Out of scope:

- Full Date API (start with basic methods)

## Affected paths

Expected:

- `crates/backend-wasm/src/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] Date constructor works correctly
- [ ] Basic Date methods work correctly
- [ ] Fixtures cover Date behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/date-test.ts -o /tmp/test.wasm
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

2026-04-29 superseded-reference merge note:

- Duplicate issue 061 has been closed as superseded by this Date epic.
- Its reference affected-test evidence remains preserved here for Date child-slice planning:
  Annex B `Date.prototype.getYear`, `setYear`, and `toGMTString` cases, including
  NaN return behavior, not-a-constructor checks, receiver validation, and setYear
  valid/invalid date-value handling.
- Keep those legacy Annex B methods as child Date work rather than a competing parent
  issue. The existing deterministic Date slices below remain the implemented evidence;
  this merge note is issue-queue deduplication only.

2026-04-28 progress evidence:

- Implemented deterministic `new Date(<epoch-ms integer>)` lowering for integer epoch
  literals, including unary-negative integer literals, without adding host time imports.
- Implemented `Date.prototype.getTime()` lowering for Date receivers.
- Added fixture `fixtures/builtins-and-io/date-epoch-get-time.ts` covering `0`, `1`, and `-1`.
- Node/iwasm differential evidence for the fixture:

  ```text
  command: node fixtures/builtins-and-io/date-epoch-get-time.ts
  result: exit 0
  stdout:
  0
  1
  -1

  command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-epoch-get-time.ts -o /tmp/ts2wasm-date-epoch-get-time.wasm
  result: exit 0

  command: iwasm /tmp/ts2wasm-date-epoch-get-time.wasm
  result: exit 0
  stdout:
  0
  1
  -1
  ```

- Targeted regression:

  ```text
  command: cargo nextest run -p ts2wasm-cli date_epoch_get_time_fixture_matches_node_output_under_iwasm
  result: pass
  ```

- Remaining issue 050 scope stays open: `Date.now()`, no-argument `new Date()`,
  `toString`, non-integer/non-literal Date inputs, and full Date API behavior are not
  complete. No live host time import was added.

2026-04-28 diagnostic progress evidence:

- Replaced generic `Date.now()` lowering failure with an issue-linked diagnostic:

  ```text
  command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-now-live-time-unsupported.ts -o /tmp/ts2wasm-date-now-live-time-unsupported.wasm
  result: exit 1
  stderr: error: [UnsupportedSyntax] issue-050: Date.now() requires live host time; define an auditable time capability policy before enabling it. Use new Date(<epoch-ms integer>) for deterministic Date values at 12..22
  ```

- Refined no-argument `new Date()` to explain the same live host time capability-policy blocker:

  ```text
  command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-noarg-live-time-unsupported.ts -o /tmp/ts2wasm-date-noarg-live-time-unsupported.wasm
  result: exit 1
  stderr: error: [UnsupportedSyntax] issue-050: new Date() requires live host time; define an auditable time capability policy before enabling it. Use new Date(<epoch-ms integer>) for deterministic Date values at 12..22
  ```

- Added focused unsupported fixtures for those live-time entry points:
  `fixtures/builtins-and-io/date-now-live-time-unsupported.ts` and
  `fixtures/builtins-and-io/date-noarg-live-time-unsupported.ts`.
- Targeted regression:

  ```text
  command: cargo nextest run -p ts2wasm-cli date_live_time_fixtures_report_capability_policy_diagnostic date_epoch_get_time_fixture_matches_node_output_under_iwasm
  result: pass
  ```

- Remaining issue 050 scope stays open: full live time support, an auditable time
  capability policy, `toString`, non-integer/non-literal Date inputs, and full Date API
  behavior are not complete. No live host time import was added.

2026-04-28 valueOf progress evidence:

- Implemented deterministic `Date.prototype.valueOf()` for `new Date(<epoch-ms integer>)`
  receivers by reusing the existing Date epoch representation and `DateGetTime`
  runtime helper. No live host time import was added.
- Added fixture `fixtures/builtins-and-io/date-epoch-value-of.ts` covering `0`,
  positive, and negative integer epochs.
- Node/iwasm differential evidence for the fixture:

  ```text
  command: node fixtures/builtins-and-io/date-epoch-value-of.ts
  result: exit 0
  stdout:
  0
  123456
  -123456

  command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-epoch-value-of.ts -o /tmp/ts2wasm-date-epoch-value-of.wasm
  result: exit 0

  command: iwasm /tmp/ts2wasm-date-epoch-value-of.wasm
  result: exit 0
  stdout:
  0
  123456
  -123456
  ```

- Targeted regression:

  ```text
  command: cargo nextest run -p ts2wasm-cli date
  result: pass, 5 tests run
  ```

- Remaining issue 050 scope stays open: full live time support, an auditable time
  capability policy, `toString`, non-integer/non-literal Date inputs, and full Date API
  behavior are not complete. No live host time import was added.

2026-04-28 toString diagnostic progress evidence:

- Added a precise issue-050 diagnostic for `Date.prototype.toString()` on deterministic
  Date receivers instead of falling through to generic method/class receiver errors.
  The diagnostic records the timezone/host formatting-policy blocker and points users
  at `getTime()` / `valueOf()` for deterministic epoch milliseconds.
- Added fixture `fixtures/builtins-and-io/date-to-string-timezone-unsupported.ts`.
- Direct build evidence:

  ```text
  command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-to-string-timezone-unsupported.ts -o /tmp/ts2wasm-date-to-string-timezone.wasm
  result: exit 1
  stderr: error: [UnsupportedSyntax] issue-050: Date.prototype.toString() requires timezone/host formatting policy; use getTime() or valueOf() for deterministic epoch milliseconds at 12..34
  ```

- Targeted regression:

  ```text
  command: cargo nextest run -p ts2wasm-cli date
  result: pass, 7 tests run

  command: cargo nextest run -E 'test(date)'
  result: pass, 10 tests run
  ```

- Remaining issue 050 scope stays open: full live time support, an auditable time
  capability policy, actual timezone-aware `toString` formatting, non-integer/non-literal
  Date inputs, and full Date API behavior are not complete. No live host time import was added.

2026-04-28 blocker evidence:

- `new Date(0)` currently reaches class-constructor lowering and fails before backend Date runtime code can be used:

  ```text
  command: cargo run -q -p ts2wasm-cli -- build /tmp/ts2wasm-date-ZvvJxH.ts -o /tmp/ts2wasm-date-test.wasm
  result: exit 1
  stderr: error: [UnsupportedSyntax] issue-207: instanceof right-hand side must be a supported class constructor `Date`
  ```

- `Date.now()` currently fails in name/lowering before backend emission:

  ```text
  command: cargo run -q -p ts2wasm-cli -- build /tmp/ts2wasm-date-now-jjxJat.ts -o /tmp/ts2wasm-date-now-test.wasm
  result: exit 1
  stderr: error: [UnresolvedName] unresolved name: `Date`
  ```

- The required recognition/lowering changes live in `crates/ir/src/name_resolver.rs` and `crates/ir/src/lowered.rs`, which are outside the child assignment's allowed files. Completing `Date.now()` or zero-argument `new Date()` also requires an auditable time capability policy; the assignment explicitly forbids inventing untracked host time imports.

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
