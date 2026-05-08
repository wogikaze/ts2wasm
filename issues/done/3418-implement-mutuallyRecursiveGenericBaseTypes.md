---
id: 3418
title: "Split mutuallyRecursiveGenericBaseTypes bucket"
type: maintenance
area: frontend/semantics
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed the generated `mutuallyRecursiveGenericBaseTypes` bucket after splitting
the two current reference outcomes:

- `mutuallyRecursiveGenericBaseTypes1.ts` is superseded by
  `issues/open/5222-support-interface-typed-method-calls-on-erased-locals.md`.
- `mutuallyRecursiveGenericBaseTypes2.ts` is split to
  `issues/open/5437-report-typed-class-method-null-return.md`.

## Problem

Fresh coverage shows one active `issue-211` interface-typed receiver blocker
and one false build-pass with a TypeScript TS2322 null-return diagnostic.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveGenericBaseTypes --detail --no-dashboard-data
result: executed=2, build_pass=1, unsupported=1, blocked=0, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveGenericBaseTypes1.ts
result: UnsupportedSyntax issue-211 unknown receiver class for method `foo`
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveGenericBaseTypes2.ts
result: BuildPass; TypeScript oracle reports TS2322 for `return null`
date: 2026-05-08
```

## Evidence

Case 1:

```ts
interface A<T> {
    foo(): B<T>;
    foo(): void;
    foo2(): B<number>;
}

interface B<T> extends A<T> {
    bar(): void;
}

var b: B<number>;
b.foo();
```

Compiler evidence: AST has `Let b` and `Call(Member(b, "foo"))`; lowering
reports `issue-211: unknown receiver class for method foo`. TypeScript reports
the later TS2454 definite-assignment diagnostic for `b`.

Case 2:

```ts
class foo<T> {
    bar(): foo2<T[]> { return null; }
}

class foo2<T> extends foo<T> {
}
```

Compiler evidence: tokens, AST, resolved, and build all succeed. TypeScript
reports `TS2322 Type 'null' is not assignable to type 'foo2<T[]>'.`

## Validation

Issue sync and health checks:

```text
python scripts/manager.py update-issue-index
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Focused reference checks:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveGenericBaseTypes --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveGenericBaseTypes1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveGenericBaseTypes2.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- Implementations remain open in issues 5222 and 5437.
