---
id: 5463
title: "Parse nested let declarations named status"
type: bug
area: frontend/parser
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Parse a block-scoped `let status = ...` declaration inside a nested statement
block instead of treating `let` as the start of an expression statement and
failing at the following identifier.

Split from generated bucket
`issues/open/3487-implement-nestedRedeclarationInES.md`.

## Problem

Problem: `nestedRedeclarationInES6AMD.ts` is currently labeled as an AMD module
system bucket, but fresh triage shows the first blocker is a parser
misclassification inside an ordinary function body:

```text
UnsupportedSyntax: expected Comma, got Some(Ident("status")) at 69..75
```

The parser tokenizes `let status = 1;` as `Let Ident("status") ...`, but the
diagnostic indicates the nested block path is not dispatching it as a lexical
declaration.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedRedeclarationInES6AMD.ts
```

Representative source:

```ts
// @module: AMD
function a() {
    {
        let status = 1;
        status = 2;
    }
}
```

Compiler evidence:

```text
tokens: ok; Function a, nested block, Let, Ident("status"), assignment
ast: fails before AST construction
resolved: fails with the same parser diagnostic
diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
message: expected Comma, got Some(Ident("status")) at 69..75
```

TypeScript oracle evidence:

```text
TypeScript diagnostics: none
TypeScript AST path: SourceFile -> FunctionDeclaration -> Block -> Block -> VariableDeclarationList -> VariableDeclaration -> Identifier("status")
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedRedeclarationInES6AMD.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=module-system-amd:1
```

## Desired final state

The parser accepts `let status = 1;` inside a nested block and the representative
advances past the current `expected Comma` diagnostic. Any later AMD directive,
assignment, or scope-analysis blocker should be recorded separately.

## Scope

In scope:

- [ ] Dispatch nested-block `let <identifier> = <expr>;` as a lexical
  declaration, including the binding name `status`.
- [ ] Preserve ordinary expression statements named `let` only where the
  language mode and token stream actually allow them.
- [ ] Add focused parser coverage for `function a() { { let status = 1; status = 2; } }`.
- [ ] Re-run `nestedRedeclarationInES6AMD.ts` triage and record any next
  blocker.

Out of scope:

- AMD module emit or module graph behavior; current evidence does not reach a
  module-system boundary.
- Redeclaration diagnostics for same-scope names.
- General block-scoped shadowing behavior, tracked by related scope issues such
  as `issues/open/5458-allow-block-scoped-shadowing-in-nested-blocks-and-switch-cases.md`.
- Parenthesized function expression statements in nested blocks, tracked by
  `issues/open/5212-implement-remaining-bigint-mixed-runtime-coercion-edges.md`.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- focused frontend/parser tests

Do not touch:

- module graph / AMD lowering
- backend/runtime code unless parsing advances and proves a separate blocker

## Acceptance criteria

- [ ] `nestedRedeclarationInES6AMD.ts` no longer reports
  `expected Comma, got Some(Ident("status"))` at `let status`.
- [ ] A focused parser test covers
  `function a() { { let status = 1; status = 2; } }`.
- [ ] Existing valid `let` declarations in top-level and function block scopes
  still parse.
- [ ] If the representative advances to a new blocker, this issue records that
  blocker before closure.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend parser
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedRedeclarationInES6AMD.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedRedeclarationInES6AMD.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

The representative carries `// @module: AMD`, but the source has no import or
export and TypeScript accepts it as an ordinary function declaration. Current
evidence points to parser dispatch for the nested block `let` declaration.

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
