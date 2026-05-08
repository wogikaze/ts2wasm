---
id: 1208
title: "Implement Classfieldsuperaccessiblejs"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1208.

## Summary

Triage classFieldSuperAccessibleJs across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh smart triage splits this bucket into two independent current blockers:
`super` inside class static blocks for JS1, and
`D.prototype.foo.call(obj)` receiver-bound dispatch for JS2.

Problem: classFieldSuperAccessibleJs was a stale generated bucket and needed
smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs2.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5319-support-super-property-reads-in-class-static-blocks.md`
- [x] created: `issues/done/5320-support-class-prototype-method-call-dispatch.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs2.ts`
- `reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated 2026-05-07 for both affected paths:

```sh
TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs1.ts

TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs2.ts
```

JS1 result:

```text
diagnostic: UnsupportedSyntax / class
message: issue-254: `super` in class static blocks is not supported at 193..198
source: console.log(super.blah1); console.log(super.blah2);
TypeScript oracle: TS2551 for C.blah2 / super.blah2, no diagnostic for super.blah1
```

JS2 result:

```text
diagnostic: UnsupportedSyntax / class
message: issue-211: D.prototype.foo.call is not supported at 399..424
source: D.prototype.foo.call(obj);
TypeScript oracle: ok, diagnostics=[]
```

Coverage window:

```text
classFieldSuperAccessibleJs1.ts: UnsupportedSyntax: class
classFieldSuperAccessibleJs2.ts: UnsupportedSyntax: class
```

Duplicate review:

- No exact open issue was found for `super` property reads in class static
  blocks.
- No exact open issue was found for direct
  `Class.prototype.method.call(receiver)` dispatch; broad issue 435 tracks
  method-call coverage generally.

## Completion evidence

This generated bucket was split and closed as superseded by issues 5319 and
5320.

Commits:

- this commit

Validation result:

```text
command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs1.ts
result: pass; current blocker split to static-block super property issue 5319
date: 2026-05-07

command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs2.ts
result: pass; current blocker split to class-prototype method call issue 5320
date: 2026-05-07

command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs --detail --no-dashboard-data
result: pass; two-file window reports UnsupportedSyntax:2
date: 2026-05-07
```

Remaining risks:

- Implementations remain open in issues 5319 and 5320.
