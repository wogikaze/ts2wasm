---
id: 404
title: "Support mutable outer local captures in callback functions"
type: feature
area: ir/runtime
class: done
priority: P2
depends_on: []
blocks: [338]
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Implement the narrow mutable outer local capture support needed by known
callback functions used in runtime builtin calls such as `Array.prototype.map`.
The immediate blocker is a Test262 sparse-map representative whose callback
mutates the top-level `callCnt` binding.

## Problem

Problem: a function callback that mutates an outer local currently loses the
binding during resolution/lowering and reports raw `UnresolvedName` for the
outer local instead of compiling the supported callback subset or producing an
issue-linked unsupported diagnostic.

## Current failure

```sh
mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js
```

Current result after issue 338 increment-expression progress:

```text
UnresolvedName: unresolved name: `callCnt`
```

Visible-symbol triage shows:

```text
binding callCnt = 0
function callbackfn(val, idx, obj) { callCnt++; ... }
binding resArr = srcArr.map(callbackfn)
```

## Desired final state

Known callback functions can safely read and mutate supported outer locals in
the runtime subset used by `Array.prototype.map`, or the compiler emits a
specific issue-linked unsupported diagnostic for forms outside that subset.

## Scope

In scope:

- [x] Resolve outer locals referenced from known callback functions instead of
      falling through to raw `UnresolvedName`.
- [x] Preserve mutation semantics for the narrow top-level local counter pattern
      used by the sparse-map Test262 representative.
- [x] Add a focused core-semantics fixture with a callback mutating an outer
      counter.
- [x] Rerun the issue 338 Test262 sparse-map representative.

Out of scope:

- Escaping closures beyond already tracked closure issues.
- Arbitrary heap closure dispatch.
- Broader direct `eval`, `Function`, generator, async, or class-method capture
  semantics.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`

Do not touch:

- unrelated builtin families
- unrelated issue files except blockers/index sync

## Acceptance criteria

- [x] A focused fixture like `array-map-callback-mutates-outer-counter.ts`
      matches Node output under `iwasm`.
- [x] The selected representative
      `reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js`
      no longer reports raw `UnresolvedName: callCnt`.
- [x] Unsupported forms outside the implemented subset produce a specific
      issue-linked diagnostic rather than generic `UnresolvedName`.
- [x] Issue 338 is unblocked or updated with the next concrete Test262 blocker.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli array_map
mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo test -p ts2wasm-cli array_map -- --nocapture
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected unless closure/cell ABI changes

Current state:

- [x] not affected

Follow-up issues:

- [x] issue 405 tracks the next selected Test262 representative blocker

## Notes

Issue 292 intentionally closed the earlier `initCount` raw unresolved-name
bucket by adding a narrower diagnostic for a different class/destructuring
representative. This issue tracks the remaining runtime implementation needed
for ordinary callback functions in the supported builtin-call subset.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `0ceae4bb` `Support map callback mutable outer captures`

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-01

command: cargo nextest run -p ts2wasm-cli array_map
result: pass (17/17)
date: 2026-05-01

command: mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js
result: no raw `UnresolvedName: callCnt`; next blocker is `[UnsupportedSyntax] issue-207: instanceof right-hand side must be a supported class constructor `Array``
date: 2026-05-01

command: mise run check issues
result: pass
date: 2026-05-01
```

Remaining risks:

- none

## Progress evidence

2026-05-01 child-404-callback-captures-20260430T231258Z:

- Implemented a narrow lowering slice for named `Array.prototype.map` callbacks:
  top-level locals captured by selected callback functions are passed as hidden
  capture parameters, and mutable captures are lowered through existing env
  cells so callback writes update the outer binding.
- Added focused Node/iwasm regression fixture
  `fixtures/core-semantics/array-map-callback-mutates-outer-counter.ts`.
- Validation passed:
  `cargo test -p ts2wasm-cli array_map_callback_mutates_outer_counter_fixture_matches_node_output_under_iwasm -- --nocapture`.
- Validation passed:
  `cargo test -p ts2wasm-cli array_map -- --nocapture` (17/17 array-map
  tests passed).
- Selected Test262 representative rerun:
  `mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js`
  no longer reports raw `UnresolvedName: callCnt` and advances to
  `[UnsupportedSyntax] issue-207: instanceof right-hand side must be a
  supported class constructor \`Array\`` from the Test262 assertion harness.
- Issue 404 is closed. Issue 405 tracks the next harness `instanceof` RHS
  blocker before issue 338 can close with selected Test262 representative
  evidence.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/404-mutable-outer-local-callback-captures.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
