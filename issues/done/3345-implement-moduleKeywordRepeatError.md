---
id: 3345
title: "Implement Modulekeywordrepeaterror"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: [5416]
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Split into implementation-ready child issue 5416. Fresh triage for
`moduleKeywordRepeatError.ts` shows the generated bucket's current blocker is a
narrow parser recovery/misclassification problem for `module.module { }`.

## Problem

Reference test results show 1 case failing in directory
`moduleKeywordRepeatError`. Fresh coverage now classifies the failure as
`UnresolvedName` / name-resolution, but compiler dumps show the real first
actionable boundary is parser-owned: `module.module { }` is accepted as a
member-expression statement and the `{ }` block boundary is lost before the
resolver runs.

Problem: this generated bucket is too broad for direct implementation. The
actionable work is tracked by issue 5416.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts --detail
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
- [x] Child issue 5416 contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, parser AST, and TypeScript oracle evidence
- [x] Child issue acceptance names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5416-report-invalid-block-after-member-expression-statement.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts: UnresolvedName: name-resolution
```

Source context:

```text
// @target: es2015
// "module.module { }" should raise a syntax error

module.module { }
```

Compiler evidence:

```text
tokens: Ident("module"), Dot, Ident("module"), LeftBrace, RightBrace
ast: Expr(Member(Ident module, property module))
resolved: UnresolvedName unresolved name `module`
visible symbols: []
```

TypeScript oracle:

```text
TS2591: Cannot find name 'module'.
TS1005: ';' expected. at the `{` after `module.module`
```

Split to:

- `issues/open/5416-report-invalid-block-after-member-expression-statement.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnresolvedName/name-resolution
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts
result: pass; current blocker split to issue 5416
date: 2026-05-08
```

Remaining risks:

- After issue 5416 fixes the parser misclassification, this reference may need
  narrower diagnostics for TS1005 and TS2591 parity.
