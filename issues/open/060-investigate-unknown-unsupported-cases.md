---
id: 060
title: "Investigate and classify unknown-unsupported diagnostic cases"
type: spike
area: frontend
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-28
---

## Summary

Investigate unknown-unsupported diagnostic cases to determine their root causes and classify them into appropriate feature issues.

Problem: The spike has progressed through large validated windows, but its completion condition is still open-ended; direct selection invites unbounded coverage ramping.

Queue design note:

- This is now a parent spike and must not be selected directly from the Ready queue.
- Close through issue 060a by fixing an explicit suite/window contract and recording any remaining out-of-scope reference roots.
- Future unknown-unsupported work should be new ramp issues with fixed suite, limit, and expected output contract.

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
mise run reference-coverage -- test262 --limit 200
mise run reference-coverage -- tsc --limit 100
mise run reference-coverage -- tsgo --limit 50
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
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 100
result: pass; unsupported_features=regexp-literal:47,name-resolution:33,date:16,string-builtin:3,array-builtin:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 200
result: pass; unsupported_features=name-resolution:76,string-builtin:60,regexp-literal:47,date:16,array-builtin:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/tmp/ts2wasm-issue060-reference mise run reference-coverage -- tsc --limit 100
result: pass; unsupported_features=parser-syntax:47,type-alias:23,class-accessor:17,import-export:3,declaration-emit:2,scope-analysis:2,jsdoc:1,module-system-amd:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- tsgo --limit 82
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
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 300
result: pass; unsupported_features=name-resolution:93,string-builtin:63,eval:51,regexp-literal:47,legacy-global-builtin:20,date:16,html-comment:8,array-builtin:1,builtin-api:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- tsc --limit 150
result: blocked; assigned reference root is missing /home/wogikaze/wgkz/ts2wasm/reference/TypeScript, so the command fails before classification.

TS2WASM_REFERENCE_ROOT=/tmp/ts2wasm-issue060-reference mise run reference-coverage -- tsc --limit 150
result: pass; unsupported_features=parser-syntax:50,ambient-declaration:25,type-alias:23,import-export:20,class-accessor:17,declaration-emit:3,scope-analysis:2,function:1,jsdoc:1,module-resolution:1,module-system-amd:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- tsgo --limit 100
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
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 500
result: pass; unsupported_features=eval:246,name-resolution:106,string-builtin:63,regexp-literal:47,legacy-global-builtin:20,date:16,array-builtin:1,builtin-api:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/tmp/ts2wasm-issue060-reference mise run reference-coverage -- tsc --limit 200
result: pass; unsupported_features=parser-syntax:59,ambient-declaration:30,type-alias:23,import-export:21,class-accessor:17,arguments-object:10,module-system-amd:10,declaration-emit:8,class:3,scope-analysis:3,module-resolution:2,name-resolution:2,function:1,jsdoc:1,object-literal:1,type-assertion:1,type-system:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- tsgo --limit 120
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
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 750 --detail
result before classifier update: pass; unsupported_features=eval:461,name-resolution:118,string-builtin:63,regexp-literal:47,legacy-global-builtin:20,parser-syntax:17,date:16,unknown-unsupported:4,builtin-api:1,function:1,object-literal:1

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 750 --detail
result after classifier update: pass; unsupported_features=eval:461,name-resolution:118,string-builtin:63,regexp-literal:47,legacy-global-builtin:20,date:16,parser-syntax:16,logical-assignment:3,legacy-octal-escape:2,array-builtin:1,builtin-api:1,function:1,object-literal:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 750 --json
result: pass; stored artifacts/coverage/results/test262.json with executed=750 and unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated

mise run update-issue-index
result: pass; issues/index.md updated

mise run check issues
result: pass

