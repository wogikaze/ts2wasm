---
id: 3417
title: "Close mutuallyRecursiveCallbacks to ambient var assignment owner"
type: maintenance
area: frontend/resolver
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

Closed the generated `mutuallyRecursiveCallbacks` bucket as superseded by
`issues/open/5344-resolve-ambient-var-assignment-targets.md`.

## Problem

Fresh triage shows the current first blocker is not the recursive callback type
relationship. The parser erases `declare var bar: Bar<{}>;`, then name
resolution rejects the later assignment target `bar = foo;`.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveCallbacks.ts --detail --no-dashboard-data
result: executed=1, unsupported=1, blocked=0, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveCallbacks.ts
result: UnresolvedName unresolved name: `bar` at 199..209
date: 2026-05-08
```

## Evidence

Source context:

```ts
interface Foo<T> { (bar: Bar<T>): void };
type Bar<T> = (foo: Foo<T>) => Foo<T>;
declare function foo<T>(bar: Bar<T>): void;
declare var bar: Bar<{}>;
bar = foo;
```

Compiler evidence:

```text
tokens: ok through interface Foo<T>, type Bar<T>, declare function foo<T>, declare var bar, and assignment
ast: Function foo with empty body; Assign bar = foo
resolved: fails in resolve_names with UnresolvedName for assignment target bar
visible symbols: ambient binding bar is listed by triage but not resolver-visible for assignment
```

TypeScript oracle evidence:

```text
TS2322: Type '<T>(bar: Bar<T>) => void' is not assignable to type 'Bar<{}>'.
Oracle binding hint: bar has type Bar<{}>.
```

Issue 5344 already owns declaration-only ambient `var` bindings used as
assignment targets. This path is folded into that owner; recursive callback
assignability remains behind the ambient assignment-target resolver blocker.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveCallbacks.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveCallbacks.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- Implementation remains open in issue 5344.
