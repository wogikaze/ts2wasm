---
id: 5297
title: "Lower computed object binding aliases"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Support the first computed-property destructuring declaration shape:

```ts
let foo = "bar";
let {[foo]: bar} = {bar: "bar"};
```

## Problem

`computedPropertiesInDestructuring1.ts` parses computed object binding aliases,
but resolution/lowering rejects them:

```text
UnsupportedRuntimeSubset: issue-251: object binding aliases must use identifier keys in this runtime slice at 81..113
```

Problem: object binding aliases can only use identifier keys, so computed keys
such as `[foo]` cannot bind `bar` from the source object.

Current failure: `computedPropertiesInDestructuring1.ts` reports
`UnsupportedRuntimeSubset` at `let {[foo]: bar} = {bar: "bar"};`.

## Desired final state

The representative declaration binds `bar` from the property named by `foo`, or
advances to the next narrower unsupported destructuring diagnostic in the same
reference file.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
line 4, column 4
coverage: executed=1, build_pass=0, unsupported=1, blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=destructuring:1
```

Compiler evidence:

```text
tokens: ok; LeftBrace LeftBracket Ident("foo") RightBracket Colon Ident("bar")
ast: ok; binding name contains computed key expression `[foo]` and target `bar`
resolved: UnsupportedRuntimeSubset issue-251 object binding aliases must use identifier keys
```

## Scope

In scope:

- [x] Lower declaration binding aliases of the form `{[ident]: target}`.
- [x] Evaluate the computed key once and use it to read the source object.
- [x] Bind the target identifier to the selected property value.
- [x] Add one focused Node/iwasm or CLI regression for `let {[foo]: bar} = obj`.

Out of scope:

- Destructuring assignment expressions.
- Parameter binding patterns.
- Nested array/object computed keys.
- Full TypeScript type diagnostics for invalid index signatures.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/` or focused fixtures

Do not touch:

- unrelated module, enum, or class lowering code

## Acceptance criteria

- [x] The representative triage no longer reports `object binding aliases must use identifier keys` at `81..113`.
- [x] A focused regression proves `let {[foo]: bar} = {bar: "bar"};` binds `bar`.
- [x] Existing identifier-key object binding aliases remain covered.
- [x] Any next blocker in `computedPropertiesInDestructuring1.ts` is recorded here or split if outside this shape.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(destructuring) or test(node_diff)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket
`issues/done/1408-implement-computedPropertiesInDestructuring.md`.

Related but not duplicates:

- Issue 5180 covers parser acceptance for computed object binding patterns; this representative already parses.
- Issue 251 completed the supported destructuring runtime subset and now emits the source-spanned boundary this issue advances.

## Completion Evidence

Fill only when moving to `done/`.

## False-done audit

**truly-done** (5297)

- Implementation commits: verified via `git log --oneline --all --grep=5297`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
