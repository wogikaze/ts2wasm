---
id: 1179
title: "Implement Classdeclaredbeforeclassfactory"
type: spike
area: frontend/syntax
class: blocked
priority: P2
depends_on: [5252]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1179.

## Summary

Triage classDeclaredBeforeClassFactory across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `classDeclaredBeforeClassFactory` with diagnostics: declaration-emit. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: classDeclaredBeforeClassFactory has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classDeclaredBeforeClassFactory.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classDeclaredBeforeClassFactory.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classDeclaredBeforeClassFactory.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classDeclaredBeforeClassFactory.ts
```

Not run:

- cargo gates; issue split only, no implementation changes

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5252-support-call-expression-class-heritage.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classDeclaredBeforeClassFactory.ts`

## Duplicate detection

- `issues/open/1741-implement-declarationEmitExpressionInExtends-declaration-emit.md` is related and covers other expression-in-extends declaration-emit cases.
- `issues/open/5225-w0-typed-wat-writer.md` is related but limited to qualified/member heritage names such as `extends Foo.Object`.
- No existing implementation-ready issue covered the call-expression heritage factory case exactly.

## Smart triage

Fresh triage on 2026-05-06:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclaredBeforeClassFactory.ts
```

Result:

```text
Smart triage class: triage-needed
Feature label: declaration-emit
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Current error: only simple inheritance (extends ClassName) is supported
```

Compiler dump evidence:

```text
tokens: ok
ast: ok; ClassDecl Derived extends Call(callee=Ident("makeBaseClass"), args=[])
resolved: stops in builtin_resolver with only simple inheritance diagnostic
```

TypeScript oracle:

```text
ok; diagnostics=[]
hint: makeBaseClass has type typeof Base
```

The generated bucket is closed because the executable slice is the narrower
resolver/lowering support for call-expression class heritage, captured in issue
5252.

## Completion evidence

Commits:

- local split commit for issue 1179 / child 5252

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classDeclaredBeforeClassFactory.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_features=declaration-emit:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclaredBeforeClassFactory.ts
result: pass; current blocker is resolver/builtin UnsupportedSyntax for call-expression heritage
date: 2026-05-06
```

Remaining risks:

- Issue 5252 removes the resolver/lowering blocker. Broader declaration emit expression-in-extends buckets may still require separate import/export or `.d.ts` work.
