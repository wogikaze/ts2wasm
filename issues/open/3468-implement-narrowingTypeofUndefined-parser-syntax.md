---
id: 3468
title: "Implement Narrowingtypeofundefined Parser Syntax"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: []
status: done
completed: 2026-05-08
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as superseded by the existing implementation-ready ambient variable ASI
and type-erasure owner:
`issues/open/5193-parse-asi-after-ambient-variable-declarations.md`.

## Problem

Reference test results show 1 case fails in directory
`narrowingTypeofUndefined-parser-syntax` with diagnostics: parser-syntax.

Fresh triage shows the current blocker is the declaration-only ambient const:

```ts
declare const a: { error: { prop: string }, result: undefined } | { error: undefined, result: { prop: number } }
```

The parser reports `issue-400: unterminated ambient variable declaration type`
before reaching the later `typeof a.error` narrowing checks. This is the same
ambient variable declaration ASI/type-erasure boundary already tracked by issue
5193.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingTypeofUndefined1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTypeofUndefined1.ts --detail
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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTypeofUndefined1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingTypeofUndefined1.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into `issues/open/5193-parse-asi-after-ambient-variable-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingTypeofUndefined1.ts`

## Duplicate detection

- `issues/open/5193-parse-asi-after-ambient-variable-declarations.md` is an
  exact implementation-ready owner for declaration-only ambient variables that
  stop at `issue-400: unterminated ambient variable declaration type` before a
  following statement.
- Broad parser syntax issues 059 and 442 are no-match because 5193 is the
  narrow implementation owner.
- Other parser-syntax generated buckets share only the broad feature label.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingTypeofUndefined1.ts
```

Result:

```text
Feature label: parser-syntax
Diagnostic: UnsupportedTypeScriptSyntax / unsupported-feature-boundary
Message: issue-400: unterminated ambient variable declaration type at 19..26
Failure location: line 2, column 1
Source context: declare const a: { error: { prop: string }, result: undefined } | { error: undefined, result: { prop: number } }
tokens: ok, including nested object type literals, union pipe, and following if statements
ast: fails at issue-400 before preserving ambient declaration metadata
resolved: same parser boundary
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTypeofUndefined1.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedTypeScriptSyntax:1
unsupported_features=parser-syntax:1
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTypeofUndefined1.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnsupportedTypeScriptSyntax, parser-syntax
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingTypeofUndefined1.ts
result: pass; current blocker is ambient variable declaration ASI/type-erasure, folded into issue 5193
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- After issue 5193 is implemented, this reference may expose ambient value
  name resolution or `typeof a.error` discriminated-union narrowing behavior.
  Split those separately if they appear.
