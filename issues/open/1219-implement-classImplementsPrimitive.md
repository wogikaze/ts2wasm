---
id: 1219
title: "Implement Classimplementsprimitive"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5263]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1219.

## Summary

Closed by splitting the current class expression `implements` primitive parser
blocker to
`issues/open/5263-report-primitive-implements-on-class-expressions.md`.

## Problem

Reference test results showed 1 case failing in directory
`classImplementsPrimitive` with diagnostics: parser-syntax. Fresh triage shows
the current blocker is an anonymous class expression with an `implements`
primitive clause.

Problem: `const C4 = class implements number {}` is misparsed as a class named
`implements`, then fails with `expected LeftBrace` at `number` instead of a
source-spanned TypeScript primitive-implements diagnostic.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsPrimitive.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classImplementsPrimitive.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing open/done issues do not cover the exact current boundary
- [x] Split one observable behavior into an implementation-ready child issue
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

- [x] Duplicate candidates below are confirmed as no-match for the exact current boundary
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classImplementsPrimitive.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classImplementsPrimitive.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5263-report-primitive-implements-on-class-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classImplementsPrimitive.ts`

Source context:

```ts
class C implements number { }
class C2 implements string { }
class C3 implements boolean { }

const C4 = class implements number {}
const C5 = class implements string {}
const C6 = class implements boolean {}

const C7 = class A implements number { }
const C8 = class B implements string { }
const C9 = class C implements boolean { }
```

## Duplicate detection

- Broad parser-syntax buckets are not exact matches because this blocker is the
  specific class-expression `implements` clause misparse at `class implements
  number`.
- No existing open/done issue owns `classImplementsPrimitive.ts` or this exact
  `expected LeftBrace` diagnostic.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classImplementsPrimitive.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsPrimitive.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: expected LeftBrace, got Some(Ident("number")) at 188..194
Source: const C4 = class implements number {}
tokens: ok
visible symbols: class C/C2/C3, binding C4, class name "implements"
AST/resolved: fail at `number`
TypeScript oracle: TS2864 at each primitive implements type
TypeScript AST path: FirstStatement -> VariableDeclaration -> ClassExpression
  -> HeritageClause -> ExpressionWithTypeArguments
```

Split issue:

- `issues/open/5263-report-primitive-implements-on-class-expressions.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5263-report-primitive-implements-on-class-expressions.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsPrimitive.ts
result: pass; current blocker split to issue 5263
date: 2026-05-06
```

Remaining risks:

- none
