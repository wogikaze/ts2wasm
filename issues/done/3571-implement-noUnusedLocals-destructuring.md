---
id: 3571
title: "Implement Nounusedlocals Destructuring"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: [5481]
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage noUnusedLocals-destructuring across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows the fixture tokenizes and builds an AST, but the
parenthesized object destructuring assignment is represented as assignment to a
synthetic name:

```text
Assign { name: "{x}", expr: This }
UnresolvedName: unresolved name: `{x}`
```

Problem: noUnusedLocals-destructuring has a destructuring assignment
representation gap, now split to issue 5481.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noUnusedLocals_destructuringAssignment.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_destructuringAssignment.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one observable behavior into child issue 5481
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

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
- [x] Child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_destructuringAssignment.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_destructuringAssignment.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only issue split.
- `cargo nextest run`; metadata-only issue split.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5481-represent-object-destructuring-assignment-statements.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noUnusedLocals_destructuringAssignment.ts`

## Duplicate detection

- `issues/open/425-implement-destructuring.md` - Implement destructuring (same feature label, title overlap)
- `issues/done/247-implement-destructuring-binding-pattern-parser.md` - Implement destructuring binding pattern parser support (same feature label, title overlap)
- `issues/done/251-implement-destructuring-binding-runtime-semantics.md` - Implement destructuring binding runtime semantics (same feature label, title overlap)
- `issues/done/252-implement-destructuring-assignment-pattern-parser.md` - Implement destructuring assignment pattern parser support (same feature label, title overlap)

## Smart triage

### Smart triage: Triage name resolution: noUnusedLocals destructuringAssignment

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/noUnusedLocals_destructuringAssignment.ts`

Current compiler message:

```text
unresolved name: `{x}`
```

Compiler evidence:

```text
tokens: ok through parenthesized object destructuring assignments
ast: ok but `({ x } = this)` becomes Assign { name: "{x}", expr: This }
resolved/lowered: UnresolvedName for synthetic name `{x}`
```

TypeScript oracle:

```text
diagnostics=[]
local x: number
local f: Function
```

## Completion evidence

Status: done

Commits:

- this local issue-cleanup commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_destructuringAssignment.ts --detail --no-dashboard-data
result: pass; representative path reports UnresolvedName/name-resolution for synthetic destructuring target `{x}`
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_destructuringAssignment.ts
result: pass; fresh triage split the destructuring assignment representation gap to issue 5481
date: 2026-05-08
```

Remaining risks:

- The destructuring assignment behavior is not implemented yet; issue 5481 owns the remaining work.
