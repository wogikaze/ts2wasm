---
id: 1210
title: "Implement Classfieldsupernotaccessiblejs"
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
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1210.

## Summary

Triage classFieldSuperNotAccessibleJs across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh smart triage shows this file parses and reaches lowering. The current
blocker is `this.b()` where `b` is declared as a class auto-accessor initialized
with an arrow function.

Problem: classFieldSuperNotAccessibleJs was a stale generated bucket and needed
smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classFieldSuperNotAccessibleJs.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperNotAccessibleJs.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperNotAccessibleJs.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classFieldSuperNotAccessibleJs.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5322-support-callable-class-auto-accessor-fields.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classFieldSuperNotAccessibleJs.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated 2026-05-07:

```sh
TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/classFieldSuperNotAccessibleJs.ts
```

Result:

```text
diagnostic: UnsupportedSyntax / class
message: method `YaddaBase.b` not found at 344..352
source: this.b()
```

Compiler evidence:

- Tokens and AST succeed.
- AST includes constructor body calls and `accessor b = () => { this.foo = 10 }`.
- Lowering treats `this.b()` as a missing method instead of a callable
  auto-accessor field.

TypeScript oracle evidence:

- Reports TS2339 property diagnostics for `roots`, `justProp`, `foo`, and
  `literalElementAccess`.
- Does not report a diagnostic for `this.b()`.

Duplicate review:

- `issues/open/422-implement-class-accessor.md` is related but is a broad
  generated triage bucket.
- No exact open/done owner was found for callable class auto-accessor fields.

## Completion evidence

This generated bucket was split and closed as superseded by issue 5322.

Commits:

- this commit

Validation result:

```text
command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperNotAccessibleJs.ts
result: pass; current blocker split to callable class auto-accessor field support
date: 2026-05-07

command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperNotAccessibleJs.ts --detail --no-dashboard-data
result: pass; single-file window reports UnsupportedSyntax: class
date: 2026-05-07
```

Remaining risks:

- Implementation remains open in issue 5322.
