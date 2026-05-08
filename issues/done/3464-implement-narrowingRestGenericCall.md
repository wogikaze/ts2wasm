---
id: 3464
title: "Implement Narrowingrestgenericcall"
type: spike
area: frontend/semantics
class: superseded
priority: P1
depends_on: [5002]
blocks: []
status: done
completed: 2026-05-08
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as superseded by the existing implementation-ready ambient value
name-resolution owner:
`issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`.

## Problem

Reference test results show 1 case fails in directory
`narrowingRestGenericCall` with diagnostics: type-system.

Fresh triage shows the current blocker is not rest-generic-call narrowing yet.
The parser succeeds, then the resolver rejects the ambient value declared by:

```ts
declare let obj: Slugs;
```

The later `call(obj, ({foo, ...rest}) => { ... })` use reports
`UnresolvedName: unresolved name: \`obj\``, which is the same declaration-only
ambient value visibility gap already tracked by issue 5161.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingRestGenericCall.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingRestGenericCall.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingRestGenericCall.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingRestGenericCall.ts
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

- [x] folded into `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingRestGenericCall.ts`

## Duplicate detection

- `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`
  is an exact implementation-ready owner for declaration-only
  `declare var` / `declare let` / `declare const` values that are visible in
  source but rejected by name resolution after ambient erasure.
- Generic name-resolution buckets are no-match because they share only the
  broad feature label.
- Rest destructuring and narrowing issues are not matches for the current
  blocker because the compiler stops before callback narrowing is reached.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingRestGenericCall.ts
```

Result:

```text
Feature label: name-resolution
Diagnostic: UnresolvedName / resolver-symbol
Message: unresolved name: `obj` at 188..191
Failure location: line 12, column 17
Source context: call(obj, ({foo, ...rest}) => {
Visible symbols before failure: obj binding from line 11, column 9
tokens: ok
ast: ok; ambient declaration is erased, call expression remains
resolved: fails resolving the later obj reference
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingRestGenericCall.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingRestGenericCall.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnresolvedName, name-resolution
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingRestGenericCall.ts
result: pass; current blocker is ambient declare-let name resolution, folded into issue 5161
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- After issue 5161 is implemented, this reference may expose rest destructuring
  binding, generic contextual typing, or narrowing behavior. Split those
  separately if they appear.
