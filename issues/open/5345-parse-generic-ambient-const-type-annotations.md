---
id: 5345
title: "Parse generic ambient const type annotations"
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

Erase nested generic TypeScript type annotations on declaration-only ambient
`const` declarations, such as `declare const authorPromise:
Promise<Result<Author, "NOT_FOUND_AUTHOR">>;`.

## Problem

`asyncYieldStarContextualType.ts` currently fails before reaching the async
generator body. The first blocker is the ambient declaration:

```ts
declare const authorPromise: Promise<Result<Author, "NOT_FOUND_AUTHOR">>;
```

The lexer reaches the declaration successfully, but ambient variable parsing
does not consume the nested generic type annotation and reports the generic
issue-400 boundary.

Problem: nested generic ambient const annotations are not erased as a complete
TypeScript type annotation before ambient declaration parsing resumes.

Current diagnostic:

```text
UnsupportedTypeScriptSyntax: issue-400: expected ambient variable declaration name at 338..345
```

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts
```

Source context:

```ts
declare const authorPromise: Promise<Result<Author, "NOT_FOUND_AUTHOR">>;
declare const mapper: <T>(result: Result<T, "NOT_FOUND_AUTHOR">) => Result<T, "NOT_FOUND_AUTHOR">;
declare const g: <T, U, V>() => AsyncGenerator<T, U, V>;
```

Compiler evidence observed 2026-05-07:

```text
tokens: ok through interface/type aliases and declare const authorPromise/mapper/g
ast: fail before AST with issue-400 expected ambient variable declaration name at 338..345
resolved: same parser failure
visible symbols: none before failure
TypeScript oracle: ok, diagnostics=[]
oracle hints: authorPromise has type Promise<Result<Author, "NOT_FOUND_AUTHOR">>, mapper is generic callable, g is generic AsyncGenerator factory
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedTypeScriptSyntax:1
unsupported_features=parser-syntax:1
semantic_enabled=0
```

## Desired final state

Declaration-only ambient `const` declarations erase nested generic type
annotations as a single TypeScript annotation. The representative file should
advance past the `authorPromise` declaration and expose the next narrower
parser, resolver, or async/generator diagnostic.

## Scope

In scope:

- [ ] Erase nested generic type references in ambient `declare const` annotations, including commas and string literal type arguments inside `<...>`.
- [ ] Erase generic callable ambient const annotations such as `<T>(result: Result<T, E>) => Result<T, E>`.
- [ ] Erase zero-argument generic callable annotations such as `<T, U, V>() => AsyncGenerator<T, U, V>`.
- [ ] Add focused parser coverage for `declare const p: Promise<Result<A, "E">>;`.
- [ ] Re-run `asyncYieldStarContextualType.ts` and record the next diagnostic.

Out of scope:

- Async generator runtime semantics and `yield*` lowering.
- Direct generic call expression parsing, tracked by `issues/done/5242-w2-completion-declaration.md`.
- ASI after ambient variable declarations, tracked by `issues/done/5193-parse-asi-after-ambient-variable-declarations.md`.
- Generic async generator declaration parsing already completed in `issues/done/5148-parse-generic-async-generator-declarations.md`.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- focused parser or CLI fixture

Do not touch:

- backend emit or runtime ABI
- async/generator lowering unless this parser blocker has already advanced

## Acceptance criteria

- [ ] `declare const authorPromise: Promise<Result<Author, "NOT_FOUND_AUTHOR">>;` no longer reports `expected ambient variable declaration name`.
- [ ] A focused parser test covers an ambient const annotation with nested generic type arguments and string literal type arguments.
- [ ] A focused parser test or reference triage covers an ambient const generic callable annotation.
- [ ] Ambient declaration initializers remain rejected.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts` advances past the current issue-400 diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend ambient
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts --detail --no-dashboard-data
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

Split from `issues/open/762-implement-asyncYieldStarContextualType.md`.

2026-05-07 fold-in:

- `issues/open/1509-implement-contextualTypeCaching.md` reaches the same
  ambient const generic callable annotation boundary for
  `declare const A: <T, P extends keyof T>(obj: T, prop: P, factory: () => T[P]) => void;`.
- Current diagnostic:
  `UnsupportedTypeScriptSyntax: issue-400: unterminated ambient variable declaration at 864..871`.
- TypeScript oracle accepts the declaration and reports `A` as
  `<T, P extends keyof T>(obj: T, prop: P, factory: () => T[P]) => void`.

Related but not duplicate:

- `issues/done/5193-parse-asi-after-ambient-variable-declarations.md` covers
  semicolon insertion after ambient declarations.
- `issues/done/5242-w2-completion-declaration.md`
  covers later runtime call syntax such as `object<T>()`.
- `issues/done/5148-parse-generic-async-generator-declarations.md` covers the
  async generator declaration parser shape after this ambient declaration
  blocker is cleared.

## Completion evidence

Fill when implemented.
