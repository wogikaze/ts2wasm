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

2026-04-28 child coverage ramp continuation:

- Expanded the stored test262 coverage window from limit 500 to limit 750.
- The first limit-750 detail run surfaced 4 `unknown-unsupported` entries:
  - 2 logical assignment cases under `annexB/language/expressions/logical-assignment/`.
  - 2 template literal legacy octal escape cases under `annexB/language/expressions/template-literal/`.
- Added classifier labels:
  - `logical-assignment` for Annex B logical assignment operator cases.
  - `legacy-octal-escape` for strict/non-strict legacy octal escape cases in template literals.
- Created follow-up feature issues:
  - issue 228: logical assignment operators (`logical-assignment`, 3 cases in the limit-750 test262 window).
  - issue 229: legacy octal escape handling (`legacy-octal-escape`, 2 cases in the limit-750 test262 window).
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `issues/index.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 750 --detail
result before classifier update: pass; unsupported_features=eval:461,name-resolution:118,string-builtin:63,regexp-literal:47,legacy-global-builtin:20,parser-syntax:17,date:16,unknown-unsupported:4,builtin-api:1,function:1,object-literal:1

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 750 --detail
result after classifier update: pass; unsupported_features=eval:461,name-resolution:118,string-builtin:63,regexp-literal:47,legacy-global-builtin:20,date:16,parser-syntax:16,logical-assignment:3,legacy-octal-escape:2,array-builtin:1,builtin-api:1,function:1,object-literal:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 750 --json
result: pass; stored artifacts/coverage/results/test262.json with executed=750 and unknown-unsupported=0

scripts/manager update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated

scripts/manager update-issue-index
result: pass; issues/index.md updated

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp2 continuation:

- Expanded the stored test262 coverage window from limit 750 to limit 1000.
- The limit-1000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- Refreshed `artifacts/coverage/results/test262.json` and `artifacts/coverage/reference-coverage-matrix.md`.
- A first detail run reported one transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; a targeted rerun classified that case as `array-builtin`, and the stored JSON artifact has `blocked=0`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 1000 --detail
result: pass; unsupported_features=eval:461,parser-syntax:168,name-resolution:138,function:87,string-builtin:63,regexp-literal:46,date:16,legacy-global-builtin:16,arguments-object:1,builtin-api:1,object-literal:1,switch:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --path-filter annexB/built-ins/Array/from/iterator-method-emulates-undefined.js --detail
result: pass; unsupported_features=array-builtin:1; blocked=0; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 1000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=1000, unsupported=1000, blocked=0, unknown-unsupported=0

scripts/manager update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp3 continuation:

- Expanded the stored test262 coverage window from limit 1000 to limit 1250.
- The first limit-1250 detail run surfaced 1 `unknown-unsupported` entry:
  - `annexB/language/statements/for-await-of/iterator-close-return-emulates-undefined-throws-when-called.js`.
- The reference file metadata names `async-iteration`, and the source uses `for await (var x of iter)` with `Symbol.asyncIterator` and Annex B `IsHTMLDDA` behavior.
- Added classifier labels for `for-await-of` paths:
  - `async-iteration` for async iteration / `for await...of` cases.
- Created follow-up feature issue:
  - issue 230: async iteration and `for await...of` (`async-iteration`, 1 case in the limit-1250 test262 window).
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `issues/index.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 1250 --detail
result before classifier update: pass; unsupported_features=eval:461,name-resolution:207,parser-syntax:188,function:127,array-builtin:88,string-builtin:63,regexp-literal:53,date:16,legacy-global-builtin:16,builtin-api:14,declaration-emit:4,duplicate-local:4,destructuring:2,object-literal:2,arguments-object:1,class:1,switch:1,unknown-unsupported:1; blocked=1

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 1250 --detail
result after classifier update: pass; unsupported_features=eval:461,name-resolution:207,parser-syntax:188,function:127,array-builtin:89,string-builtin:63,regexp-literal:53,date:16,legacy-global-builtin:16,builtin-api:14,declaration-emit:4,duplicate-local:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=0; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 1250 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=1250, unsupported=1250, blocked=0, unknown-unsupported=0

scripts/manager update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp1500 continuation:

- Expanded the stored test262 coverage window from limit 1250 to limit 1500.
- The limit-1500 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run initially reported one blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; a targeted rerun classified that case as `array-builtin`, and the stored JSON artifact has `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json` and `artifacts/coverage/reference-coverage-matrix.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 1500 --detail
result: pass; unsupported_features=eval:461,name-resolution:283,array-builtin:259,parser-syntax:188,function:127,string-builtin:63,regexp-literal:53,date:16,legacy-global-builtin:16,builtin-api:14,duplicate-local:7,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --path-filter annexB/built-ins/Array/from/iterator-method-emulates-undefined.js --detail
result: pass; unsupported_features=array-builtin:1; blocked=0; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 1500 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=1500, unsupported=1500, blocked=0, unknown-unsupported=0

scripts/manager update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp2000 continuation:

- Expanded the stored test262 coverage window from limit 1500 to limit 2000.
- The limit-2000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run initially reported one blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; a targeted rerun classified that case as `array-builtin`, and the stored JSON artifact has `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json` and `artifacts/coverage/reference-coverage-matrix.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 2000 --detail
result: pass; unsupported_features=array-builtin:598,eval:461,name-resolution:444,parser-syntax:188,function:127,string-builtin:63,regexp-literal:53,date:16,legacy-global-builtin:16,builtin-api:14,duplicate-local:7,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --path-filter annexB/built-ins/Array/from/iterator-method-emulates-undefined.js --detail
result: pass; unsupported_features=array-builtin:1; blocked=0; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 2000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=2000, unsupported=2000, blocked=0, unknown-unsupported=0

scripts/manager update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
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
