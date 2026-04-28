---
id: 059
title: "Implement parser syntax extensions for TypeScript and advanced JS"
type: feature
area: frontend
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-28
---

## Summary

Implement parser syntax extensions to handle parser-syntax feature gaps in reference tests.

Problem: Parser syntax work is an epic spanning independent TypeScript erasure and advanced JavaScript syntax families; direct selection hides the next smallest syntax slice.

Queue design note:

- This is an epic-level issue and must not be selected directly from the Ready queue.
- Use child syntax-family slices with a concrete source snippet, parser/unparse expectation, and focused reference coverage.
- Duplicate issue 065 should be merged into this epic rather than selected independently.

## Problem

Reference test results show 115 cases fail with parser-syntax diagnostic (test262:14, tsc:77, tsgo:24). The parser cannot handle various TypeScript and advanced JavaScript syntax constructs, preventing compilation of modern code.

## Desired final state

Parser supports common TypeScript and advanced JavaScript syntax constructs. parser-syntax diagnostic is only emitted for genuinely unsupported syntax.

## Scope

In scope:

- [ ] Add TypeScript type annotations to parser
- [ ] Add TypeScript interface declarations
- [ ] Add TypeScript generic syntax
- [ ] Add advanced JavaScript syntax (decorators, private fields, etc.)
- [ ] Update diagnostic to emit parser-syntax only when appropriate

Out of scope:

- [ ] Full TypeScript type checking (separate issue)
- [ ] TypeScript emit semantics (separate issue)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] Parser accepts common TypeScript syntax
- [ ] parser-syntax diagnostic significantly reduced in reference tests
- [ ] Regression test added for parser syntax
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 200
mise run reference-coverage -- tsc --limit 100
mise run reference-coverage -- tsgo --limit 50
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

Start with basic TypeScript type annotations before adding advanced features.

2026-04-29 superseded-reference merge note:

- Duplicate issue 065 has been closed as superseded by this parser syntax epic.
- Its affected-test evidence remains preserved here for child syntax-slice planning:
  Annex B String HTML wrapper methods (`big`, `blink`, `bold`, `fixed`, `italics`,
  `small`, `strike`, `sub`) and `String.prototype.substr` legacy cases, plus the
  remaining parser-syntax family represented by the original 52-case reference
  window.
- Treat those cases as child parser/runtime classification work instead of selecting
  issue 065 as a separate parent. The implemented TypeScript erasure slices below
  remain the current progress evidence; this merge note is issue-queue deduplication
  only.

2026-04-28 progress evidence:

