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

- [ ] Resolve names introduced by array/object binding patterns.
- [ ] Lower declaration patterns for the supported dense-array and object-property subset.
- [ ] Lower ordinary function and arrow parameter patterns for the supported call subset.
- [ ] Implement defaults, nested patterns, elisions, and rest binding where runtime support is available.
- [ ] Emit issue-linked diagnostics for unsupported iterator/property semantics that remain outside the runtime subset.

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

- [ ] `let [a, b] = arr; console.log(a);` binds `a` and `b` from supported arrays.
- [ ] `let { x } = obj; console.log(x);` binds `x` from supported objects.
- [ ] `function f([a], { x }) { return a + x; }` and `let f = ([a]) => a;` bind parameter patterns.
- [ ] Unsupported destructuring forms report issue-linked diagnostics instead of compiling to incorrect bindings.
- [ ] Node/iwasm differential fixtures cover supported declaration and parameter patterns.
- [ ] `docs/language-reference/javascript-features.md` is updated from parser-only to runtime-supported status for covered forms.

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

- [ ] updated: `docs/language-reference/javascript-features.md`

Current state:

- [ ] updated: `current-state.md` if runtime support changes

Follow-up issues:

- [x] split: `issues/open/252-implement-destructuring-assignment-pattern-parser.md`

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
