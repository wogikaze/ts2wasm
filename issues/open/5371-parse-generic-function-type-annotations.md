---
id: 5371
title: "Parse generic function type annotations"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Parse and erase generic TypeScript function type annotations such as
`<T, S>(f: (_: T) => S) => <U>(g: (_: U) => T) => (_: U) => S`.

## Problem

`contextualSignatureInstantiation2.ts` fails before the compiler can reach the
intended contextual signature behavior. The parser treats the generic type
parameter list in a variable annotation as statement syntax and stops at the
closing `>`.

Problem: generic function type annotations in variable declarations are not
erased as a complete TypeScript annotation.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualSignatureInstantiation2.ts
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstantiation2.ts --detail --no-dashboard-data
```

Current diagnostic:

```text
UnsupportedSyntax: expected Semicolon, got Some(Greater) at 58..59
line 3, column 17
```

Representative source:

```ts
var dot: <T, S>(f: (_: T) => S) => <U>(g: (_: U) => T) => (_: U) => S;
dot = <T, S>(f: (_: T) => S) => <U>(g: (_: U) => T): (r:U) => S => (x) => f(g(x));
```

Compiler evidence:

```text
tokens: ok through `<T, S>`, function-typed parameters, nested `<U>`, and arrows
ast/resolved: fail at the closing `>` in the generic function type annotation
visible symbols before failure: binding `dot`
TypeScript oracle: accepts FunctionType nodes and only reports later TS2454 for `id`
```

## Desired final state

The parser consumes generic function type annotations as erasable TypeScript
syntax and lets the representative file advance to the next semantic or
definite-assignment diagnostic.

## Scope

In scope:

- [ ] Erase variable type annotations that start with a generic function type.
- [ ] Support nested function type return annotations such as `=> <U>(...) => ...`.
- [ ] Preserve parser behavior for generic arrow function expressions in value position.
- [ ] Add focused parser or CLI coverage for a `var f: <T>(x: T) => T;` annotation.

Out of scope:

- Type checking generic function types.
- Contextual signature instantiation semantics after parsing succeeds.
- Generic arrow function expressions in assignment value position, tracked by
  `issues/open/5304-parse-generic-arrow-functions-with-typed-parameters.md`.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- focused parser or CLI fixture

Do not touch:

- backend emit or runtime ABI
- contextual typing semantics unless fresh triage proves parsing already advanced

## Acceptance criteria

- [ ] `contextualSignatureInstantiation2.ts` no longer reports
  `expected Semicolon, got Some(Greater)` at the generic function type annotation.
- [ ] A focused test accepts `var f: <T>(x: T) => T;`.
- [ ] A focused test or reference triage covers a nested function type return.
- [ ] If the file advances to TS2454 or another semantic blocker, record that
  blocker separately.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualSignatureInstantiation2.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstantiation2.ts --detail --no-dashboard-data
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

Split from `issues/done/1501-implement-contextualSignatureInstantiation-parser-syntax.md`.

Related but not duplicates:

- `issues/open/5304-parse-generic-arrow-functions-with-typed-parameters.md`
  covers generic arrow expressions in value position.
- `issues/open/5345-parse-generic-ambient-const-type-annotations.md` covers
  generic callable annotations on `declare const`, not ordinary local variables.

## Completion evidence

Fill only when implemented.
