---
id: 405
title: "Support Test262 harness instanceof function RHS"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: [207]
blocks: [338]
created: 2026-05-01
updated: 2026-05-01
status: done
completed: 2026-05-01
---

## Summary

Support or explicitly shim the Test262 harness `instanceof` pattern that uses a
function-valued right-hand side rather than a lowered class constructor.

Problem: the sparse Array.map representative now advances past mutable callback
capture, but stops in the harness at an `instanceof` RHS diagnostic.

## Problem

Issue 207 completed ordinary prototype-chain `instanceof` for supported class
constructors. The Test262 assertion harness still contains function-constructor
RHS forms that are not represented as supported class constructors in the
current frontend/runtime subset.

## Current failure

```sh
mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js
```

Current result after issue 404:

```text
[UnsupportedSyntax] issue-207: instanceof right-hand side must be a supported class constructor `Array`
```

## Desired final state

The selected Test262 sparse Array.map representative no longer stops at the
harness `instanceof` function-RHS boundary. It either compiles the required
ordinary-function constructor `instanceof` subset or the harness shim removes the
unsupported construct while preserving assertion semantics.

## Scope

In scope:

- [x] Support the minimal `instanceof` RHS form used by the Test262 harness for
      the selected representative, or replace it in the wasm-side harness shim
      with equivalent supported assertion logic.
- [x] Keep issue 207 ordinary class-constructor semantics intact.
- [x] Rerun the selected Array.map representative and update issue 338 with the
      next concrete blocker or pass evidence.

Out of scope:

- Custom `Symbol.hasInstance`.
- Broad JavaScript function constructor/prototype semantics beyond the harness
  slice.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `scripts/lib/test262_harness.py`
- `issues/open/338-array-map-sparse-array-holes.md`

Do not touch:

- unrelated runtime builtin families

## Acceptance criteria

- [x] The selected representative no longer reports the issue-207 harness
      `instanceof` RHS diagnostic.
- [x] A focused regression or harness test covers the supported path.
- [x] Issue 338 is updated with the next concrete Test262 blocker or pass
      evidence.
- [x] `cargo fmt --all --check` and `mise run check issues` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli array_map
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected unless broad `instanceof` semantics change

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Prefer a harness shim if the failing construct is only used to implement Test262
assertion helpers; prefer compiler/runtime support if an ordinary source fixture
needs the same behavior.

## Completion evidence

Fill only when moving to `done`.

Commits:

- `pending child-405 retry commit`

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-01

command: mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js
result: pass; reports BuildPass / pass and no issue-207 `instanceof Array` blocker
date: 2026-05-01

command: cargo nextest run -p ts2wasm-cli array_map
result: pass
date: 2026-05-01

command: mise run update-issue-index -- --check
result: pass
date: 2026-05-01

command: mise run check issues
result: pass
date: 2026-05-01
```

Remaining risks:

- Parent should use the committed hash from the final child report; this file
  was filled before the validation commit hash existed.
