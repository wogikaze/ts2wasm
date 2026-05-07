---
id: 1346
title: "Implement Commentonambientvariable"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1346.

## Summary

Closed this generated bucket by splitting the current concrete blocker to
`issues/open/5344-resolve-ambient-var-assignment-targets.md`.

## Problem

Fresh triage confirms the current blocker is ambient value name resolution for
an assignment target. The parser tokenizes `declare var x: number;` and
`x = 2;`, erases the ambient declaration from the runtime AST, then
`resolve_names` reports `UnresolvedName` for the assignment target `x`.

Problem: declaration-only ambient variable `x` is visible to TypeScript but not
to ts2wasm name resolution after erasure.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnAmbientVariable2.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnAmbientVariable2.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implement from
`issues/open/5344-resolve-ambient-var-assignment-targets.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
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
- [x] Child issue contains exact `reference-triage` commands
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names this ambient value assignment target shape

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnAmbientVariable2.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnAmbientVariable2.ts
```

Not run:

- cargo fmt / nextest not run for this metadata-only issue lifecycle closure

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5344-resolve-ambient-var-assignment-targets.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentOnAmbientVariable2.ts`

## Duplicate detection

Split to `issues/open/5344-resolve-ambient-var-assignment-targets.md`.

Issue 5161 owns declaration-only ambient values in expression positions. This
bucket is narrower: the current first blocker is assignment target resolution
for `declare var x: number; x = 2;`.

## Smart triage

Generated 2026-05-07 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnAmbientVariable2.ts
```

Result:

```text
Smart triage: Triage name resolution: commentOnAmbientVariable2
Diagnostic: UnresolvedName / resolver-symbol
Message: unresolved name: `x` at 206..212
Feature label: name-resolution
tokens: ok through var y = 1, declare var x: number, x = 2
ast: ok; Let y = 1, Assign x = 2
resolved: fail in resolve_names with UnresolvedName for x
visible symbols: y only
TypeScript oracle: ok, diagnostics=[], hints y:number and x:number
```

Source context:

```ts
// @Filename: commentOnAmbientVariable2_1.ts
var y = 1;

// @Filename: commentOnAmbientVariable2_2.ts
/// <reference path='commentOnAmbientVariable2_1.ts'/>
declare var x: number;
x = 2;
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
semantic_enabled=0
```

## Completion evidence

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnAmbientVariable2.ts
result: pass; reproduced UnresolvedName for ambient x assignment and split to issue 5344
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnAmbientVariable2.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnresolvedName=1
date: 2026-05-07
```

Remaining risks:

- Issue 5344 still needs implementation; this closure only removes the generated bucket from the blocked queue.
