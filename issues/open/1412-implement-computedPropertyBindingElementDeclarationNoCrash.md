---
id: 1412
title: "Implement Computedpropertybindingelementdeclarationnocrash"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [432]
blocks: [5298]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1412.

## Summary

Triage computedPropertyBindingElementDeclarationNoCrash across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case fails in directory `computedPropertyBindingElementDeclarationNoCrash`. Fresh triage on 2026-05-07 showed this generated bucket is an array-binding-pattern `for-of` head parser gap, not the original placeholder import/export classification.

Problem: computedPropertyBindingElementDeclarationNoCrash has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/computedPropertyBindingElementDeclarationNoCrash1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertyBindingElementDeclarationNoCrash1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertyBindingElementDeclarationNoCrash1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/computedPropertyBindingElementDeclarationNoCrash1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/computedPropertyBindingElementDeclarationNoCrash1.ts`

## Duplicate detection

- No matching open issue was found for `for (const [key, value] of Object.entries(e))`.
- Issues 247, 251, and 252 are related destructuring work, but their completed scope explicitly excludes `for-in` / `for-of` destructuring heads.
- Issue 342 is related only after parsing succeeds because it covers `Object.entries`.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: unknown-unsupported
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Failure: const declarations require an initializer at 265..277
line 16, column 16
Source context:
13 | export class Test {
14 |   setState(state: State) {}
15 |   test = (e: any) => {
16 |     for (const [key, value] of Object.entries(e)) {
17 |       this.setState({
18 |         [key]: value,
19 |       });
Visible symbols before failure:
class Test
```

Compiler evidence:

```text
tokens: ok; For Const LeftBracket Ident("key") Comma Ident("value") RightBracket Of Object.entries(e)
ast/resolved: UnsupportedSyntax const declarations require an initializer at 265..277
TypeScript AST path: ClassDeclaration -> PropertyDeclaration -> ArrowFunction -> Block -> ForOfStatement -> VariableDeclarationList -> VariableDeclaration -> ArrayBindingPattern
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertyBindingElementDeclarationNoCrash1.ts --detail --no-dashboard-data
result: executed=1, build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5298-parse-for-of-array-binding-pattern-heads.md`.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- Child issue 5298 still needs implementation.
