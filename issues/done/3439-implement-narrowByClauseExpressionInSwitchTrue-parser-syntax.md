---
id: 3439
title: "Implement Narrowbyclauseexpressioninswitchtrue Parser Syntax"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
---

## Summary

Closed after splitting/folding all three fresh failing paths:

- `narrowByClauseExpressionInSwitchTrue2.ts` folded into
  `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`.
- `narrowByClauseExpressionInSwitchTrue6.ts` split to
  `issues/open/5445-parse-braced-switch-case-clause-statements.md`.
- `narrowByClauseExpressionInSwitchTrue7.ts` folded into
  `issues/open/5192-support-first-class-class-constructor-values.md`.

## Problem

Reference test results showed 3 cases fail in directory
`narrowByClauseExpressionInSwitchTrue-parser-syntax` with diagnostics:
parser-syntax. Fresh evidence now shows one ambient name-resolution blocker,
one braced case-clause parser blocker, and one class-value blocker.

Problem: narrowByClauseExpressionInSwitchTrue-parser-syntax had 3 generated
reference failures and needed smart-triage evidence before implementation
starts.

Disposition: one implementation-ready child issue was created for the remaining
parser blocker; the other two paths are covered by existing implementation
issues.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue2.ts --detail
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
mise run reference-coverage -- tsc --limit 6
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue2.ts
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

- [x] `issues/open/5445-parse-braced-switch-case-clause-statements.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue2.ts`
- `reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue6.ts`
- `reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue7.ts`

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/open/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/663-implement-arrayAssignmentTest-parser-syntax.md` - Implement Arrayassignmenttest Parser Syntax (same feature label, title overlap)
- `issues/open/734-implement-assignmentCompatability-parser-syntax.md` - Implement Assignmentcompatability Parser Syntax (same feature label, title overlap)
- `issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md` - Implement Asyncfunctionreturntype Parser Syntax (same feature label, title overlap)
- `issues/open/767-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/open/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)
- `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`
  owns the `narrowByClauseExpressionInSwitchTrue2.ts` ambient `declare const f`
  name-resolution blocker.
- `issues/open/5192-support-first-class-class-constructor-values.md` owns the
  `narrowByClauseExpressionInSwitchTrue7.ts` `issue-5011` class constructor
  value blocker at `base instanceof Derived1`.
- No existing open implementation-ready issue owned the braced switch case
  clause parser blocker in `narrowByClauseExpressionInSwitchTrue6.ts`, so it
  was split to issue 5445.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue2.ts --detail --no-dashboard-data
result: executed=1, unsupported=1, unsupported_diagcodes=UnresolvedName:1, unsupported_features=name-resolution:1

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue6.ts --detail --no-dashboard-data
result: executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue7.ts --detail --no-dashboard-data
result: executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
```

Fresh triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue2.ts
result: UnresolvedName: unresolved name: `f` at 170..171

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue6.ts
result: UnsupportedSyntax: expected Comma, got Some(Dot) at 670..671

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue7.ts
result: UnsupportedSyntax: issue-5011: class `Derived1` cannot be used as a value - class runtime is not yet supported at 337..345
```

Representative source contexts:

```ts
// narrowByClauseExpressionInSwitchTrue2.ts
declare const f: 'a' | 'b' | 'c';
switch(true) {
    case f === 'a':
    case f === 'b':
        f;
        break
}
```

```ts
// narrowByClauseExpressionInSwitchTrue6.ts
case x.kind === "a": {
    x.aProps;
    break;
}
```

```ts
// narrowByClauseExpressionInSwitchTrue7.ts
case base instanceof Derived1:
    base.d
```

Compiler evidence:

```text
narrowByClauseExpressionInSwitchTrue2.ts: tokens and AST ok; ambient declaration is erased; visible symbols include f; resolve_names reports UnresolvedName for f.
narrowByClauseExpressionInSwitchTrue6.ts: tokens ok; AST fails while parsing x.aProps inside a braced case body.
narrowByClauseExpressionInSwitchTrue7.ts: tokens and AST ok; resolve_names reports issue-5011 for class value Derived1 in instanceof.
```

## Completion evidence

Closed after splitting issue 5445 and folding the ambient/class-value cases
into issues 5161 and 5192.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue2.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue2.ts
result: pass; folded into issue 5161 for ambient declare const f
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue6.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue6.ts
result: pass; split to issue 5445 for braced switch case clause statements
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue7.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue7.ts
result: pass; folded into issue 5192 for issue-5011 class constructor values
date: 2026-05-08
```

Remaining risks:

- After issues 5161, 5192, and 5445 advance these paths, later
  `switch (true)` narrowing semantics may need focused semantic follow-ups.
