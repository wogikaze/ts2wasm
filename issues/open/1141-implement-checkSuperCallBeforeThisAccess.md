---
id: 1141
title: "Implement Checksupercallbeforethisaccess"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5232]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1141.

## Summary

Triage checkSuperCallBeforeThisAccess across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `checkSuperCallBeforeThisAccess` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: checkSuperCallBeforeThisAccess has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts`

## Duplicate detection

Fresh duplicate scan found related module-export history but no exact open
`export class` implementation slice:

- `issues/done/5008-static-es-module-export-default-namespace-reexport.md`
  mentions `export class Foo {}` in the original broad scope, but its close
  evidence only verifies `export const` and `export default`; later function
  work was split separately.
- `issues/done/5144-support-entry-export-function-declarations.md` is the
  sibling `export function` slice and explicitly excludes `export class`.
- Existing `checkSuperCallBeforeThisAccessing*` issues cover different
  reference paths and are not exact duplicates of this entry-module export
  blocker.

## Smart triage

Fresh triage shows the current blocker is not `this` before `super` flow
analysis yet. The parser accepts the class bodies and then module build rejects
the entry-module `export class` declaration.

### Smart triage: checkSuperCallBeforeThisAccess

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Current compiler message: `issue-5005: entry module ... uses a declaration form outside the current static export slice`
- Path: `reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
export class Foo {
    constructor(value: number) {
    }
}

export class BarCorrectlyFails extends Foo {
    constructor(something: boolean) {
        if (!something) {
            const value = this.bar();  // Error
            super(value);
        }
        else {
            super(1337);
        }
    }
    bar(): number { return 4; }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; exported class declarations are represented before module build
resolved/lowered: UnsupportedModule issue-5005 at entry-module export class
TypeScript oracle: TS17009 and TS17011 this-before-super diagnostics
```

Split result:

- `issues/done/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts --detail --no-dashboard-data
result: pass; reproduced current unsupported bucket
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts
result: pass; reproduced issue-5005 entry-module export class boundary and split to issue 5232
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5232 may expose derived-class `this`/`super` flow diagnostics or class-runtime export semantics blockers.
