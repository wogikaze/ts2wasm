---
id: 5190
title: "Skip implements in ambient class declarations"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

`bluebirdStaticThis.ts` starts with `export declare class Promise<R> implements Promise.Thenable<R>`, and the parser stops at `implements` with `expected LeftBrace`.

## Problem

Normal class parsing already skips TypeScript `implements` clauses before the class body. The ambient `declare class` erasure path only handles `extends`, so it expects `{` immediately after the generic class name:

```ts
export declare class Promise<R> implements Promise.Thenable<R> {
    constructor(callback: (resolve: (value: R) => void) => void);
}
```

TypeScript accepts the heritage clause and reports later type/namespace diagnostics. The compiler stops before the ambient class body can be erased.

Problem: ambient class declaration parsing does not skip `implements` heritage clauses.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bluebirdStaticThis.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: expected LeftBrace, got Some(Ident("implements")) at 287..297
```

Compiler evidence:

- Tokens include `Export`, contextual `declare`, `Class`, generic parameter `<R>`, contextual `implements`, qualified name `Promise.Thenable<R>`, and `{`.
- AST/resolved construction fails before the ambient class body is skipped.
- Runtime class parsing already has `skip_class_implements`; the ambient declaration path does not use the equivalent behavior.

TypeScript oracle evidence:

```text
TS2420: Class 'Promise<R>' incorrectly implements interface 'Thenable<R>'.
```

The TypeScript AST identifies `implements Promise.Thenable<R>` as a class `HeritageClause`.

## Desired final state

The parser erases/skips `implements` clauses in ambient class declarations and advances to the class body. The representative file should no longer fail at the `implements` keyword.

## Scope

In scope:

- [ ] Skip one or more `implements` heritage types in `declare class` parsing.
- [ ] Support qualified names with generic type arguments in the skipped implements list.
- [ ] Add focused parser coverage for `declare class C<T> implements NS.I<T> {}`.

Out of scope:

- Type-checking class/interface conformance.
- Reporting TS2420 or namespace export diagnostics after parsing advances.
- Runtime class `implements` behavior, already skipped separately.
- Interface declaration semantics.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- `crates/ir/src/`
- backend/runtime lowering

## Acceptance criteria

- [ ] `parse_program("declare class C<T> implements NS.I<T> {}")` succeeds and erases the ambient class.
- [ ] Parser tests cover qualified generic implements clauses in ambient classes.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bluebirdStaticThis.ts` no longer reports `expected LeftBrace` at `implements`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bluebirdStaticThis.ts
```

Impacted commands:

```sh
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

Split from generated bucket `1080` on 2026-05-06. Broader Bluebird static `this` semantics and later namespace diagnostics remain outside this parser slice.

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
