---
id: 1202
title: "Implement Classextendsinterfaceinmodule"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1202.

## Summary

Triage classExtendsInterfaceInModule across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `classExtendsInterfaceInModule` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: classExtendsInterfaceInModule has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceInModule.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterfaceInModule.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5156
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] This issue contains an exact `reference-triage` command
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterfaceInModule.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceInModule.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by existing `issues/open/5156-parse-generic-type-arguments-in-class-heritage.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExtendsInterfaceInModule.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-07.

Fresh triage shows the first blocker is not module loading. The parser advances
past `namespace M`, `class C1 extends M.I1 {}`, and into the generic qualified
heritage clause `class C2<T> extends M.I2<T> {}`. It then consumes the
type-argument tokens as expression syntax and reaches the next namespace while
still expecting a class body.

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceInModule.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterfaceInModule.ts --detail --no-dashboard-data
```

Observed coverage:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

Failure:

```text
UnsupportedSyntax expected LeftBrace, got Some(Ident("namespace")) at 145..154
```

Source context:

```ts
namespace M {
  export interface I1 {}
  export interface I2<T> {}
}
class C1 extends M.I1 {}
class C2<T> extends M.I2<T> {}

namespace Mod {
    export namespace Nested {
        export interface I {}
    }
}
```

Compiler evidence:

```text
tokens: ok; includes class C2<T> extends M.I2<T> {}, then namespace Mod
ast/resolved: fail with expected LeftBrace at the following namespace
TypeScript oracle: parses the file and reports TS2689 for M.I1, M.I2, and Mod.Nested.I
```

Resolution:

```text
Issue 5156 owns erasing/skipping TypeScript type arguments in class heritage clauses before class body parsing. This reference adds the qualified generic heritage shape `extends M.I2<T>` to that same parser contract. Later TS2689 class-extends-interface diagnostics are tracked by issue 5315 after parser support advances.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceInModule.ts
result: pass; current blocker is generic type arguments in qualified class heritage, superseded by issue 5156
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterfaceInModule.ts --detail --no-dashboard-data
result: pass; unsupported=1, UnsupportedSyntax/import-export for the same parser boundary
date: 2026-05-07
```

Remaining risks:

- After issue 5156 advances this path, class-extends-interface TS2689 diagnostics should be handled by issue 5315.