- Implemented a narrow erasable TypeScript type-annotation parser slice for variable declarations, function parameter annotations, and return annotations.
- Uninitialized `let` / `var` declarations after optional type annotations now parse as `undefined`; uninitialized `const` declarations still report a diagnostic.
- Added fixture `fixtures/basics-types/type-annotation-erasure.ts`.
- Added CLI coverage showing `dump --ast --unparse` erases annotations and build accepts the fixture.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -p ts2wasm-frontend`
  - `cargo nextest run -p ts2wasm-cli --test dump_cli`
  - `cargo nextest run`
  - `mise run update-issue-index -- --check`
  - `mise run check issues`
  - `mise run check agent-state`
  - `mise run check`
- Issue 059 remains open. Interfaces, generics, decorators, private fields, broader parser-syntax diagnostic reduction, and reference-ramp evidence remain outside this slice.

2026-04-28 progress evidence (interface-erasure slice):

- Implemented a narrow parser-only TypeScript `interface` / `export interface` declaration erasure slice.
- Interface declarations are consumed before AST construction, so dump `--ast --unparse` and build output omit them while preserving subsequent runtime statements.
- Added fixture `fixtures/basics-types/interface-erasure.ts`.
- Added frontend parser coverage for erased interface declarations with members, methods, optional members, `extends`, and nested type-literal braces.
- Added CLI coverage showing dump unparse erases interface declarations and build accepts the fixture.
- Validation passed:
  - `cargo test -p ts2wasm-frontend parses_typescript_interface_declarations_as_erased_syntax -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli dump_ast_unparse_erases_typescript_interface_declarations -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli build_accepts_erasable_typescript_interface_declarations -- --nocapture`
  - `cargo fmt --all --check`
  - `mise run fmt`
  - `cargo nextest run -p ts2wasm-frontend`
  - `cargo nextest run -p ts2wasm-cli --test dump_cli`
  - `mise run update-issue-index -- --check`
  - `mise run check agent-state`
- Validation not clean due unrelated pre-existing local-report references:
  - `mise run check issues` failed because issue 052 and done issue 228 reference missing `reports/runs/...` paths. `reports/` is local/gitignored and those issue files are outside this assignment.
  - `mise run check` failed at the same issue health step after shell syntax checks passed.
- Parent validation note: after syncing the referenced local `reports/runs/...` artifacts into the merge-review worktree, `mise run check issues` and `mise run check` passed.
- Issue 059 remains open. Type aliases, generics, decorators, private fields, broader parser-syntax diagnostic reduction, and reference-ramp evidence remain outside this slice.

2026-04-28 progress evidence (type-alias-erasure slice):

- Implemented a narrow parser-only TypeScript `type` / `export type` alias declaration erasure slice.
- Type alias declarations are consumed before AST construction, so dump `--ast --unparse` and build output omit aliases while preserving subsequent runtime statements.
- Alias bodies are skipped with balanced parentheses, brackets, and braces, covering simple aliases plus object/function type bodies.
- Added fixture `fixtures/basics-types/type-alias-erasure.ts`.
- Added frontend parser coverage for erased type aliases with nested object type braces and function type members.
- Added CLI coverage showing dump unparse erases type alias declarations and build accepts the fixture.
- Validation passed:
  - `cargo test -p ts2wasm-frontend parses_typescript_type_alias_declarations_as_erased_syntax -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli dump_ast_unparse_erases_typescript_type_alias_declarations -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli build_accepts_erasable_typescript_type_alias_declarations -- --nocapture`
  - `cargo fmt --all --check`
  - `mise run fmt`
  - `cargo nextest run -p ts2wasm-frontend`
  - `cargo nextest run -p ts2wasm-cli --test dump_cli`
  - `mise run update-issue-index -- --check`
  - `mise run check agent-state`
- Validation not clean due unrelated pre-existing local-report references:
  - `mise run check issues` failed because issue 052 and done issue 228 reference missing `reports/runs/...` paths. `reports/` is local/gitignored and those issue files are outside this assignment.
  - `mise run check` failed at the same issue health step after shell syntax checks passed.
- Parent validation note: after syncing the referenced local `reports/runs/...` artifacts into the merge-review worktree, `mise run check issues` and `mise run check` passed.
- Issue 059 remains open. Generics, decorators, private fields, broader parser-syntax diagnostic reduction, and reference-ramp evidence remain outside this slice.

2026-04-28 progress evidence (generic-erasure slice):

- Implemented a narrow parser-only TypeScript generic erasure slice for simple generic function declarations and directly attached simple generic call type arguments.
- Generic declarations such as `function id<T>(value: T): T { return value; }` parse as ordinary runtime functions, and generic calls such as `id<number>(3)` parse as ordinary runtime calls before AST/lowering.
- Added fixture `fixtures/basics-types/generic-erasure.ts`.
- Added frontend parser coverage for erased generic function declarations and generic call type arguments.
- Added CLI coverage showing dump unparse erases generic syntax and build accepts the fixture.
- Validation passed:
  - `cargo test -p ts2wasm-frontend parses_typescript_generic_functions_and_calls_as_erased_syntax -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli dump_ast_unparse_erases_typescript_generics -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli build_accepts_erasable_typescript_generics -- --nocapture`
  - `cargo fmt --all --check`
  - `mise run fmt`
  - `cargo nextest run -p ts2wasm-frontend`
  - `cargo nextest run -p ts2wasm-cli --test dump_cli`
  - `cargo run -q -p ts2wasm-cli -- dump --ast --unparse fixtures/basics-types/generic-erasure.ts`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/generic-erasure.ts -o /tmp/ts2wasm-059-generic-erasure.wasm`
  - `iwasm /tmp/ts2wasm-059-generic-erasure.wasm` (stdout: `3`, `7`)
  - `mise run update-issue-index -- --check`
  - `mise run check agent-state`
  - `cargo nextest run`
- Validation not clean due unrelated pre-existing local-report references:
  - `mise run check issues` failed because issue 052 and done issue 228 reference missing `reports/runs/...` paths. `reports/` is local/gitignored and those issue files are outside this assignment.
  - `mise run check` failed at the same issue health step after shell syntax checks passed.
