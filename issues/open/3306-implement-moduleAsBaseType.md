---
id: 3306
title: "Implement Moduleasbasetype"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import/export bucket by splitting the current false
build-pass into a focused implementation issue:
`issues/open/5410a-report-namespace-as-base-type-diagnostics.md`.

## Problem

Fresh triage shows the original import/export blocker is stale. The compiler now
build-passes the reference case, but TypeScript reports namespace-as-base
diagnostics:

```text
TS2708: Cannot use namespace 'M' as a value.
TS2709: Cannot use namespace 'M' as a type.
TS2709: Cannot use namespace 'M' as a type.
```

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAsBaseType --detail --no-dashboard-data
```

Observed result:

```text
executed=1
build_pass=1
unsupported=0
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAsBaseType.ts
```

Source shape:

```ts
namespace M {}
class C extends M {}
interface I extends M { }
class C2 implements M { }
```

Compiler evidence:

```text
tokens: ok through namespace M, class C extends M, interface I extends M, and class C2 implements M
ast: ClassDecl C extends Ident("M"); ClassDecl C2 retained without implements; namespace and interface are erased
resolved: ClassDecl C extends "M"; ClassDecl C2 retained
```

TypeScript oracle evidence:

```text
TS2708: Cannot use namespace 'M' as a value.        // class C extends M
TS2709: Cannot use namespace 'M' as a type.         // interface I extends M
TS2709: Cannot use namespace 'M' as a type.         // class C2 implements M
```

## Desired final state

Implement the focused diagnostic work in
`issues/open/5410a-report-namespace-as-base-type-diagnostics.md`.

## Scope

In scope:

- [x] Confirm the generated bucket's current evidence.
- [x] Split the false build-pass into an implementation-ready issue.
- [x] Preserve exact reproduction commands and TypeScript diagnostics.

Out of scope:

- Direct implementation from this generated bucket.
- Full namespace runtime lowering.
- General class/interface heritage type checking.

## Affected paths

Expected implementation owner:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused frontend/resolver tests or fixtures

Do not touch from this bucket:

- backend namespace emit
- static ES module resolution
- package resolution

## Acceptance criteria

- [x] Created `issues/open/5410a-report-namespace-as-base-type-diagnostics.md`.
- [x] Exact reproduction commands and observed diagnostics are recorded.
- [x] The generated bucket no longer remains as a stale blocked import/export issue.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
git diff --cached --check
```

Reference commands already run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAsBaseType --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAsBaseType.ts
```

Not run:

- `cargo fmt --all --check` (issue lifecycle only; no Rust changes)
- `cargo nextest run` (issue lifecycle only; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5410a-report-namespace-as-base-type-diagnostics.md`

## Notes

Split to `issues/open/5410a-report-namespace-as-base-type-diagnostics.md`.

## Completion evidence

Commits:

- filled by commit

Validation result:

```text
command: python scripts/manager.py update-issue-index
result: pass
date: 2026-05-08

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-08

command: git diff --check
result: pass
date: 2026-05-08

command: git diff --cached --check
result: pass
date: 2026-05-08
```

Remaining risks:

- Issue 5410 may expose additional namespace or heritage diagnostics after the
  first TS2708/TS2709-style error is implemented.
