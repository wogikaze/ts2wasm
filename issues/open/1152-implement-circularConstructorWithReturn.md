---
id: 1152
title: "Implement Circularconstructorwithreturn"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5239]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1152.

## Summary

Triage circularConstructorWithReturn across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 1 case failing in directory `circularConstructorWithReturn` with diagnostics: import-export. Fresh coverage and triage show the current blocker is name resolution for a nested class returned from an exported function.

Problem: `circularConstructorWithReturn.ts` fails with `UnresolvedName` for `PrismaClient` at `return PrismaClient`, even though the AST and visible-symbol dump include the nested class declaration.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts --detail
```

## Desired final state

This generated bucket is closed after splitting the current name-resolution blocker into implementation-ready child issue 5239. Do not implement directly from this bucket.

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
- [x] Child issue 5239 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue 5239 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue 5239 acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5239-w0-expr-emit-multi-line-to-linefmt.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts`

## Duplicate detection

Fresh duplicate scan found related class-value work, but no exact match for this
raw name-resolution failure:

- `issues/done/5192-support-first-class-class-constructor-values.md` covers class constructor bindings after they already resolve and then fail as values with `issue-5011`.
- `issues/done/5011-class-runtime-value-semantics.md` records the current structural rejection of unsupported class values.

Issue 5239 is a narrower precondition: bind the nested local `ClassDecl` so
`return PrismaClient` does not fail as `UnresolvedName`.

## Smart triage

### Smart triage: name-resolution

- Issue class: `implementation`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts --detail --no-dashboard-data
```

Source context:

```ts
export type Client = ReturnType<typeof getPrismaClient> extends new () => infer T ? T : never

export function getPrismaClient(options?: any) {
  class PrismaClient {
    self: Client;
    constructor(options?: any) {
      return (this.self = applyModelsAndClientExtensions(this));
    }
  }

  return PrismaClient
}

export function applyModelsAndClientExtensions(client: Client) {
  return client;
}
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Compiler evidence:

```text
tokens: ok
ast: ok; ExportDecl function getPrismaClient contains nested ClassDecl PrismaClient and later Return Ident("PrismaClient")
visible symbols: function getPrismaClient and nested class PrismaClient are listed before failure
resolved: fails during resolve_names with UnresolvedName PrismaClient at 474..486
```

TypeScript oracle evidence:

```text
TypeScript reports ok with no diagnostics.
The oracle type for getPrismaClient is typeof PrismaClient.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts --detail --no-dashboard-data
result: fail on the main checkout; unsupported=1, unsupported_diagcodes=UnresolvedName:1, unsupported_features=name-resolution:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularConstructorWithReturn.ts
result: fail; current blocker is UnresolvedName PrismaClient at return PrismaClient, split into issue 5239
date: 2026-05-06
```

Remaining risks:

- Fixing issue 5239 may expose the later class constructor value behavior tracked by issue 5192.
