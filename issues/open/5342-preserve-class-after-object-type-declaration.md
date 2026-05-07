---
id: 5342
title: "Preserve class after object type declaration"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Parse a TypeScript-erased object type `var` declaration when the next runtime
declaration is a class.

## Problem

`collisionThisExpressionAndNameResolution.ts` tokenizes successfully, then
reports `UnsupportedTypeScriptSyntax: unterminated TypeScript type annotation
at 440..441` while parsing this shape:

```ts
var console: {
    log(message: any);
}
class Foo {
    x() {
        var _this = 10;
        function inner() {
            console.log(_this);
            return x => this;
        }
    }
}
```

Problem: `var name: { method(param: Type); }` is not terminated before a
following `class` declaration, so the parser never reaches the class body or
the later lexical `this` behavior.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndNameResolution.ts
```

Observed 2026-05-07:

```text
Smart triage: Triage parser syntax: collisionThisExpressionAndNameResolution
Diagnostic: UnsupportedTypeScriptSyntax / unsupported-feature-boundary
Message: unterminated TypeScript type annotation at 440..441
Failure location: line 15, column 2, at the final class closing brace
tokens: ok through typed var, class Foo, method x, local _this, inner function, console.log(_this), return x => this
ast/resolved: fail with unterminated TypeScript type annotation
```

TypeScript oracle parses the file and reports:

```text
TS2403: duplicate global console declaration at line 4
TS2683: implicit any this at line 12 in `return x => this`
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndNameResolution.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=module-resolution:1
semantic_enabled=0
```

## Desired final state

The parser erases the object type annotation and preserves the following
`class Foo { ... }` declaration as a separate runtime declaration.

## Scope

In scope:

- [ ] Preserve a following class declaration after `var typed: { ... }`.
- [ ] Add a focused parser regression for `var typed: { m(x: any); }\nclass Next {}`.
- [ ] Re-run the reference triage and record the next diagnostic.

Out of scope:

- Full TypeScript structural type support.
- Duplicate global `console` compatibility diagnostics.
- Lexical arrow `this` behavior after parsing advances.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- backend emit or runtime ABI
- unrelated resolver/lowering call semantics

## Acceptance criteria

- [ ] `collisionThisExpressionAndNameResolution.ts` no longer reports `unterminated TypeScript type annotation` at `440..441`.
- [ ] A focused parser regression proves `class Next {}` is preserved after an object type annotation declaration.
- [ ] Existing parser behavior for the following-var and following-function cases remains covered by issues 5339 and 5340.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend parser
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndNameResolution.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndNameResolution.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from `issues/open/1332-implement-collisionThisExpressionAndNameResolution.md`.

Related but not duplicate:

- `issues/open/5339-preserve-var-after-object-type-declaration.md` covers a following `var` declaration.
- `issues/open/5340-preserve-function-after-object-type-declaration.md` covers a following `function` declaration.

The current failure happens before the class body is parsed, so this issue does
not own the later `return x => this` lexical receiver behavior.

## Completion evidence

Fill when implemented.
