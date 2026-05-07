---
id: 1255
title: "Implement Clodulestaticmembers"
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
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1255.

## Summary

Closed after splitting the current blocker to
`issues/open/5271-parse-modified-static-class-fields.md`. Fresh triage shows
the first failure is a parser gap for modified static class fields.

## Problem

Reference test results show 1 case failing in directory `cloduleStaticMembers`.
Fresh triage confirms the current blocker is `private static x = 10;`.

Problem: `cloduleStaticMembers.ts` reports `expected LeftParen, got
Some(Ident("x"))` at `private static x = 10;`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm related static field issues do not own this modifier field-name parser gap
- [x] Split one observable behavior into child issue 5271
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

- [x] Duplicate candidates below are confirmed and the exact behavior is split to 5271
- [x] Child issue 5271 contains an exact `python scripts/manager.py reference-triage ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5271-parse-modified-static-class-fields.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts`

Source context:

```ts
class Clod {
    private static x = 10;
    public static y = 10;
}
```

## Duplicate detection

- `issues/open/5254-parse-asi-between-static-class-fields.md` is related but
  owns ASI between uninitialized static fields, not modifier-prefixed static
  fields with initializers.
- No existing implementation-ready child owned `private static x = 10;`, so
  this bucket was split to issue 5271.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: expected LeftParen, got Some(Ident("x")) at 53..54
Source: private static x = 10;
tokens: ok; private, static, Ident("x"), Equal, Number(10)
AST: fails before modified static field construction
resolved: same parser failure
TypeScript oracle: parses; expected diagnostics TS2341 and TS2304
Child issue: 5271
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5271-parse-modified-static-class-fields.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleStaticMembers.ts
result: pass; reproduced modified static field parser failure and split child issue 5271
date: 2026-05-06
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5271