- Parent review tightened the generic call erasure guard so call type arguments are only erased for function names declared with TypeScript generic parameters in the current parser run, avoiding a regression where `a<b>(c)` could be misread as a generic call instead of adjacent relational comparisons.
- Parent added regression coverage `preserves_adjacent_relational_expression_that_resembles_generic_call` and validated:
  - `cargo fmt --all --check`
  - `cargo nextest run -p ts2wasm-frontend`
  - `cargo nextest run -p ts2wasm-cli --test dump_cli`
  - direct dump for `let a = 1; let b = 2; let c = 3; console.log(a<b>(c));`, which unparses as `console.log(((a < b) > c));`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/generic-erasure.ts -o /tmp/ts2wasm-059-generic-erasure.parent2.wasm && iwasm /tmp/ts2wasm-059-generic-erasure.parent2.wasm`
  - `mise run check issues`
  - `mise run check`
- Issue 059 remains open. Decorators, private fields, broader parser-syntax diagnostic reduction, and reference-ramp evidence remain outside this slice.

2026-04-28 progress evidence (as-assertion-erasure slice):

- Implemented a narrow parser-only TypeScript `as` assertion erasure slice for expressions such as `3 as number`, `({ x: value } as { x: number })`, and chained assertions such as `[value] as number[] as unknown`.
- `as` assertions are consumed before AST/lowering, so dump `--ast --unparse` and build output preserve only the runtime expression.
- Added fixture `fixtures/basics-types/as-assertion-erasure.ts`.
- Added frontend parser coverage for erased `as` assertions with primitive, object-literal, array, and chained type forms.
- Added CLI coverage showing dump unparse erases `as` assertions and build accepts the fixture.
- Validation passed:
  - pre-change reproducer before implementation: `cargo run -q -p ts2wasm-cli -- dump --ast --unparse /tmp/ts2wasm-059-as-erasure-probe.ts` failed with `expected Semicolon, got Some(Ident("as"))`
  - `cargo test -p ts2wasm-frontend parses_typescript_as_assertions_as_erased_syntax -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli dump_ast_unparse_erases_typescript_as_assertions -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli build_accepts_erasable_typescript_as_assertions -- --nocapture`
  - `cargo fmt --all --check`
  - `cargo nextest run -p ts2wasm-frontend`
  - `cargo nextest run -p ts2wasm-cli --test dump_cli`
  - post-change reproducer: `cargo run -q -p ts2wasm-cli -- dump --ast --unparse /tmp/ts2wasm-059-as-erasure-probe.ts`
  - `cargo run -q -p ts2wasm-cli -- dump --ast --unparse fixtures/basics-types/as-assertion-erasure.ts`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/as-assertion-erasure.ts -o /tmp/ts2wasm-059-as-erasure.wasm`
  - `iwasm /tmp/ts2wasm-059-as-erasure.wasm` (stdout: `3`, `3`, `3`)
- Parent validation note: after rebasing onto master `2c7d09d` and syncing local report artifacts, validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -p ts2wasm-frontend`
  - `cargo nextest run -p ts2wasm-cli --test dump_cli`
  - `cargo run -q -p ts2wasm-cli -- dump --ast --unparse fixtures/basics-types/as-assertion-erasure.ts`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/as-assertion-erasure.ts -o /tmp/ts2wasm-059-as-erasure.parent.wasm`
  - `iwasm /tmp/ts2wasm-059-as-erasure.parent.wasm`
  - `mise run update-issue-index -- --check`
  - `mise run check agent-state`
  - `mise run check issues`
  - `mise run check`
- Issue 059 remains open. Decorators, private fields, broader parser-syntax diagnostic reduction, and reference-ramp evidence remain outside this slice.

2026-04-28 progress evidence (satisfies-erasure slice):

- Implemented a narrow parser-only TypeScript `satisfies` expression erasure slice for expressions such as `expr satisfies Type`.
- `satisfies` type operands are consumed before AST/lowering, so dump `--ast --unparse` and build output preserve only the runtime expression.
- Added fixture `fixtures/basics-types/satisfies-erasure.ts`.
- Added frontend parser coverage for erased `satisfies` expressions with object type operands and chained existing `as` erasure.
- Added CLI coverage showing dump unparse erases `satisfies` expressions and build accepts the fixture.
- Validation passed:
  - pre-change reproducer before implementation: `cargo run -q -p ts2wasm-cli -- dump --ast --unparse /tmp/ts2wasm-059-satisfies-erasure-probe.ts` failed with `expected Semicolon, got Some(Ident("satisfies"))`
  - `cargo test -p ts2wasm-frontend parses_typescript_satisfies_expressions_as_erased_syntax -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli dump_ast_unparse_erases_typescript_satisfies_expressions -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli build_accepts_erasable_typescript_satisfies_expressions -- --nocapture`
  - `cargo fmt --all --check`
  - `cargo nextest run -p ts2wasm-frontend`
  - `cargo nextest run -p ts2wasm-cli --test dump_cli`
  - `cargo run -q -p ts2wasm-cli -- dump --ast --unparse fixtures/basics-types/satisfies-erasure.ts`
  - `cargo run -q -p ts2wasm-cli -- dump --ast --unparse /tmp/ts2wasm-059-satisfies-erasure-probe.ts`
  - `cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/satisfies-erasure.ts -o /tmp/ts2wasm-059-satisfies-erasure.wasm`
  - `iwasm /tmp/ts2wasm-059-satisfies-erasure.wasm` (stdout: `3`)
  - `mise run update-issue-index -- --check`
  - `mise run check agent-state`
  - `cargo nextest run`
- Validation not clean due unrelated pre-existing local-report references:
  - `mise run check issues` failed because issue 052 and done issue 228 reference missing `reports/runs/...` paths. `reports/` is local/gitignored and those issue files are outside this assignment.
  - `mise run check` failed at the same `check issues` step after shell syntax checks passed.
- Issue 059 remains open. Decorators, private fields, broader parser-syntax diagnostic reduction, and reference-ramp evidence remain outside this slice.

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
