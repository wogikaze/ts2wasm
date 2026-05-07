---
id: 1509
title: "Implement Contextualtypecaching"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1509.

## Summary

Triage contextualTypeCaching across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypeCaching` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypeCaching has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeCaching.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeCaching.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Fold into the existing ambient const generic annotation owner
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

- [x] Duplicate candidates below are confirmed; this issue is superseded by issue 5345
- [x] Existing issue 5345 contains exact `reference-triage` commands and accepted source forms
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5345 acceptance names the exact generic callable ambient const annotation shape

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeCaching.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeCaching.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into `issues/open/5345-parse-generic-ambient-const-type-annotations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypeCaching.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows this generated parser-syntax bucket is the
same ambient const generic callable annotation boundary already tracked by
`issues/open/5345-parse-generic-ambient-const-type-annotations.md`.

Current diagnostic:

```text
UnsupportedTypeScriptSyntax: issue-400: unterminated ambient variable declaration at 864..871
```

Source context:

```ts
declare const A: <T, P extends keyof T>(
  obj: T,
  prop: P,
  factory: () => T[P]
) => void;
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedTypeScriptSyntax:1
unsupported_features=parser-syntax:1
```

TypeScript oracle accepts the declaration and reports the binding type as
`<T, P extends keyof T>(obj: T, prop: P, factory: () => T[P]) => void`.
Issue 5345 already includes generic callable ambient const annotations in its
scope and acceptance criteria, so no new child issue is required.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeCaching.ts --detail --no-dashboard-data
result: pass; current blocker is UnsupportedTypeScriptSyntax/parser-syntax at the ambient const generic callable annotation
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeCaching.ts
result: pass; triage identifies issue-400 unterminated ambient variable declaration at `declare const A: <T, P extends keyof T>(...) => void`
date: 2026-05-07
```

Remaining risks:

- The reference path remains parser-unsupported until issue 5345 implements ambient const generic callable annotation erasure.
