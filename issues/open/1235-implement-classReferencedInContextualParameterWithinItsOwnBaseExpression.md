---
id: 1235
title: "Implement Classreferencedincontextualparameterwithinitsownbaseexpression"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1235.

## Summary

Triage classReferencedInContextualParameterWithinItsOwnBaseExpression across 1
failing reference test case and close it as superseded by existing export-class
and call-expression heritage issues.

## Problem

Reference test results previously showed 1 case failing in directory
`classReferencedInContextualParameterWithinItsOwnBaseExpression` with
diagnostics: import-export. Fresh triage shows the current first blocker is
entry-module `export class`; the resolver dump also identifies an existing
call-expression heritage boundary.

Problem: `export class A extends Class<A>("A")(...)` stops before this generated
bucket can become an implementation issue. Existing issues already own the
current first blocker and the next heritage-expression boundary.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classReferencedInContextualParameterWithinItsOwnBaseExpression.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classReferencedInContextualParameterWithinItsOwnBaseExpression.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5232 covers the current first blocker
- [x] Confirm existing issue 5252 covers the next call-expression heritage boundary
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
- [x] Superseding issues contain implementation-ready scopes for current and next boundaries
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classReferencedInContextualParameterWithinItsOwnBaseExpression.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classReferencedInContextualParameterWithinItsOwnBaseExpression.ts
```

Not run:

- `cargo fmt --all --check`; issue close only, no Rust code changed
- `cargo nextest run`; issue close only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`
- [x] related next boundary: `issues/open/5252-support-call-expression-class-heritage.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classReferencedInContextualParameterWithinItsOwnBaseExpression.ts`

## Duplicate detection

- `issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md` - exact current first blocker for entry-module `export class`
- `issues/open/5252-support-call-expression-class-heritage.md` - exact next boundary for `extends Class<A>("A")(...)` after export class support

## Smart triage

Fresh triage shows this generated import/export bucket is currently blocked by
entry-module export class support, with a known follow-on call-expression
heritage boundary.

### Smart triage: classReferencedInContextualParameterWithinItsOwnBaseExpression

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Current compiler message: `issue-5005: entry module export A uses a declaration form outside the current static export slice`
- Path: `reference/typescript/tests/cases/compiler/classReferencedInContextualParameterWithinItsOwnBaseExpression.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classReferencedInContextualParameterWithinItsOwnBaseExpression.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classReferencedInContextualParameterWithinItsOwnBaseExpression.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
declare const Class: <Self>(identifier: string) => <Fields>(
  fields: Fields,
  annotations?: Schema<Self>,
) => Class<OutputFrom<Fields>>;

export class A extends Class<A>("A")(
  { a: string },
  { pretty: (a) => JSON.stringify(a) },
) {}
```

Compiler evidence:

```text
tokens: ok
ast: ok; ExportDecl(ClassDecl A) with extends Call(Call(Ident Class, "A"), object args)
resolved dump: UnsupportedSyntax only simple inheritance (extends ClassName) is supported
top-level stack: UnsupportedModule issue-5005 entry-module export class
```

TypeScript oracle evidence:

```text
typescript ok: true
diagnostics: []
AST topLevel includes exported ClassDeclaration A with call-expression heritage.
```

Superseding owners:

- `issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`
- `issues/open/5252-support-call-expression-class-heritage.md`

## Completion evidence

Commits:

- Superseded by existing issues 5232 and 5252; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classReferencedInContextualParameterWithinItsOwnBaseExpression.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; current issue-5005 export class boundary superseded by 5232, next call-expression heritage boundary covered by 5252
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classReferencedInContextualParameterWithinItsOwnBaseExpression.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; unsupported=1
date: 2026-05-07
```

Remaining risks:

- After issues 5232 and 5252 land, this reference may expose contextual
  parameter inference or declaration-emit parity as a later blocker.
