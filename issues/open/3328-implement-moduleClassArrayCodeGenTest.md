---
id: 3328
title: "Implement Moduleclassarraycodegentest"
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

Closed this generated `import-export` bucket because the current compiler build
now passes. The remaining TypeScript oracle mismatch is split to issue 5409.

## Problem

Fresh coverage shows `moduleClassArrayCodeGenTest.ts` no longer fails with
`import-export`; it is a build pass. TypeScript still reports TS2694 for a
non-exported namespace member used in a qualified array type annotation.

Problem: the stale generated blocker is gone, and the remaining semantic parity
gap belongs to a focused TS2694 namespace-member diagnostic issue.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleClassArrayCodeGenTest --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleClassArrayCodeGenTest.ts
```

## Desired final state

This generated bucket is closed. Implement semantic parity from
`issues/open/5409a-report-non-exported-namespace-member-type-annotations.md`.

## Scope

In scope:

- [x] Inspect fresh coverage and smart triage.
- [x] Confirm the stale import-export blocker is gone.
- [x] Split the remaining semantic oracle mismatch into a child issue.
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence.

Out of scope:

- Direct implementation from this generated bucket.
- Broad namespace type-checker parity.
- Qualified class heritage diagnostics, tracked separately by issue 5313.
- Import-equals alias missing-member diagnostics, tracked separately by issue 5397.

## Affected paths

Expected:

- `issues/open/5409a-report-non-exported-namespace-member-type-annotations.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Child issue contains an exact `reference-triage` command.
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence.
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleClassArrayCodeGenTest --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleClassArrayCodeGenTest.ts
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

- [x] created: `issues/open/5409a-report-non-exported-namespace-member-type-annotations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleClassArrayCodeGenTest.ts`

## Duplicate detection

- `issues/open/5313-report-non-exported-namespace-member-qualified-heritage.md`
  covers non-exported namespace members in class heritage clauses, not type
  annotations.
- `issues/open/5397-report-missing-namespace-alias-member-diagnostic.md`
  covers import-equals aliases, not direct namespace member annotations.
- No exact owner existed for `var t2: M.B[] = []`, so issue 5409 was created.

## Smart triage

Fresh coverage on 2026-05-08:

```text
executed=1
build_pass=1
unsupported=0
```

The source is:

```ts
namespace M {
    export class A { }
    class B{ }
}

var t: M.A[] = [];
var t2: M.B[] = [];
```

The compiler tokens include namespace `M`, exported class `A`, non-exported
class `B`, and both qualified array type annotations. The retained runtime AST
and resolved dump contain only the two array initializers:

```text
Let("t", Array([]))
Let("t2", Array([]))
```

TypeScript oracle still reports:

```text
TS2694: Namespace 'M' has no exported member 'B'.
```

That semantic parity gap is split to issue 5409.

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleClassArrayCodeGenTest --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleClassArrayCodeGenTest.ts
result: pass; compiler build-passes, TypeScript oracle reports TS2694 split to issue 5409
date: 2026-05-08
```

Remaining risks:

- TS2694-like non-exported namespace member diagnostics for type annotations remain open in issue 5409.
