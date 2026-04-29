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
completed: 2026-04-29
status: done
---

Problem: Unknown-unsupported classification has reached zero in large test262 windows, but the parent spike has no fixed completion boundary.

## Summary

Convert the open-ended spike into a closeable fixed-window verification record and leave future coverage expansion to separate ramp issues.

## Scope

In scope:

- [x] State the exact suite/window contract that issue 060 closes against.
- [x] Preserve the TypeScript reference-root limitation as an explicit residual risk or follow-up.
- [x] Move issue 060 to done if the fixed-window contract passes.

Out of scope:

- [x] Further test262 limit increases.
- [x] Fixing missing external reference checkouts.
- [x] Implementing classified feature gaps.

## Affected paths

Expected:

- `issues/done/060-investigate-unknown-unsupported-cases.md`
- `issues/done/060a-close-unknown-unsupported-fixed-window-spike.md`
- `issues/index.md`
- `current-state.md`

Do not touch:

- `crates/frontend/src/`
- `scripts/lib/feature-labels.sh`

## Acceptance criteria

- [x] The close boundary is explicit: suite `test262`, limit `17000`, command `TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 17000 --detail`, expected `unknown-unsupported=0`.
- [x] Any incomplete tsc/tsgo external-reference condition is recorded as residual risk or follow-up.
- [x] Future unknown-unsupported work has a separate issue pattern: create a dedicated ramp issue with explicit suite, limit, command, and expected `unknown-unsupported` count.

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

Commits:

- `pending in child-060a close commit`

Validation result:

```text
command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 17000 --detail
result: pass; executed=17000; build_pass=5; semantic_pass=3; unsupported=16994; blocked=1; unknown-unsupported=0
date: 2026-04-29

command: mise run update-issue-index && mise run update-issue-index -- --check && mise run check issue-index && mise run check issues && mise run check-agent-state
result: partial; update-issue-index passed; update-issue-index -- --check passed; check-agent-state passed; check issue-index and check issues failed on pre-existing missing report paths in issue 052 and issue 228, outside child-060a allowed paths
date: 2026-04-29
```

Remaining risks:

- The assigned `/home/wogikaze/wgkz/ts2wasm/reference` root lacks `TypeScript`, so exact tsc validation from that root remains a residual external-reference risk. Existing issue-060 tsc evidence used `/tmp/ts2wasm-issue060-reference`.
- Future unknown-unsupported expansion should not reopen this parent spike; file fixed-window ramp issues instead.
