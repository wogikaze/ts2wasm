---
id: 1201
title: "Implement Classextendsinterfaceinexpression"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: [5257]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed by splitting the current object type literal construct-signature parser
blocker to `issues/open/5257-parse-object-type-literal-construct-signatures.md`.

## Problem

Reference test results showed 1 case in `classExtendsInterfaceInExpression`
with diagnostic `unknown-unsupported`.

Problem: fresh triage shows the current failure occurs in the function return
type annotation `{new(): Object}` before the later class heritage expression can
be parsed.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceInExpression.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterfaceInExpression.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing interface construct-signature issue is not the exact owner
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterfaceInExpression.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceInExpression.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5257-parse-object-type-literal-construct-signatures.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExtendsInterfaceInExpression.ts`

Source context:

```ts
interface A {}

function factory(a: any): {new(): Object} {
  return null;
}

class C extends factory(A) {}
```

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterfaceInExpression.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceInExpression.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 66, end: 67 } }) at 67..68
Tokens: ok
AST: fails while parsing return type annotation `{new(): Object}`
```

TypeScript AST accepts the annotation as `TypeLiteral -> ConstructSignature`.
The current parser blocker occurs before the later `class C extends factory(A)`
heritage expression can be triaged. If this parser slice advances to heritage
resolution, `issues/open/5252-support-call-expression-class-heritage.md` is the
likely existing owner for the `extends factory(A)` shape.

Split issue:

- `issues/open/5257-parse-object-type-literal-construct-signatures.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5257-parse-object-type-literal-construct-signatures.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceInExpression.ts
result: pass; current blocker split to issue 5257
date: 2026-05-06
```

Remaining risks:

- After 5257 advances past the object type literal construct signature,
  `classExtendsInterfaceInExpression.ts` may expose the later
  `extends factory(A)` class heritage call-expression blocker tracked by 5252.
