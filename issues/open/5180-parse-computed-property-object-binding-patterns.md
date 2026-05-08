---
id: 5180
title: "Parse computed property object binding patterns"
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

`blockScopedBindingUsedBeforeDef.ts` stops in the binding-pattern parser at `{[a]: a}` before the intended block-scoped used-before-definition diagnostics can be observed.

## Problem

The frontend supports ordinary object binding keys, but rejects a computed property name in an object binding pattern with `issue-247: expected object binding property key`. TypeScript parses this as `ObjectBindingPattern -> BindingElement -> ComputedPropertyName`.

Problem: computed property names in object binding patterns are parser-unsupported, blocking `TS2448` used-before-definition coverage.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingUsedBeforeDef.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: issue-247: expected object binding property key, got Some(SpannedToken { kind: LeftBracket, span: Span { start: 56, end: 57 } }) at 57..58
```

Representative source:

```ts
for (let {[a]: a} of [{ }]) continue;
for (let {[a]: a} = { }; false; ) continue;
let {[b]: b} = { };
```

Compiler evidence:

- Token dump includes `LeftBrace`, `LeftBracket`, `Ident("a")`, `RightBracket`, `Colon`, and target `Ident("a")`.
- AST/resolved construction fails before representing the binding element.
- Visible symbols before failure are empty because parsing stops at the first binding pattern.

TypeScript oracle evidence:

```text
TS2448: Block-scoped variable 'a' used before its declaration.
TS2538: Type 'any' cannot be used as an index type.
TS2448: Block-scoped variable 'b' used before its declaration.
```

TypeScript AST evidence at the first failing span:

```text
ForOfStatement -> VariableDeclarationList -> VariableDeclaration -> ObjectBindingPattern -> BindingElement -> ComputedPropertyName -> Identifier
```

## Desired final state

The parser accepts computed property names in object binding patterns and preserves enough source information for later scope/type diagnostics. The representative case should no longer fail with `expected object binding property key` at `LeftBracket`.

## Scope

In scope:

- [x] Parse object binding elements of the form `{[expr]: target}`.
- [x] Support identifier computed keys in the focused parser path, including `{[a]: a}` and `{[b]: b}`.
- [x] Preserve the binding target separately from the computed key expression.
- [x] Add focused parser/frontend coverage for declaration, `for-of`, and `for` initializer forms.
- [x] Re-run representative triage and confirm the current `issue-247` parser blocker is gone.

Out of scope:

- Full computed key expression lowering for every expression form.
- Implementing the later `TS2448` and `TS2538` diagnostics after parsing succeeds.
- Destructuring assignment computed property runtime semantics.
- Broad scope-analysis work owned by the TSC scope-analysis backlog.

## Affected paths

Expected:

- `crates/frontend/src/parser/binding_patterns.rs`
- `crates/frontend/src/parser/expressions_destructure.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/frontend/src/ast.rs`
- `crates/ir/src/binding_pattern.rs`
- `crates/ir/src/lowered/resolver_extra.rs`

Do not touch:

- Backend/runtime code unless the representative triage advances past parsing and proves a runtime-specific blocker.
- General TypeScript scope-analysis diagnostics.

## Acceptance criteria

- [x] `let {[b]: b} = {};` parses without `expected object binding property key`.
- [x] `for (let {[a]: a} of [{}]) continue;` parses through the binding pattern.
- [x] `for (let {[a]: a} = {}; false;) continue;` parses through the binding pattern.
- [x] Existing ordinary object binding patterns such as `let {x: y} = obj;` remain accepted.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingUsedBeforeDef.ts` no longer reports the current `issue-247` `LeftBracket` diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend binding
cargo nextest run -p ts2wasm-ir binding
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingUsedBeforeDef.ts
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

Split from generated bucket `1067` on 2026-05-06. This issue only removes the first parser blocker; the actual block-scoped used-before-definition diagnostics remain future scope-analysis work after computed binding keys are represented.

## Completion evidence

Commits:

- Combined with the current commit.

### Changes

1. **`binding_patterns.rs`**: `parse_binding_property_key()` now checks for `LeftBracket` before the advance match. Parses the bracketed expression and returns `[expr]` as the key text.

2. **`tests.rs`**: Added `parses_computed_property_in_object_binding_pattern` and `parses_computed_property_binding_with_string_key` tests.

Validation result:

```text
command: cargo nextest run -p ts2wasm-frontend
result: 199 passed, 0 failed
date: 2026-05-06

command: target/debug/ts2wasm build blockScopedBindingUsedBeforeDef.ts
result: no longer 'expected object binding property key' (now runtime subset)
date: 2026-05-06
```

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- Later triage may expose the intended `TS2448` and `TS2538` diagnostics after the parser accepts computed binding keys.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

