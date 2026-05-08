---
id: 5218
title: "Support nested function closures capturing this"
type: feature
area: ir/runtime
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-07
---

## Summary

Support nested ordinary function expressions that capture `this` from an object
literal or contextual receiver when the function is lowered as a closure.

## Problem

Problem: `castTest.ts` parses and erases its angle-bracket type assertions, but
lowering rejects the object literal `add` function because the nested function
uses `this.x` and `this.y`.

Current diagnostic:

```text
UnsupportedRuntimeSubset: issue-062e: nested function `` closures with `this` or `arguments` are not supported in this slice
```

## Current failure

Use the validation command below to reproduce the representative failure.

Source shape:

```ts
var p_cast = <Point> ({
    x: 0,
    y: 0,
    add: function(dx, dy) {
        return new Point(this.x + dx, this.y + dy);
    },
    mult: function(p) { return p; }
})
```

Compiler evidence:

```text
tokens: ok
ast: ok; object literal property `add` is a FunctionExpr with Return(New(Point, [this.x + dx, this.y + dy]))
resolved/lowered: issue-062e nested function closure with `this` or `arguments`
TypeScript oracle: reports cast-overlap diagnostics for earlier null casts, but provides binding/type evidence for `p_cast: Point`
```

## Desired final state

The representative object literal function can lower when it captures `this`,
or the compiler emits a narrower source-spanned diagnostic that identifies the
unsupported `this` capture site instead of the generic closure boundary.

## Scope

In scope:

- [ ] Lowering: support or explicitly source-diagnose `this` capture in a nested ordinary function expression used as an object literal property.
- [ ] Runtime/ABI: preserve existing immutable closure capture behavior from issue 062e while deciding whether `this` is an extra captured value or a diagnostic-only boundary.
- [ ] Tests: add a focused fixture for `function(dx) { return this.x + dx; }` inside an object literal.
- [ ] Diagnostics: keep the diagnostic behavior for `arguments` capture covered if it remains unsupported.

Out of scope:

- Full JavaScript `this` binding parity for extracted methods.
- Async/generator closure semantics.
- Mutable closure environment support beyond the existing issue-062e boundaries.
- TypeScript cast-overlap diagnostics such as TS2352.

## Affected paths

Expected:

- `crates/ir/src/lowered/`
- `crates/backend-wasm/src/`
- `fixtures/`
- `crates/cli/tests/`

Do not touch:

- parser grammar unless a regression proves the AST shape changed
- unrelated TypeScript type-checking diagnostics

## Acceptance criteria

- [ ] `castTest.ts` no longer reports the generic `issue-062e` nested-function `this` closure guard as the first compiler blocker.
- [ ] A focused fixture covers an object literal property function that reads `this.x`.
- [ ] `contextualTypeShouldBeLiteral.ts` no longer reports the
  `issue-5179` implicit-this diagnostic for contextual object literal
  `method() { this; this.type; this.value; }`.
- [ ] A focused fixture covers a method-shorthand object literal property that
  reads `this` under a contextual object/interface type.
- [ ] Existing closure fixtures for immutable captures still pass.
- [ ] Remaining unsupported `arguments` capture behavior has a source-spanned diagnostic or explicit regression test.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir closure
cargo nextest run -p ts2wasm-cli -E 'test(closure) or test(object) or test(node_diff)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/castTest.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/castTest.ts --detail
```

Not run:

- none

## Notes

Split from generated bucket `issues/done/1121-implement-castTest.md`.
Issue `issues/open/5179-report-implicit-this-before-closure-runtime-guard.md`
is related but narrower: it handles a TypeScript implicit-`this` diagnostic
before the runtime guard. This issue tracks the runtime/diagnostic boundary for
an object literal function where TypeScript provides contextual member evidence.

Additional superseded buckets:

- `issues/done/1516-implement-contextualTypeShouldBeLiteral.md` reaches the
  same object-literal/contextual receiver `this` boundary for method shorthand:
  `method() { this; this.type; this.value; }`. Fresh triage on 2026-05-07
  reports `UnsupportedTypeScriptSyntax issue-5179` for `method`, while the
  TypeScript oracle accepts the file with diagnostics `[]`.
- `issues/done/1375-implement-commentsOnObjectLiteral-object-literal.md` reaches
  the same issue-062e boundary for an object-literal accessor:
  `get a() { return this.prop; }`. `commentsOnObjectLiteral4.ts` from the same
  bucket is already a build pass.
- `issues/done/3490-implement-nestedThisContainer.md` reaches the same
  issue-062e boundary for `foo.bar = function () { const self = this; }`;
  TypeScript accepts it with diagnostics `[]`.
- `issues/done/3558-implement-noImplicitThisBigThis.md` reaches the same
  issue-062e boundary for object-literal method shorthand returning `this`;
  TypeScript accepts the file with diagnostics `[]`.

## Completion Evidence

Fill when implemented.
