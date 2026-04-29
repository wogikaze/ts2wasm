---
id: 251
title: "Implement destructuring binding runtime semantics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: ["247"]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement binding semantics for array and object destructuring patterns now that issue 247 records the parser-level pattern surface.

Problem: Parsed destructuring binding patterns are accepted for AST/dump coverage, but name resolution, lowering, and runtime binding still treat declarations and parameters as simple identifier names.

## Current failure

Parser-only commands accept binding patterns:

```sh
cargo run -q -p ts2wasm-cli -- dump --ast --unparse /tmp/destructuring.ts
```

Compilation/lowering does not safely bind destructured names such as `a` and `x` from:

```ts
let [a, b] = arr;
let { x } = obj;
function f([value]) { return value; }
```

## Desired final state

Destructuring declarations and parameters bind their target names with ECMAScript-compatible observable behavior for the supported array/object runtime subset.

## Scope

In scope:

- [x] Resolve names introduced by supported array/object binding patterns.
- [x] Lower declaration patterns for the supported dense-array and object-property subset.
- [x] Lower ordinary function and arrow parameter patterns for the supported call subset.
- [ ] Implement defaults, nested patterns, elisions, and rest binding where runtime support is available.
- [x] Emit issue-linked diagnostics for unsupported iterator/property semantics that remain outside the runtime subset.

Out of scope:

- Destructuring assignment expressions such as `({ x } = obj)`.
- `for-in` / `for-of` destructuring heads.
- Broad iterator protocol implementation beyond the supported runtime subset.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/compiler/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`

Do not touch:

- unrelated runtime builtins
- broad iterator protocol implementation unless explicitly split into this issue

## Acceptance criteria

- [x] `let [a, b] = arr; console.log(a);` binds `a` and `b` from supported arrays.
- [x] `let { x } = obj; console.log(x);` binds `x` from supported objects.
- [x] `function f([a], { x }) { return a + x; }` and `let f = ([a]) => a;` bind parameter patterns.
- [x] Unsupported destructuring forms report issue-linked diagnostics instead of compiling to incorrect bindings.
- [x] Node/iwasm differential fixtures cover supported declaration and parameter patterns.
- [x] `docs/language-reference/javascript-features.md` is updated from parser-only to runtime-supported status for covered forms.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/destructuring/binding/ --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md` if runtime support changes

Follow-up issues:

- [x] split: `issues/done/252-implement-destructuring-assignment-pattern-parser.md`

## Notes

Issue 247 intentionally accepted parser/AST-level binding patterns without claiming runtime semantics.

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

## Progress evidence

2026-04-29:

- Implemented a mergeable runtime subset for binding patterns:
  - declaration array bindings such as `let [a, b] = arr`
  - declaration object shorthand/identifier alias bindings such as `let { x, y: renamed } = obj`
  - ordinary function parameter patterns such as `function f([a], { x }) { ... }`
  - arrow parameter patterns such as `let f = ([a]) => a`
- Added `fixtures/core-semantics/destructuring-binding-runtime.ts` and `destructuring_binding_runtime_fixture_matches_node_output_under_iwasm`.
- Added `fixtures/core-semantics/destructuring-binding-unsupported.ts` and `destructuring_binding_unsupported_forms_report_issue_251` for unsupported default binding diagnostics.
- Remaining runtime work: defaults, nested patterns, elisions, rest binding, and broad iterator semantics.
- Focused validation passed: `cargo nextest run -p ts2wasm-cli --test m2_node_diff destructuring_binding` (2 passed).
- Required validation passed: `cargo fmt --all --check`; `cargo nextest run -E 'test(destructuring) or test(node_diff)'` (10 passed); `cargo nextest run` (489 passed, 4 skipped); `mise run update-issue-index -- --check`; `mise run check issues`.

Parent-review follow-up:

- Carried `ResolvedParam` source spans into issue-251 diagnostics for unsupported rest/defaulted parameter binding patterns.
- Added `fixtures/core-semantics/destructuring-binding-param-default-unsupported.ts` and `fixtures/core-semantics/destructuring-binding-param-rest-unsupported.ts`; the CLI negative fixture helper asserts issue-251 diagnostics include source span text.
- Required validation passed: `cargo fmt --all --check`; `cargo nextest run -E 'test(destructuring) or test(node_diff)'` (10 passed); `cargo nextest run` (489 passed, 4 skipped); `mise run update-issue-index -- --check`; `mise run check issues`.

Defaults follow-up:

- Implemented literal default initializers for simple identifier-only array/object binding elements and ordinary function whole-pattern parameter defaults.
- Added `fixtures/core-semantics/destructuring-binding-defaults-runtime.ts` and Node/iwasm differential coverage.
- Kept non-literal defaults, rest, nested patterns, elisions, and broad iterator semantics as issue-251 unsupported diagnostics; updated unsupported fixtures to assert source-spanned non-literal default diagnostics.
- Required validation passed: `cargo fmt --all --check`; `cargo nextest run -E 'test(destructuring) or test(node_diff)'` (11 passed); `mise run update-issue-index -- --check`; `mise run check issues`; `cargo nextest run` (496 passed, 4 skipped).

Elision follow-up:

- Implemented array binding elisions for simple identifier-only declaration and parameter patterns while preserving later element indexes.
- Added `fixtures/core-semantics/destructuring-binding-elision-runtime.ts` and Node/iwasm differential coverage for declaration, ordinary function parameter, and arrow parameter elisions.
- Rest, nested patterns, non-literal defaults, and broad iterator semantics remain issue-251 unsupported diagnostics.
- Required validation passed: `cargo fmt --all --check`; `cargo nextest run -E 'test(destructuring) or test(node_diff)'` (12 passed); `mise run check issues`; `cargo nextest run` (501 passed, 4 skipped).

Array rest follow-up:

- Implemented simple array rest binding for dense-array declaration and parameter patterns by lowering to the existing `ArraySlice` runtime helper.
- Added `fixtures/core-semantics/destructuring-binding-rest-runtime.ts` and Node/iwasm differential coverage for declarations, ordinary function parameters, and arrow parameters.
- Added `fixtures/core-semantics/destructuring-binding-object-rest-unsupported.ts`; object rest remains a source-spanned issue-251 unsupported diagnostic.
- Required validation passed: `cargo fmt --all --check`; `cargo nextest run -E 'test(destructuring) or test(node_diff)'` (13 passed); `mise run check issues`; `cargo nextest run` (503 passed, 4 skipped).

Nested array follow-up:

- Implemented nested array binding patterns for dense-array declaration and parameter patterns by making binding targets recursive and reusing existing array element lowering.
- Added `fixtures/core-semantics/destructuring-binding-nested-runtime.ts` and Node/iwasm differential coverage for declaration, ordinary function parameter, and arrow parameter nested arrays.
- Added `fixtures/core-semantics/destructuring-binding-nested-object-unsupported.ts`; nested object binding remains a source-spanned issue-251 unsupported diagnostic.
- Required validation passed: `cargo fmt --all --check`; `cargo nextest run -E 'test(destructuring) or test(node_diff)'` (14 passed); `mise run update-issue-index -- --check`; `mise run check issues`; `cargo nextest run` (506 passed, 4 skipped).
