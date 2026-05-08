---
id: 3419
title: "Implement Mutuallyrecursiveinference"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5002]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Closed as superseded by open implementation owner
`5356-report-uninitialized-generic-class-fields.md`.

Fresh triage showed this bucket is a false build pass for uninitialized typed
instance class fields. The compiler successfully parses and resolves the
mutually recursive generic class declarations, but it erases typed instance
field declarations and misses the TypeScript oracle's TS2564 diagnostics.

## Problem

Reference test results show 1 cases fail in directory `mutuallyRecursiveInference` with diagnostics: type-system. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: mutuallyRecursiveInference has 1 reference failure and needed
smart-triage evidence before implementation starts.

Disposition: the implementation work remains open in issue `5356`, which now
records this representative fold-in case.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveInference.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveInference.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket into the existing implementation owner issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the owner issue

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
- [x] Owner issue contains an exact `reference-triage` command
- [x] Owner issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Owner issue acceptance covers the same TS2564 diagnostic family

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveInference.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveInference.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; implementation remains open in `5356`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/mutuallyRecursiveInference.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveInference.ts --detail --no-dashboard-data

result:
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
reference/typescript/tests/cases/compiler/mutuallyRecursiveInference.ts: build_pass
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveInference.ts

result:
BuildPass: ts2wasm build succeeded
TypeScript oracle reports TS2564 for uninitialized fields T.a, X.a, and X.b.
```

Source context:

```ts
class T<A> {
    a: A;
    b: any
}
class L<RT extends { a: 'a' | 'b', b: any }> extends T<RT[RT['a']]> {
    m() { this.a }
}
class X extends L<X> {
    a: 'a' | 'b'
    b: number
    m2() {
        this.a
    }
}
```

Compiler evidence:

```text
tokens: ok through generic classes and heritage L<RT extends { a: 'a' | 'b', b: any }> extends T<RT[RT['a']]>
ast/resolved: ok; classes T, L, and X are retained; L.m and X.m2 contain this.a
semantic gap: typed instance fields are erased, so TS2564-style diagnostics are not emitted
coverage: executed=1, build_pass=1, unsupported=0
```

TypeScript oracle evidence:

```text
TS2564: Property 'a' has no initializer and is not definitely assigned in the constructor. (class T)
TS2564: Property 'a' has no initializer and is not definitely assigned in the constructor. (class X)
TS2564: Property 'b' has no initializer and is not definitely assigned in the constructor. (class X)
```

## Completion evidence

Superseded by:

- `5356-report-uninitialized-generic-class-fields.md`

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveInference.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveInference.ts
result: pass; BuildPass with oracle TS2564 diagnostics for T.a, X.a, and X.b
date: 2026-05-08
```

Remaining risks:

- Implementation remains open in `5356`.
