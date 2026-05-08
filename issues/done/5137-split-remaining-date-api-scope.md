---
id: 5137
title: "Split remaining Date API scope"
type: cleanup
area: runtime/builtins
class: design-ready
priority: P1
depends_on: []
blocks: [050]
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Split the remaining broad Date parent scope into executable child issues or close the parent with evidence if the remaining fixtures are already covered.

Problem: Date issue 050 currently stays open after its named child issues 240 and 241 are done, but non-literal constructor inputs and broader Date API fixtures are still only described on the parent.

## Problem

Date issue 050 currently has no open child issue for the unsupported remaining Date surface after issues 240 and 241 moved to done. The parent still mentions non-integer/non-literal constructor inputs and broader Date API behavior, which leaves implementation workers with a broad epic instead of a concrete work order.

## Current failure

Representative remaining Date fixtures exist outside the deterministic `getTime()`/`valueOf()`, live-time, timezone `toString()`, and Annex B child completions:

```sh
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-local-getters-unsupported.ts -o /tmp/ts2wasm-date-local-getters.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-to-iso-string.ts -o /tmp/ts2wasm-date-to-iso-string.wasm
```

Those commands identify the remaining Date API surface that must be split before issue 050 can be selected or closed.

## Desired final state

Issue 050 depends only on open, concrete Date child issues, or it is closed with completion evidence proving the remaining Date fixtures are covered by existing done issues.

## Scope

In scope:

- [x] Inventory remaining Date fixtures under `fixtures/builtins-and-io/`.
- [x] Map each remaining Date constructor/method gap to an existing done issue or a new child issue.
- [x] Update issue 050 dependencies and acceptance criteria to reference only open remaining child work.

Out of scope:

- Implementing Date runtime behavior.
- Changing Date host capability policy.
- Editing compiler/runtime code.

## Affected paths

Expected:

- `issues/done/050-implement-date.md`
- `issues/open/`
- `issues/index.md`
- `fixtures/builtins-and-io/`

Do not touch:

- `crates/`
- `docs/`

## Acceptance criteria

- [x] Remaining Date fixture inventory names each uncovered fixture and its assigned issue or done evidence.
- [x] Any new Date child issues include exact fixture, stdout/stderr, diagnostic, or pass evidence expectations.
- [x] Issue 050 no longer lists closed child issues as open blockers.
- [x] Issue index and issue health checks pass after the split.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
python scripts/manager.py update-issue-index
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] create only if the fixture inventory finds uncovered Date runtime work


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
