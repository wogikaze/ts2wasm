---
id: 1217
title: "Implement Classimplementsimportedinterface"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5262]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1217.

## Summary

Closed by splitting the current namespace import-equals alias resolver blocker
to
`issues/open/5262-resolve-import-equals-aliases-in-class-implements-clauses.md`.

## Problem

Reference test results showed 1 case failing in directory
`classImplementsImportedInterface` with diagnostics: import-export. Fresh triage
shows the current blocker is name resolution for an import-equals alias in a
class `implements` clause.

Problem: `namespace M2 { import T = M1.I; class C implements T { ... } }`
reports `UnresolvedName` at `T`, while TypeScript accepts the type-only alias.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsImportedInterface.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classImplementsImportedInterface.ts --detail --no-dashboard-data
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classImplementsImportedInterface.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classImplementsImportedInterface.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5262-resolve-import-equals-aliases-in-class-implements-clauses.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classImplementsImportedInterface.ts`

Source context:

```ts
namespace M1 {
    export interface I {
        foo();
    }
}

namespace M2 {
    import T = M1.I;
    class C implements T {
        foo() {}
    }
}
```

## Duplicate detection

- `issues/open/432-implement-import-export.md` is a broad umbrella for module
  syntax and not an exact implementation-ready alias-in-heritage slice.
- Existing alias/import-equals buckets cover other reference paths and do not
  own `classImplementsImportedInterface.ts` or `implements T`.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classImplementsImportedInterface.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsImportedInterface.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnresolvedName:1
unsupported_features: name-resolution:1

Diagnostic: UnresolvedName
Message: unresolved name: `T` at 170..171
Source: class C implements T
tokens: ok; namespace, export interface, import T = M1.I, implements T
AST/resolved: fail at `T`
TypeScript oracle: ok, diagnostics=[]
TypeScript AST path: ModuleDeclaration -> ModuleBlock -> ClassDeclaration ->
  HeritageClause -> ExpressionWithTypeArguments -> Identifier
```

Split issue:

- `issues/open/5262-resolve-import-equals-aliases-in-class-implements-clauses.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5262-resolve-import-equals-aliases-in-class-implements-clauses.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsImportedInterface.ts
result: pass; current blocker split to issue 5262
date: 2026-05-06
```

Remaining risks:

- none
