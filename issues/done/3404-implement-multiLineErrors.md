---
id: 3404
title: "Split multiLineErrors bucket to object return type parser issue"
type: maintenance
area: frontend/parser
class: superseded
priority: P1
depends_on: [5431]
blocks: []
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Closed the generated `multiLineErrors` bucket by splitting the current blocker into focused child issue #5431.

## Problem

`reference/typescript/tests/cases/compiler/multiLineErrors.ts` still fails before semantic diagnostics because the parser treats a function object type literal return annotation as function body statements.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiLineErrors.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=UnresolvedName:1, unsupported_features=name-resolution:1
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiLineErrors.ts
result: UnresolvedName: unresolved name: string at 66..72
date: 2026-05-08
```

## Evidence

The failing source contains a plain object type literal return annotation:

```ts
function noReturn(): {
    n: string;
    y: number;
}
{
    var x = 4;
    var y = 10;
}
```

The TypeScript parser records the return annotation as a `TypeLiteral`. The current frontend AST instead consumes the annotation braces as the function body and emits labeled statements for `n: string;` and `y: number;`, then leaves the real function body as a top-level block. Resolution then fails on the annotation identifier `string`.

Related existing issues are narrower and do not fully cover this case:

- #5235 covers `x is { ... }` type predicate object return annotations.
- #5257 covers construct signatures in object type literal return annotations such as `{ new(): Object }`.

## Child Issues

- #5431: erase plain object type literal function return annotations.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiLineErrors.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiLineErrors.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- #5431 remains open for implementation.
