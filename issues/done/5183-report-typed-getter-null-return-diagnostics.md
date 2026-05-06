---
id: 5183
title: "Report typed getter null return diagnostics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

`accessorsEmit.ts` now parses class getters and builds successfully, but TypeScript reports `TS2322` for returning `null` from a getter annotated `: Result`.

## Problem

The parser currently represents `get Property(): Result { ... }` as a class method named `get Property`, with the body preserved but no return-type diagnostic. The compiler then returns `BuildPass` even though TypeScript reports an error at the `return null;` statement.

Problem: typed class getter return annotations are erased before return-expression diagnostics are checked.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorsEmit.ts
```

Current compiler result:

```text
BuildPass: ts2wasm build succeeded
```

Representative source:

```ts
class Result { }

class Test {
    get Property(): Result {
        var x = 1;
        return null;
    }
}
```

Compiler evidence:

- Tokens include `get`, `Property`, `:`, return type `Result`, and `return null`.
- AST construction succeeds and records a `Function` named `get Property` in the class body.
- Resolved output records a `ClassMethod` named `get Property` with `Return(Null)`.
- The return type annotation is not reflected in the resolved output, so the case build-passes.

TypeScript oracle evidence:

```text
TS2322: Type 'null' is not assignable to type 'Result'.
```

The oracle reports the diagnostic at the `return` statement in the first getter. The unannotated getter in `Test2` does not produce this diagnostic.

## Desired final state

The frontend preserves enough getter return type information to report the representative typed getter `return null` diagnostic instead of silently build-passing.

## Scope

In scope:

- [x] Preserve class getter return type annotations for the focused `get Property(): Result` pattern.
- [x] Report a source-spanned diagnostic when a typed getter returning a class type directly returns `null`.
- [x] Keep unannotated getters such as `get Property() { return null; }` outside this diagnostic.
- [x] Add focused parser/frontend or compiler coverage for the annotated getter and unannotated sibling.
- [x] Re-run representative triage and confirm it no longer reports `BuildPass` for this hidden `TS2322`.

Out of scope:

- Full class accessor runtime emit.
- Setter semantics.
- General TypeScript assignability beyond direct `null` return to a class return type.
- Property descriptor emission for getters.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/ast.rs`
- `crates/compiler/src/dump.rs`
- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/builtin_resolved.rs`
- focused tests/fixtures for class getter return diagnostics

Do not touch:

- Backend property descriptor/runtime emission unless triage advances past this diagnostic and proves runtime accessor emit work is the next blocker.
- Private accessor runtime code.

## Acceptance criteria

- [x] A focused test covers `class Result {} class Test { get Property(): Result { return null; } }`.
- [x] The diagnostic is source-spanned at the `return null` statement or `null` expression.
- [x] A sibling unannotated getter `get Property() { return null; }` does not report this typed-return diagnostic.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorsEmit.ts` no longer reports `BuildPass` while TypeScript reports `TS2322`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend accessor
cargo nextest run -p ts2wasm-ir class
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorsEmit.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from generated bucket `107` on 2026-05-06 after fresh triage showed the original accessor parser blocker was stale and the current mismatch is a false build-pass.

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

- Later triage may expose actual accessor runtime emit or property descriptor semantics after this typed-return diagnostic is preserved.


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
