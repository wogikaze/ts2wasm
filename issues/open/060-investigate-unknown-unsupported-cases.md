---
id: 060
title: "Investigate and classify unknown-unsupported diagnostic cases"
type: spike
area: frontend
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-28
---

## Summary

Investigate unknown-unsupported diagnostic cases to determine their root causes and classify them into appropriate feature issues.

## Problem

Reference test results show 48 cases with unknown-unsupported diagnostic (test262:5, tsc:20, tsgo:23). These cases are not classified into specific feature categories, making it unclear what implementation work is needed.

## Desired final state

All unknown-unsupported cases are investigated and classified into specific feature categories or diagnostic codes. Unknown-unsupported diagnostic is only used for genuinely unclassifiable cases.

## Scope

In scope:

- [ ] Investigate each unknown-unsupported case
- [ ] Determine root cause (parser, runtime, type system, etc.)
- [ ] Classify into appropriate feature categories
- [ ] Update feature-labels.sh with new categories if needed
- [ ] Create or update feature issues for classified gaps

Out of scope:

- [ ] Implementing the features (separate issues)

## Affected paths

Expected:

- `scripts/lib/feature-labels.sh`
- `issues/open/`

Do not touch:

- `crates/frontend/src/`
- `crates/cli/src/`

## Acceptance criteria

- [ ] All unknown-unsupported cases are classified
- [ ] Feature-labels.sh updated with new categories if needed
- [ ] Appropriate feature issues created or updated
- [ ] unknown-unsupported count reduced to 0 or only genuinely unclassifiable cases

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/manager reference-coverage test262 --limit 200
scripts/manager reference-coverage tsc --limit 100
scripts/manager reference-coverage tsgo --limit 50
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] Feature issues based on classification results

## Notes

This is a spike to understand the unknown cases before implementation.

## Progress evidence

2026-04-28 classification slice:

- Updated `scripts/lib/feature-labels.sh` and `scripts/run/reference-coverage.py` with path-based labels for currently visible unknown families: `array-builtin`, `string-builtin`, `legacy-global-builtin`, `builtin-api`, `declaration-emit`, `class-accessor`, `type-alias`, `ambient-declaration`, `module-system-amd`, `module-resolution`, `enum`, `decorator`, `type-assertion`, `type-system`, `scope-analysis`, `arguments-object`, `object-literal`, `jsx`, and `jsdoc`.
- Added `TS2WASM_REFERENCE_ROOT` support to `scripts/run/reference-coverage.py` so isolated worktrees can validate against external reference checkouts without adding reference sources to the branch.
- Refreshed `artifacts/coverage/results/test262.json` and `artifacts/coverage/reference-coverage-matrix.md`; the stored test262 limit-100 row now has zero `unknown-unsupported` entries and classifies the prior unknowns as `string-builtin` / `array-builtin`.
- Added tsc and tsgo coverage result artifacts for the validated classification windows.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 100
result: pass; unsupported_features=regexp-literal:47,name-resolution:33,date:16,string-builtin:3,array-builtin:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 200
result: pass; unsupported_features=name-resolution:76,string-builtin:60,regexp-literal:47,date:16,array-builtin:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/tmp/ts2wasm-issue060-reference scripts/manager reference-coverage tsc --limit 100
result: pass; unsupported_features=parser-syntax:47,type-alias:23,class-accessor:17,import-export:3,declaration-emit:2,scope-analysis:2,jsdoc:1,module-system-amd:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsgo --limit 82
result: pass; unsupported_features=import-export:18,declaration-emit:16,parser-syntax:10,class:6,type-system:6,jsx:3,module-resolution:3,type-assertion:3,decorator:2,destructuring:2,jsdoc:2,object-literal:2,type-alias:2,enum:1,module-system-amd:1,scope-analysis:1; unknown-unsupported=0
```

This is validated PROGRESS, not DONE: full acceptance still requires exhausting all unknown-unsupported cases across the broader reference coverage, not only the currently validated windows.

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