mise run check agent-state
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
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 1000 --detail
result: pass; unsupported_features=eval:461,parser-syntax:168,name-resolution:138,function:87,string-builtin:63,regexp-literal:46,date:16,legacy-global-builtin:16,arguments-object:1,builtin-api:1,object-literal:1,switch:1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter annexB/built-ins/Array/from/iterator-method-emulates-undefined.js --detail
result: pass; unsupported_features=array-builtin:1; blocked=0; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 1000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=1000, unsupported=1000, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
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
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 1250 --detail
result before classifier update: pass; unsupported_features=eval:461,name-resolution:207,parser-syntax:188,function:127,array-builtin:88,string-builtin:63,regexp-literal:53,date:16,legacy-global-builtin:16,builtin-api:14,declaration-emit:4,duplicate-local:4,destructuring:2,object-literal:2,arguments-object:1,class:1,switch:1,unknown-unsupported:1; blocked=1

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 1250 --detail
result after classifier update: pass; unsupported_features=eval:461,name-resolution:207,parser-syntax:188,function:127,array-builtin:89,string-builtin:63,regexp-literal:53,date:16,legacy-global-builtin:16,builtin-api:14,declaration-emit:4,duplicate-local:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=0; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 1250 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=1250, unsupported=1250, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
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
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 1500 --detail
result: pass; unsupported_features=eval:461,name-resolution:283,array-builtin:259,parser-syntax:188,function:127,string-builtin:63,regexp-literal:53,date:16,legacy-global-builtin:16,builtin-api:14,duplicate-local:7,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter annexB/built-ins/Array/from/iterator-method-emulates-undefined.js --detail
result: pass; unsupported_features=array-builtin:1; blocked=0; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 1500 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=1500, unsupported=1500, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
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
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 2000 --detail
result: pass; unsupported_features=array-builtin:598,eval:461,name-resolution:444,parser-syntax:188,function:127,string-builtin:63,regexp-literal:53,date:16,legacy-global-builtin:16,builtin-api:14,duplicate-local:7,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter annexB/built-ins/Array/from/iterator-method-emulates-undefined.js --detail
result: pass; unsupported_features=array-builtin:1; blocked=0; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 2000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=2000, unsupported=2000, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp2500 continuation:

