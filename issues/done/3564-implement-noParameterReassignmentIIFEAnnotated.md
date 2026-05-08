---
id: 3564
title: "Implement Noparameterreassignmentiifeannotated"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [5479]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage noParameterReassignmentIIFEAnnotated across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows this bucket is not blocked by parameter reassignment yet.
The current compiler stops at the top-level WebWorker global `self` in
`self.importScripts = (...)`. That worker-global name-resolution boundary is
split to issue 5479.

Problem: `noParameterReassignmentIIFEAnnotated.ts` is superseded by issue 5479
until `self` and `importScripts` worker globals are resolver-visible or
diagnosed precisely.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noParameterReassignmentIIFEAnnotated.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noParameterReassignmentIIFEAnnotated.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split worker-global name resolution to issue 5479
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
- [x] Child issue 5479 contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noParameterReassignmentIIFEAnnotated.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noParameterReassignmentIIFEAnnotated.ts
```

Not run:

- broad Rust gates; no source implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5479-bind-dom-worker-self-importscripts-globals.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noParameterReassignmentIIFEAnnotated.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-08:

- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Message: `unresolved name: \`self\` at 100..104`
- Source context: `self.importScripts = (function (importScripts) { ... })(importScripts);`
- Visible symbols before failure: none.
- Tokens and AST succeed. The AST is a top-level `PropertyAssign` with
  `Ident self` as the object and an IIFE call as the assigned value.
- TypeScript oracle reports later TS2683 for `this` and TS2345 for
  `arguments`, so parameter reassignment / nested function semantics remain
  unproven until worker globals resolve.

## Completion evidence

Split to issue 5479.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noParameterReassignmentIIFEAnnotated.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noParameterReassignmentIIFEAnnotated.ts
result: pass; current blocker is unresolved worker global `self`, split to issue 5479
date: 2026-05-08
```

Remaining risks:

- After issue 5479 resolves worker globals, this path may expose the intended
  no-parameter-reassignment, nested `this`, or `arguments` diagnostics.
