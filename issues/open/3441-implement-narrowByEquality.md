---
id: 3441
title: "Implement Narrowbyequality"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
---

## Summary

Closed as superseded by
`issues/open/5193-parse-asi-after-ambient-variable-declarations.md`.

## Problem

Reference test results show 1 case fails in directory `narrowByEquality` with
diagnostics: parser-syntax. Fresh triage shows the current first blocker is
`issue-400` for a declaration-only ambient `declare let` without an explicit
semicolon before the next statement.

Problem: narrowByEquality had 1 generated reference failure and needed
smart-triage evidence before implementation starts.

Disposition: no child issue created because existing issue 5193 owns ASI after
ambient variable declarations.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByEquality.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByEquality.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as superseded by an existing implementation-ready owner issue
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
- [x] Fresh evidence contains an exact `reference-triage` command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Existing owner issue 5193 names the exact parser diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByEquality.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByEquality.ts
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

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowByEquality.ts`

## Duplicate detection

- `issues/open/5193-parse-asi-after-ambient-variable-declarations.md`
  owns declaration-only ambient variable declarations followed by a newline
  without an explicit semicolon.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByEquality.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedTypeScriptSyntax:1
unsupported_features=parser-syntax:1
semantic_enabled=0
```

Fresh triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByEquality.ts

result:
UnsupportedTypeScriptSyntax: issue-400: unterminated ambient variable declaration type at 384..391
```

Source context:

```ts
declare let xAndObj: number | string | boolean | object

if (xAndObj == {}) {
    xAndObj;
}
```

Compiler evidence:

```text
tokens: ok; the ambient declaration has no semicolon before the following `if`
ast/resolved: fail at issue-400 unterminated ambient variable declaration type
visible symbols before failure: x, n, s, b, xUnknown
TypeScript oracle: parses and later reports TS2839/TS2322 diagnostics
```

## Completion evidence

Closed as superseded by issue 5193.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByEquality.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedTypeScriptSyntax:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByEquality.ts
result: pass; current first blocker is issue-400 ambient variable declaration ASI, owned by issue 5193
date: 2026-05-08
```

Remaining risks:

- After issue 5193 advances this path, equality narrowing diagnostics may need
  a focused semantic issue.
