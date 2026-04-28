---
id: 060a
title: "Close unknown-unsupported fixed-window spike"
type: spike
area: frontend
class: verification-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: Unknown-unsupported classification has reached zero in large test262 windows, but the parent spike has no fixed completion boundary.

## Summary

Convert the open-ended spike into a closeable fixed-window verification record and leave future coverage expansion to separate ramp issues.

## Scope

In scope:

- [ ] State the exact suite/window contract that issue 060 closes against.
- [ ] Preserve the TypeScript reference-root limitation as an explicit residual risk or follow-up.
- [ ] Move issue 060 to done if the fixed-window contract passes.

Out of scope:

- Further test262 limit increases.
- Fixing missing external reference checkouts.
- Implementing classified feature gaps.

## Affected paths

Expected:

- `issues/open/060-investigate-unknown-unsupported-cases.md`
- `issues/done/`
- `issues/index.md`
- `current-state.md`

Do not touch:

- `crates/frontend/src/`
- `scripts/lib/feature-labels.sh`

## Acceptance criteria

- [ ] The close boundary is explicit: suite, limit, command, and expected unknown-unsupported count.
- [ ] Any incomplete tsc/tsgo external-reference condition is recorded as residual risk or follow-up.
- [ ] Future unknown-unsupported work has a separate issue pattern.

## Validation

Required commands:

```sh
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issue-index
mise run check issues
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 17000 --detail
```

Not run:

- none

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
