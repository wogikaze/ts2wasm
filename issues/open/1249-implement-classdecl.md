---
id: 1249
title: "Implement Classdecl"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1249.

## Summary

Closed after splitting the current blocker to
`issues/open/5270-parse-modified-class-accessor-declarations.md`. Fresh triage
shows the first failure is a parser gap for `public static get p2()`.

## Problem

Reference test results show 1 case failing in directory `classdecl`. Fresh
triage confirms the current blocker is the modified static getter declaration
`public static get p2()`.

Problem: `classdecl.ts` reports `expected LeftParen, got Some(Ident("get"))`
at `public static get p2()`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classdecl.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classdecl.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing broad class-accessor issue is not an exact implementation-ready child for this modifier parser gap
- [x] Split one observable behavior into child issue 5270
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

- [x] Duplicate candidates below are confirmed and the exact behavior is split to 5270
- [x] Child issue 5270 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classdecl.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classdecl.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5270-parse-modified-class-accessor-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classdecl.ts`

Source context:

```ts
class a {
    public static get p2() {
        return { x: 30, y: 40 };
    }
}
```

## Duplicate detection

- `issues/open/072-implement-ClassDeclaration.md` - Implement Classdeclaration (same feature label, same group key, title overlap)
- `issues/open/424-implement-declaration-emit.md` - Implement declaration-emit support (same feature label, same group key, title overlap)
- `issues/open/773-implement-autoAsiForStaticsInClassDeclaration.md` - Implement Autoasiforstaticsinclassdeclaration (same feature label, same group key, title overlap)
- `issues/open/422-implement-class-accessor.md` is related but broad; no
  implementation-ready child owned `public static get p2()` after modifiers.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classdecl.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classdecl.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: expected LeftParen, got Some(Ident("get")) at 348..351
Source: public static get p2() { ... }
tokens: ok
AST: fails before modified getter construction
resolved: same parser failure
TypeScript oracle: ok, diagnostics=[]
Child issue: 5270
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5270-parse-modified-class-accessor-declarations.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classdecl.ts
result: pass; reproduced modified static getter parser failure and split child issue 5270
date: 2026-05-06
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5270