- Expanded the stored test262 coverage window from limit 2000 to limit 2500.
- The limit-2500 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; a targeted rerun classified that case as `array-builtin`, and the stored JSON artifact has `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 2500 --detail
result: pass; unsupported_features=array-builtin:969,name-resolution:573,eval:461,parser-syntax:188,function:127,string-builtin:63,regexp-literal:53,date:16,legacy-global-builtin:16,builtin-api:14,duplicate-local:7,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter annexB/built-ins/Array/from/iterator-method-emulates-undefined.js --detail
result: pass; unsupported_features=array-builtin:1; blocked=0; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 2500 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=2500, unsupported=2500, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp3000 continuation:

- Expanded the stored test262 coverage window from limit 2500 to limit 3000.
- The limit-3000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 3000 --detail
result: pass; unsupported_features=array-builtin:1315,name-resolution:731,eval:461,parser-syntax:188,function:127,string-builtin:63,regexp-literal:51,legacy-global-builtin:16,builtin-api:14,date:13,duplicate-local:8,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 3000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=3000, unsupported=3000, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp3500 continuation:

- Expanded the stored test262 coverage window from limit 3000 to limit 3500.
- The limit-3500 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 3500 --detail
result: pass; unsupported_features=array-builtin:1708,name-resolution:835,eval:461,parser-syntax:188,function:127,string-builtin:63,regexp-literal:51,legacy-global-builtin:16,builtin-api:14,date:13,duplicate-local:11,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 3500 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=3500, unsupported=3500, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp4000 continuation:

- Expanded the stored test262 coverage window from limit 3500 to limit 4000.
- The limit-4000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 4000 --detail
result: pass; unsupported_features=array-builtin:2032,name-resolution:1001,eval:461,parser-syntax:188,function:127,string-builtin:63,regexp-literal:51,duplicate-local:21,legacy-global-builtin:16,builtin-api:14,date:13,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 4000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=4000, unsupported=4000, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp5000 continuation:

- Expanded the stored test262 coverage window from limit 4000 to limit 5000.
- The limit-5000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 5000 --detail
result: pass; unsupported_features=array-builtin:2166,name-resolution:1209,builtin-api:667,eval:461,parser-syntax:188,function:127,string-builtin:63,regexp-literal:51,duplicate-local:26,legacy-global-builtin:16,date:13,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 5000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=5000, unsupported=5000, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp6000 continuation:

- Expanded the stored test262 coverage window from limit 5000 to limit 6000.
- The limit-6000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 6000 --detail
result: pass; unsupported_features=array-builtin:2166,name-resolution:1533,builtin-api:1215,eval:461,parser-syntax:188,date:140,function:127,string-builtin:63,regexp-literal:51,duplicate-local:27,legacy-global-builtin:16,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 6000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=6000, unsupported=6000, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp7000 continuation:

- Expanded the stored test262 coverage window from limit 6000 to limit 7000.
- The limit-7000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported two blocked cases:
  - `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`
  - `annexB/built-ins/Date/prototype/getYear/B.2.4.js`
- The JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 7000 --detail
result: pass; unsupported_features=array-builtin:2166,name-resolution:1769,builtin-api:1315,function:508,eval:461,date:421,parser-syntax:188,string-builtin:63,regexp-literal:51,duplicate-local:28,legacy-global-builtin:16,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=2; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 7000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=7000, unsupported=7000, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp8000 continuation:

- Expanded the stored test262 coverage window from limit 7000 to limit 8000.
- The limit-8000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 8000 --detail
result: pass; unsupported_features=array-builtin:2166,builtin-api:2117,name-resolution:1933,function:542,eval:461,date:421,parser-syntax:188,string-builtin:63,regexp-literal:51,duplicate-local:28,legacy-global-builtin:16,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 8000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=8000, build_pass=1, unsupported=7999, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp9000 continuation:

- Expanded the stored test262 coverage window from limit 8000 to limit 9000.
- The limit-9000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.
- The newly visible `object-builtin` bucket is already classified by existing classifier rules.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 9000 --detail
result: pass; unsupported_features=name-resolution:2546,builtin-api:2399,array-builtin:2166,function:542,eval:461,date:421,parser-syntax:188,object-builtin:102,string-builtin:63,regexp-literal:51,duplicate-local:31,legacy-global-builtin:16,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 9000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=9000, build_pass=1, unsupported=8999, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp10000 continuation:

- Expanded the stored test262 coverage window from limit 9000 to limit 10000.
- The limit-10000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.
- The expanded `object-builtin` bucket remained classified by existing classifier rules.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 10000 --detail
result: pass; unsupported_features=name-resolution:2841,builtin-api:2399,array-builtin:2166,object-builtin:807,function:542,eval:461,date:421,parser-syntax:188,string-builtin:63,regexp-literal:51,duplicate-local:31,legacy-global-builtin:16,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 10000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=10000, build_pass=1, unsupported=9999, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp11000 continuation:

- Expanded the stored test262 coverage window from limit 10000 to limit 11000.
- The limit-11000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.
- The expanded `object-builtin` bucket remained classified by existing classifier rules.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 11000 --detail
result: pass; unsupported_features=name-resolution:3058,builtin-api:2399,array-builtin:2166,object-builtin:1590,function:542,eval:461,date:421,parser-syntax:188,string-builtin:63,regexp-literal:51,duplicate-local:31,legacy-global-builtin:16,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 11000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=11000, build_pass=1, unsupported=10999, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp12000 continuation:

- Expanded the stored test262 coverage window from limit 11000 to limit 12000.
- The limit-12000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.
- The expanded `object-builtin`, `array-builtin`, and `builtin-api` buckets remained classified by existing classifier rules.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 12000 --detail
result: pass; unsupported_features=name-resolution:3677,builtin-api:2399,array-builtin:2166,object-builtin:1968,function:542,eval:461,date:421,parser-syntax:188,string-builtin:63,regexp-literal:51,duplicate-local:31,legacy-global-builtin:16,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 12000 --json > artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=12000, build_pass=4, semantic_pass=3, unsupported=11996, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp13000 continuation:

- Expanded the stored test262 coverage window from limit 12000 to limit 13000.
- The limit-13000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.
- The newly visible `function-resolution` bucket was already classified by existing classifier rules.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 13000 --detail
result: pass; unsupported_features=name-resolution:3842,builtin-api:3138,array-builtin:2166,object-builtin:2063,function:542,eval:461,date:421,parser-syntax:188,string-builtin:63,regexp-literal:51,duplicate-local:31,legacy-global-builtin:16,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,function-resolution:1,switch:1; blocked=1; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 13000 --json > temp && mv temp artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=13000, build_pass=4, semantic_pass=3, unsupported=12996, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated

mise run update-coverage-matrix -- --check
result: pass; coverage matrix OK

mise run check issues
result: pass

mise run check agent-state
result: pass
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp14000 continuation:

- Expanded the stored test262 coverage window from limit 13000 to limit 14000.
- The limit-14000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.
- The expanded `regexp-literal`, `builtin-api`, `name-resolution`, and `array-builtin` buckets remained classified by existing classifier rules.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 14000 --detail
result: pass; unsupported_features=name-resolution:4140,builtin-api:3375,array-builtin:2166,object-builtin:2063,function:542,regexp-literal:506,eval:461,date:421,parser-syntax:188,string-builtin:63,duplicate-local:41,legacy-global-builtin:16,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,function-resolution:1,switch:1; blocked=1; unknown-unsupported=0

tmp=$(mktemp artifacts/coverage/results/test262.json.tmp.XXXXXX); TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 14000 --json > "$tmp"; mv "$tmp" artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=14000, build_pass=4, semantic_pass=3, unsupported=13996, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated

mise run update-coverage-matrix -- --check
result: pass; coverage matrix OK
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp15000 continuation:

- Expanded the stored test262 coverage window from limit 14000 to limit 15000.
- The limit-15000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.
- The expanded `regexp-literal` bucket remained classified by existing classifier rules.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 15000 --detail
result: pass; unsupported_features=name-resolution:4339,builtin-api:3375,array-builtin:2166,object-builtin:2063,regexp-literal:1307,function:542,eval:461,date:421,parser-syntax:188,string-builtin:63,duplicate-local:41,legacy-global-builtin:16,declaration-emit:4,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,class:1,function-resolution:1,switch:1; blocked=1; unknown-unsupported=0

tmp=$(mktemp artifacts/coverage/results/test262.json.tmp.XXXXXX); TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 15000 --json > "$tmp"; mv "$tmp" artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=15000, build_pass=4, semantic_pass=3, unsupported=14996, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp16000 continuation:

- Expanded the stored test262 coverage window from limit 15000 to limit 16000.
- The first limit-16000 detail run surfaced 8 `unknown-unsupported` entries under Annex B emulates-undefined equality/logical/typeof/if cases:
  - `annexB/language/expressions/does-not-equals/emulates-undefined.js`
  - `annexB/language/expressions/equals/emulates-undefined.js`
  - `annexB/language/expressions/logical-and/emulates-undefined.js`
  - `annexB/language/expressions/logical-not/emulates-undefined.js`
  - `annexB/language/expressions/strict-does-not-equals/emulates-undefined.js`
  - `annexB/language/expressions/strict-equals/emulates-undefined.js`
  - `annexB/language/expressions/typeof/emulates-undefined.js`
  - `annexB/language/statements/if/emulated-undefined.js`
- Added the `annexb-ishtmldda` classifier label for those path families. These cases map to existing issue 237, so no new follow-up issue was required.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 16000 --detail
result before classifier update: pass; unsupported_features=name-resolution:4614,builtin-api:3799,array-builtin:2166,object-builtin:2064,regexp-literal:1497,function:542,eval:461,date:421,parser-syntax:188,string-builtin:159,duplicate-local:42,legacy-global-builtin:16,unknown-unsupported:8,declaration-emit:4,logical-assignment:3,class:2,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,function-resolution:1,switch:1; blocked=1

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter annexB/language/expressions/equals/emulates-undefined.js --path-filter annexB/language/statements/if/emulated-undefined.js --detail
result: pass; unsupported_features=annexb-ishtmldda:2; blocked=0; unknown-unsupported=0

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 16000 --detail
result after classifier update: pass; unsupported_features=name-resolution:4614,builtin-api:3799,array-builtin:2167,object-builtin:2064,regexp-literal:1497,function:542,eval:461,date:421,parser-syntax:187,string-builtin:159,duplicate-local:42,legacy-global-builtin:16,annexb-ishtmldda:9,declaration-emit:4,logical-assignment:3,class:2,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,function-resolution:1,switch:1; blocked=0; unknown-unsupported=0

tmp=$(mktemp artifacts/coverage/results/test262.json.tmp.XXXXXX); TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 16000 --json > "$tmp"; mv "$tmp" artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=16000, build_pass=5, semantic_pass=3, unsupported=15995, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp17000 continuation:

- Expanded the stored test262 coverage window from limit 16000 to limit 17000.
- The limit-17000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported two blocked cases, including the known transient `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js` timeout family; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.
- The expanded window remained covered by existing labels, with newly larger buckets mainly in `name-resolution`, `string-builtin`, `function`, `annexb-ishtmldda`, and `duplicate-local`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 17000 --detail
result: pass; unsupported_features=name-resolution:5224,builtin-api:3740,array-builtin:2121,object-builtin:2058,regexp-literal:1476,string-builtin:698,function:595,eval:460,date:405,parser-syntax:131,duplicate-local:45,legacy-global-builtin:16,annexb-ishtmldda:12,declaration-emit:4,class:2,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,function-resolution:1,switch:1; blocked=2; unknown-unsupported=0

tmp=$(mktemp artifacts/coverage/results/test262.json.tmp.XXXXXX); TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 17000 --json > "$tmp"; mv "$tmp" artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=17000, build_pass=5, semantic_pass=3, unsupported=16995, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
result: pass; artifacts/coverage/reference-coverage-matrix.md updated
```

