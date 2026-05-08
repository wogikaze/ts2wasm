---
id: 3307
title: "Implement Moduleassignmentcompat"
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
build-pass window into a focused implementation issue:
`issues/open/5411a-report-ts2709-for-namespace-variable-annotation.md`.

## Problem

Fresh triage shows the original import/export blocker is stale. All four
`moduleAssignmentCompat*.ts` cases now build-pass, but TypeScript reports
namespace-as-type diagnostics for the variable type annotations:

```text
TS2709: Cannot use namespace 'A' as a type.
TS2709: Cannot use namespace 'B' as a type.
```

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAssignmentCompat --detail --no-dashboard-data
```

Observed result:

```text
executed=4
build_pass=4
unsupported=0
```

Focused triage commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAssignmentCompat1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAssignmentCompat2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAssignmentCompat3.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAssignmentCompat4.ts
```

Representative source shape:

```ts
namespace A {
    export class C { }
}
namespace B {
    export class C { }
    export class D { }
}

var a: A;
var b: B;
a = b;
b = a;
```

Compiler evidence:

```text
tokens: ok through namespace declarations, variable type annotations, and assignments
ast: retains Let a, Let b, Assign a=b, Assign b=a; namespace declarations and annotations are erased
resolved: build-pass with a/b assignments only
```

TypeScript oracle evidence:

```text
moduleAssignmentCompat1.ts: TS2709 for A and B
moduleAssignmentCompat2.ts: TS2709 for A and B
moduleAssignmentCompat3.ts: TS2709 for A and B
moduleAssignmentCompat4.ts: TS2709 for A and B
```

## Desired final state

Implement the representative focused diagnostic work in
`issues/open/5411a-report-ts2709-for-namespace-variable-annotation.md`.
Re-triage the sibling `moduleAssignmentCompat2.ts` through
`moduleAssignmentCompat4.ts` cases after that lands and split follow-ups if
their next blockers differ.

## Scope

In scope:

- [x] Confirm the generated bucket's current evidence.
- [x] Split the false build-pass window into an implementation-ready issue.
- [x] Preserve exact reproduction commands and TypeScript diagnostics.

Out of scope:

- Direct implementation from this generated bucket.
- Full namespace runtime lowering.
- General assignment compatibility between namespace-shaped structural types.

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

- [x] Created `issues/open/5411a-report-ts2709-for-namespace-variable-annotation.md`.
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAssignmentCompat --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAssignmentCompat1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAssignmentCompat2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAssignmentCompat3.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAssignmentCompat4.ts
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

- [x] created: `issues/open/5411a-report-ts2709-for-namespace-variable-annotation.md`

## Notes

Split to
`issues/open/5411a-report-ts2709-for-namespace-variable-annotation.md`.

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

- Issue 5411 may expose additional assignment compatibility diagnostics after
  the first TS2709-style namespace-as-type errors are implemented.
