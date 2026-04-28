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

2026-04-28 broader classification slice:

- Expanded the stored coverage windows to test262 limit 300, tsc limit 150, and tsgo limit 100.
- Added path-based classifier labels for newly visible test262 Annex B families:
  - `html-comment` for `annexB/language/comments/` HTML-like comment syntax cases.
  - `eval` for `annexB/language/eval-code/` direct eval-code cases.
- Created follow-up feature issues:
  - issue 224: Annex B HTML-like comments (`html-comment`, 8 cases in the limit-300 test262 window).
  - issue 225: direct eval / Annex B function declaration semantics (`eval`, 51 cases in the limit-300 test262 window).
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/results/tsc.json`, `artifacts/coverage/results/tsgo.json`, and `artifacts/coverage/reference-coverage-matrix.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 300
result: pass; unsupported_features=name-resolution:93,string-builtin:63,eval:51,regexp-literal:47,legacy-global-builtin:20,date:16,html-comment:8,array-builtin:1,builtin-api:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsc --limit 150
result: blocked; assigned reference root is missing /home/wogikaze/wgkz/ts2wasm/reference/TypeScript, so the command fails before classification.

TS2WASM_REFERENCE_ROOT=/tmp/ts2wasm-issue060-reference scripts/manager reference-coverage tsc --limit 150
result: pass; unsupported_features=parser-syntax:50,ambient-declaration:25,type-alias:23,import-export:20,class-accessor:17,declaration-emit:3,scope-analysis:2,function:1,jsdoc:1,module-resolution:1,module-system-amd:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsgo --limit 100
result: pass; unsupported_features=import-export:20,declaration-emit:16,parser-syntax:13,jsx:8,type-system:7,class:6,module-resolution:4,decorator:3,enum:3,type-assertion:3,destructuring:2,jsdoc:2,object-literal:2,type-alias:2,class-accessor:1,module-system-amd:1,name-resolution:1,scope-analysis:1; unknown-unsupported=0
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for the exact assigned tsc command.

2026-04-28 child classification ramp slice:

- Expanded the stored coverage windows to test262 limit 500, tsc limit 200, and tsgo limit 120.
- The test262 and tsc expanded windows had zero `unknown-unsupported` without new labels.
- Added classifier labels for newly visible tsgo compiler cases:
  - `parameter-property` for constructor parameter properties with default values.
  - `type-directive-resolution` for triple-slash `reference types` directive diagnostic processing.
- Created follow-up feature issues:
  - issue 226: TypeScript parameter properties (`parameter-property`, 2 cases in the limit-120 tsgo window).
  - issue 227: type reference directive resolution (`type-directive-resolution`, 3 cases in the limit-120 tsgo window).
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/results/tsc.json`, `artifacts/coverage/results/tsgo.json`, and `artifacts/coverage/reference-coverage-matrix.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 500
result: pass; unsupported_features=eval:246,name-resolution:106,string-builtin:63,regexp-literal:47,legacy-global-builtin:20,date:16,array-builtin:1,builtin-api:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/tmp/ts2wasm-issue060-reference scripts/manager reference-coverage tsc --limit 200
result: pass; unsupported_features=parser-syntax:59,ambient-declaration:30,type-alias:23,import-export:21,class-accessor:17,arguments-object:10,module-system-amd:10,declaration-emit:8,class:3,scope-analysis:3,module-resolution:2,name-resolution:2,function:1,jsdoc:1,object-literal:1,type-assertion:1,type-system:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsgo --limit 120
result: pass; unsupported_features=import-export:20,parser-syntax:17,declaration-emit:16,module-resolution:10,jsx:8,class:7,type-system:7,decorator:4,enum:3,object-literal:3,type-assertion:3,type-directive-resolution:3,destructuring:2,jsdoc:2,parameter-property:2,type-alias:2,class-accessor:1,module-system-amd:1,name-resolution:1,scope-analysis:1; unknown-unsupported=0
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

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
