---
id: 5455
title: "Report nullable object receiver after typeof object check"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TypeScript-style possibly-null diagnostic for method calls on
`unknown` values narrowed only by `typeof x === "object"`.

Split from generated bucket
`issues/done/3465-implement-narrowingTruthyObject.md`.

## Problem

Problem: `narrowingTruthyObject.ts` parses and resolves, then lower_program
reports the generic issue-211 method receiver diagnostic for:

```ts
if (typeof x === 'object') {
    x.toString();
}
```

TypeScript accepts the syntax but reports TS18047 because `typeof null` is
`"object"` and `x` is still possibly null at the method call. The current
compiler falls through to method-call receiver lowering instead:

```text
UnsupportedSyntax: issue-211: unknown receiver class for method `toString` at 122..134
```

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingTruthyObject.ts
```

Representative source:

```ts
function foo(x: unknown, b: boolean) {
    if (typeof x === 'object') {
        x.toString();
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; If condition is typeof x === "object", body contains x.toString()
resolved: ok through builtins
lower_program: issue-211 unknown receiver class for method `toString`
TypeScript oracle: TS18047 "'x' is possibly 'null'."
```

## Desired final state

The compiler recognizes that `typeof x === "object"` narrows `unknown` to an
object-or-null receiver and reports a source-spanned possibly-null diagnostic
for `x.toString()` before generic issue-211 receiver lowering. The
representative path should no longer classify the first `x.toString()` as an
unknown receiver class.

## Scope

In scope:

- [ ] Detect method/property access on a receiver narrowed only by
  `typeof value === "object"`.
- [ ] Report a source-spanned possibly-null diagnostic for the receiver rather
  than `issue-211`.
- [ ] Preserve current handling for truthiness-guarded object checks such as
  `typeof x === "object" && x`.
- [ ] Add focused coverage for `if (typeof x === "object") { x.toString(); }`.
- [ ] Re-run the representative triage and record any later truthy-object
  method-call blocker.

Out of scope:

- Full TypeScript control-flow narrowing.
- Supporting `Object.prototype.toString` or `Object.prototype.hasOwnProperty`
  on truthiness-guarded object receivers after this first diagnostic advances.
- Number `toString` receiver classification, tracked by
  `issues/open/5451-classify-number-tostring-after-typeof-switch-narrowing.md`.
- Interface-typed erased-local method receivers, tracked by
  `issues/open/5222-support-interface-typed-method-calls-on-erased-locals.md`.
- Broad Object builtin coverage, tracked by `issues/open/342-implement-object-builtin-coverage.md`.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- focused CLI/reference fixtures

Do not touch:

- backend/runtime ABI unless a focused fixture proves a runtime Object method
  implementation is the smallest required step
- broad method-call receiver lowering unrelated to `typeof object` narrowing

## Acceptance criteria

- [ ] `if (typeof x === "object") { x.toString(); }` no longer reports
  `issue-211: unknown receiver class for method toString`.
- [ ] The replacement diagnostic is source-spanned at the nullable receiver
  and states that the value may be null.
- [ ] `narrowingTruthyObject.ts` no longer reports issue-211 for the first
  `x.toString()` at `122..134`.
- [ ] Any later truthiness-guarded `x.toString()` or `x.hasOwnProperty(...)`
  receiver blocker is recorded here or split into a follow-up issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(narrow) or test(method) or test(object)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingTruthyObject.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTruthyObject.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Related but distinct:

- Issue 5451 owns number `toString(radix)` after `typeof` switch narrowing.
- Issue 5222 owns interface-typed erased-local method receivers.
- Issue 342 is a broad Object builtin umbrella and is not a narrow owner for
  this first nullable-receiver diagnostic.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
