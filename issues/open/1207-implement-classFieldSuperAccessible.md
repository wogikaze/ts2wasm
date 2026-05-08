---
id: 1207
title: "Implement Classfieldsuperaccessible"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1207.

## Summary

Triage classFieldSuperAccessible across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh smart triage shows the parser now accepts the representative file. The
current blocker is resolver/builtin validation rejecting `extends class Expr {}`
with the generic simple-inheritance diagnostic.

Problem: classFieldSuperAccessible was a stale generated bucket and needed
smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classFieldSuperAccessible.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperAccessible.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperAccessible.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classFieldSuperAccessible.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5318-support-class-expression-class-heritage.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classFieldSuperAccessible.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated 2026-05-07:

```sh
TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/classFieldSuperAccessible.ts
```

Result:

```text
diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
message: only simple inheritance (extends ClassName) is supported
TypeScript oracle: ok, diagnostics=[]
visible symbols: class A, class Expr, class B, class C, class D, class E
```

Compiler evidence:

- Tokens succeed for `class A extends class Expr {} { ... }`.
- AST succeeds with `ClassDecl A extends ClassExpr Expr` and a static block
  containing `console.log(super.name)`.
- Resolved stage stops in builtin/class heritage validation before later
  `super` property behavior can be observed.

Duplicate review:

- `issues/open/5248-lower-class-expressions.md` covers class expressions as
  ordinary values, not direct class heritage.
- `issues/open/5252-support-call-expression-class-heritage.md` covers
  `extends makeBaseClass()`.
- `issues/open/5256-report-non-constructor-class-heritage-expressions.md`
  covers invalid heritage diagnostics.
- No exact open/done owner was found for direct `extends class Expr {}` support.

## Completion evidence

This generated bucket was split and closed as superseded by issue 5318.

Commits:

- this commit

Validation result:

```text
command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperAccessible.ts
result: pass; current blocker split to direct class-expression heritage support
date: 2026-05-07

command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperAccessible.ts --detail --no-dashboard-data
result: pass; single-file window reports UnsupportedSyntax: class
date: 2026-05-07
```

Remaining risks:

- Implementation remains open in issue 5318.
