---
id: 050
title: "Implement Date"
type: feature
area: runtime/builtins
class: blocked
priority: P1
depends_on: [5137]
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Implement Date object for date/time operations. The current supported subset covers deterministic integer-literal epoch-millisecond Date values plus live epoch-millisecond time for `Date.now()` and no-argument `new Date()` through the WASI realtime clock capability.

Problem: Date support is currently tracked as a broad epic; direct selection leaves live-time policy, frontend recognition, runtime helpers, and Annex B behavior mixed in one work item.

Queue design note:

- This is an epic-level issue and must not be selected directly from the Ready queue.
- Use child slices for deterministic Date behavior, host time policy, timezone formatting policy, and Annex B legacy methods.
- Current child slices:
  - issue 239: live-time capability policy for `new Date()` / `Date.now()` (done)
  - issue 242: implement live-time Date entry points after issue 239 (done)
  - issue 240: timezone-aware `Date.prototype.toString()` policy/implementation (done)
  - issue 241: Annex B legacy `getYear` / `setYear` / `toGMTString` diagnostics (done)
  - issue 5137: split remaining non-literal constructor and broader Date API scope

## Problem

Full Date is currently unsupported outside the implemented deterministic epoch-millisecond Date slices and WASI-backed live host time entry points. Timezone formatting, non-integer/non-literal inputs, and broader Date API behavior remain separate policy-backed child work.

## Current failure

Representative remaining Date gaps are still tracked through child issues rather than this parent:

```sh
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-epoch-get-time.ts -o /tmp/ts2wasm-date-epoch-get-time.wasm
iwasm /tmp/ts2wasm-date-epoch-get-time.wasm
```

That deterministic epoch fixture passes today, so it is not the blocker. The parent remains open because timezone-aware formatting, non-integer/non-literal Date inputs, and Annex B legacy methods (`getYear`, `setYear`, `toGMTString`) are intentionally split into child work. Live-time `Date.now()` and no-argument `new Date()` are already implemented by issue 242 and should not be reworked from this parent.

## Desired final state

`new Date()` and Date methods work for basic operations, with deterministic epoch behavior, live-time capability policy, timezone formatting policy, and Annex B legacy behavior completed through child issues before this epic closes.

## Scope

In scope:

- [ ] Implement Date constructor
- [x] Implement Date.now()
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

- [ ] Deterministic `new Date(<epoch-ms integer>)` fixtures still match Node/iwasm stdout for `fixtures/builtins-and-io/date-epoch-get-time.ts` and `fixtures/builtins-and-io/date-epoch-value-of.ts`.
- [ ] Live-time `Date.now()` and no-argument `new Date()` fixtures still emit the audited `wasi.clock.realtime` manifest reason and `wasi_snapshot_preview1.clock_time_get` import.
- [ ] Issue 5137 closes with a concrete remaining-Date API issue split or evidence that no open Date child work remains.

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

2026-04-29 live-time policy note:

- Issue 239 selected WASI Preview 1 realtime clock as the default live-time capability
  for `Date.now()` and no-argument `new Date()`.
- Future implementation must emit `wasi.clock.realtime: true`,
  `capability_reasons["wasi.clock.realtime"]` with `Date.now` or `new Date()`,
  and the matching `wasi_snapshot_preview1.clock_time_get` import.
- The existing unsupported diagnostics for `Date.now()` and no-argument `new Date()`
  were superseded by issue 242, which consumes that policy and adds manifest/import and
  host-deny coverage.

2026-04-29 live-time implementation note:

- Issue 242 implemented `Date.now()` and no-argument `new Date()` with WASI Preview 1
  `clock_time_get`.
- Live-time Date fixtures now assert the returned epoch millisecond value falls within the
  host execution clock window instead of comparing exact Node timestamps.
- Manifest and host-deny coverage verifies `wasi.clock.realtime: true`,
  `capability_reasons["wasi.clock.realtime"]` with `Date.now` or `new Date()`, and the
  matching `wasi_snapshot_preview1.clock_time_get` import.
- Deterministic `new Date(<epoch-ms integer>)` fixtures remain free of
  `wasi.clock.realtime` and `clock_time_get`.
- Remaining issue 050 scope stays open: timezone formatting, non-integer/non-literal Date
  inputs, and broader Date API behavior are not complete.

2026-04-29 Annex B legacy method note:

- Issue 241 owns the preserved Annex B `Date.prototype.getYear`, `setYear`, and
  `toGMTString` reference cases from the superseded issue 061 evidence.
- These legacy methods now produce stable `issue-241` unsupported diagnostics when used
  on deterministic Date receivers. This keeps the deterministic Date subset explicit
  while the broad issue 050 epic remains open for live time, timezone formatting,
  non-literal inputs, and full Date API behavior.

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
