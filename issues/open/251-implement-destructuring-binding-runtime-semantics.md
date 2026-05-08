---
id: 251
title: "Implement destructuring binding runtime semantics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: ["247"]
blocks: []
status: done
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
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
- [x] Implement defaults, nested patterns, elisions, and rest binding where runtime support is available.
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

- [x] split: `issues/open/252-implement-destructuring-assignment-pattern-parser.md`

## Notes

Issue 247 intentionally accepted parser/AST-level binding patterns without claiming runtime semantics.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- branch HEAD for issue 251 close-review

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -E 'test(destructuring) or test(node_diff)'
result: pass (16 passed)
date: 2026-04-29

command: cargo nextest run
result: pass (510 passed, 4 skipped)
date: 2026-04-29

command: mise run update-issue-index
result: pass
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-29

command: mise run check issues
result: pass
date: 2026-04-29
```

Remaining risks:

- Dynamic-source and parameter object rest, non-literal default initializers, broad iterator/property enumeration semantics, destructuring assignment expressions, and `for-in` / `for-of` destructuring heads remain out of scope for issue 251. The in-scope unsupported forms continue to emit source-spanned issue-251 diagnostics instead of compiling to incorrect bindings.

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
- Added nested object unsupported coverage at the time; this boundary was later replaced by nested object runtime coverage.
- Required validation passed: `cargo fmt --all --check`; `cargo nextest run -E 'test(destructuring) or test(node_diff)'` (14 passed); `mise run update-issue-index -- --check`; `mise run check issues`; `cargo nextest run` (506 passed, 4 skipped).

Object rest follow-up:

- Implemented object rest binding for declaration patterns whose source is a static object literal by materializing the rest object from non-excluded literal keys.
- Added `fixtures/core-semantics/destructuring-binding-object-rest-runtime.ts` and Node/iwasm differential coverage for shorthand and alias exclusions.
- Kept dynamic-source and parameter object rest out of scope with source-spanned issue-251 diagnostics via `fixtures/core-semantics/destructuring-binding-object-rest-unsupported.ts`.
- Required validation passed: `cargo fmt --all --check`; `cargo nextest run -E 'test(destructuring) or test(node_diff)'` (15 passed); `mise run update-issue-index -- --check`; `mise run check issues`; `cargo nextest run` (507 passed, 4 skipped).

Nested object follow-up:

- Implemented nested object binding aliases such as `let { outer: { value } } = obj` by representing object alias targets as nested binding patterns and lowering from the parent property value.
- Added `fixtures/core-semantics/destructuring-binding-nested-object-runtime.ts` and Node/iwasm differential coverage for declaration, ordinary function parameter, and arrow parameter nested object bindings.
- Fixed arrow/nested-function capture exclusion to treat names introduced by destructuring parameter patterns as local parameters instead of outer captures.
- Required validation passed: `cargo fmt --all --check`; `cargo nextest run -E 'test(destructuring) or test(node_diff)'` (16 passed); `mise run update-issue-index -- --check`; `mise run check issues`; `cargo nextest run` (510 passed, 4 skipped).

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/251-implement-destructuring-binding-runtime-semantics.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
