---
id: 3438
title: "Implement Narrowbyclauseexpressioninswitchtrue Name Resolution"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
---

## Summary

Closed after splitting/folding both fresh failing paths:

- `narrowByClauseExpressionInSwitchTrue1.ts` split to
  `issues/open/5444-resolve-const-arrow-predicate-calls-in-switch-true.md`.
- `narrowByClauseExpressionInSwitchTrue4.ts` folded into existing
  `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`.

## Problem

Reference test results show 2 cases fail in directory
`narrowByClauseExpressionInSwitchTrue-name-resolution` with diagnostics:
name-resolution. The compiler cannot handle these syntax/semantics,
preventing compilation of code in this category.

Problem: narrowByClauseExpressionInSwitchTrue-name-resolution had 2 generated
reference failures and needed smart-triage evidence before implementation
starts.

Disposition: one implementation-ready child issue was created for the const
arrow predicate call blocker; the ambient `declare const` blocker is covered by
existing issue 5161.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue4.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue4.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue4.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue4.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5444-resolve-const-arrow-predicate-calls-in-switch-true.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue4.ts`
- `reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue1.ts`

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, title overlap)
- `issues/open/437-implement-name-resolution.md` - Implement name resolution (same feature label, title overlap)
- `issues/open/648-implement-argumentsAsPropertyName-name-resolution.md` - Implement Argumentsaspropertyname Name Resolution (same feature label, title overlap)
- `issues/open/654-implement-argumentsReferenceInConstructor-name-resolution.md` - Implement Argumentsreferenceinconstructor Name Resolution (same feature label, title overlap)
- `issues/open/657-implement-argumentsReferenceInMethod-name-resolution.md` - Implement Argumentsreferenceinmethod Name Resolution (same feature label, title overlap)
- `issues/open/693-implement-arrayToLocaleStringES-name-resolution.md` - Implement Arraytolocalestringes Name Resolution (same feature label, title overlap)
- `issues/open/733-implement-assignmentCompatability-name-resolution.md` - Implement Assignmentcompatability Name Resolution (same feature label, title overlap)
- `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`
  owns the `narrowByClauseExpressionInSwitchTrue4.ts` ambient
  `declare const f` name-resolution blocker.
- No existing open implementation-ready issue owned the
  `narrowByClauseExpressionInSwitchTrue1.ts` const arrow predicate call
  blocker, so it was split to issue 5444.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue1.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedFunction:1
unsupported_features=function-resolution:1
semantic_enabled=0

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue4.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
semantic_enabled=0
```

Fresh triage for `narrowByClauseExpressionInSwitchTrue1.ts`:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue1.ts

result:
UnresolvedFunction: unresolved function: `isA`
```

Source context:

```ts
const isA = (x: AorB): x is A => x.type === "A";
const isB = (x: AorB): x is B => x.type === "B";

function test1(x: AorB) {
  switch (true) {
    case isA(x):
      x;
      break;
    case isB(x):
      x;
      break;
  }
}
```

Compiler evidence:

```text
tokens: ok through const arrow predicate bindings and switch case calls
ast: ok; `case isA(x)` and `case isB(x)` are Call expressions
resolved/lowered: fails with UnresolvedFunction for isA
visible symbols before failure: isA, isB, test1, test2, x, isSomeType, processInput
TypeScript oracle: ok, diagnostics=[]
```

Fresh triage for `narrowByClauseExpressionInSwitchTrue4.ts`:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue4.ts

result:
UnresolvedName: unresolved name: `f` at 112..113
```

Source context:

```ts
declare const f: 'a' | 'b' | 'c';

switch (true) {
  case f === "a":
  default:
    f;
  case f === "b":
    f;
}
```

Compiler evidence:

```text
tokens: ok; includes declare const f and all switch clauses
ast: ok; ambient declaration is erased, switch remains
visible symbols before failure: binding f
resolved: fails in resolve_names with UnresolvedName for f in first case expression
TypeScript oracle: ok, diagnostics=[]
```

## Completion evidence

Closed after splitting issue 5444 and folding the ambient declaration case into
issue 5161.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue1.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedFunction:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue1.ts
result: pass; split to issue 5444 for UnresolvedFunction isA
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue4.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue4.ts
result: pass; folded into issue 5161 for ambient declare const f
date: 2026-05-08
```

Remaining risks:

- After issues 5444 and 5161 advance these paths, later TypeScript narrowing
  semantics for `switch (true)` clauses may need a focused semantic issue.
