---
id: 5338
title: "Support rest constructor outer local captures"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Allow class constructors with rest parameters to reference outer local
bindings, or report a narrower accepted diagnostic once the call ABI can carry
both rest arguments and hidden capture parameters.

## Problem

`collisionRestParameterUnderscoreIUsage.ts` parses and resolves the class
shape, then stops with:

```text
issue-289: class method `constructor` captures outer local `_i` with a rest parameter; hidden capture parameters after rest require a broader call ABI
```

Problem: constructor rest parameters and hidden lexical-capture parameters
cannot currently coexist in the lowered call ABI.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterUnderscoreIUsage.ts
```

Representative source:

```ts
var _i = "This is what I'd expect to see";
class Foo {
    constructor(...args: any[]) {
        console.log(_i);
    }
}
new Foo();
```

Compiler evidence:

```text
tokens: ok
ast: ok; ClassDecl Foo has constructor rest parameter args and body call console.log(_i)
resolved: UnsupportedSyntax issue-289 at _i
```

## Desired final state

The representative reference advances past the current issue-289 rest
constructor capture blocker.

## Scope

In scope:

- [ ] Represent constructor rest parameters and hidden outer-local captures without ABI collision.
- [ ] Preserve existing rest constructor calls without captures.
- [ ] Preserve existing issue-289 diagnostics for unsupported non-rest constructor capture shapes.

Out of scope:

- General constructor callback capture behavior, tracked by `issues/done/5152-support-class-constructor-outer-callback-captures.md`.
- Later TypeScript `console` redeclaration diagnostics.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- focused lowering/runtime fixtures

Do not touch:

- unrelated parser syntax
- object type literal rest signature parsing

## Acceptance criteria

- [ ] `collisionRestParameterUnderscoreIUsage.ts` no longer reports issue-289 for `_i` captured from a rest-parameter constructor.
- [ ] A focused fixture covers `class C { constructor(...args: any[]) { use(outer); } }`.
- [ ] Existing constructor rest parameter fixtures still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(class) or test(constructor) or test(rest)'
cargo nextest run -p ts2wasm-cli -E 'test(class) or test(constructor) or test(rest)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterUnderscoreIUsage.ts
```

Impacted commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from
`issues/done/1306-implement-collisionRestParameterUnderscoreIUsage.md`.

## Completion evidence

Fill only when implemented.