This remains validated PROGRESS, not DONE: full acceptance still requires exhausting broader reference windows, and the assigned `/home/wogikaze/wgkz/ts2wasm/reference` root currently lacks the TypeScript checkout needed for tsc validation from that exact root.

2026-04-28 child coverage ramp18000 continuation:

- Expanded the stored test262 coverage window from limit 17000 to limit 18000.
- The limit-18000 window had zero `unknown-unsupported` entries; no classifier changes or new follow-up issues were required for this slice.
- The detail run reported the known transient blocked timeout for `annexB/built-ins/Array/from/iterator-method-emulates-undefined.js`; the JSON artifact rerun completed with `blocked=0`.
- Refreshed `artifacts/coverage/results/test262.json`, `artifacts/coverage/reference-coverage-matrix.md`, and `current-state.md`.
- The expanded window remained covered by existing labels, with the newly larger bucket mainly in `builtin-api`.

Validated classification commands:

```text
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 18000 --detail
result: pass; unsupported_features=name-resolution:5513,builtin-api:4434,array-builtin:2120,object-builtin:2058,regexp-literal:1476,string-builtin:715,function:595,eval:460,date:405,parser-syntax:131,duplicate-local:45,legacy-global-builtin:16,annexb-ishtmldda:12,declaration-emit:4,class:2,destructuring:2,object-literal:2,arguments-object:1,async-iteration:1,function-resolution:1,switch:1; blocked=1; unknown-unsupported=0

tmp=$(mktemp artifacts/coverage/results/test262.json.tmp.XXXXXX); TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 18000 --json > "$tmp"; mv "$tmp" artifacts/coverage/results/test262.json
result: pass; stored artifacts/coverage/results/test262.json with executed=18000, build_pass=5, semantic_pass=3, unsupported=17995, blocked=0, unknown-unsupported=0

mise run update-coverage-matrix
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
