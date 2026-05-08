---
id: 5473
title: "Report destructuring declaration missing initializer"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TypeScript-compatible diagnostic for destructuring variable
declarations that omit the required initializer, such as `var [a], { b };`.

This is the current blocker from
`noImplicitAnyDestructuringVarDeclaration.ts`.

## Problem

The parser recognizes destructuring binding patterns, but rejects a binding
pattern without an initializer using the generic issue-247 unsupported syntax
boundary:

```text
UnsupportedSyntax: issue-247: binding patterns require an initializer at 49..52
```

TypeScript parses the same declarations and reports TS1182 at each
initializer-less destructuring declaration.

Problem: initializer-less destructuring declarations report a generic
UnsupportedSyntax boundary instead of a TS1182-style source-spanned diagnostic.

## Current failure

Fresh triage/coverage for
`reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration.ts`
shows:

```text
coverage: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=destructuring:1
triage: UnsupportedSyntax issue-247 binding patterns require an initializer at 49..52
```

Representative source:

```ts
var [a], {b}, c, d; // error
var [a1 = undefined], {b1 = null}, c1 = undefined, d1 = null; // error
var [a2]: [any], {b2}: { b2: any }, c2: any, d2: any; // error
```

Compiler evidence:

```text
tokens: ok through var [a], {b}, c, d and later destructuring declaration forms
ast/resolved: fail before AST construction with issue-247 at [a]
```

TypeScript oracle:

```text
TS1182: A destructuring declaration must have an initializer.
```

The oracle reports TS1182 at each initializer-less array or object binding
pattern and still provides hints for non-destructured bindings such as `c`,
`d`, `c1`, and `d1`.

## Desired final state

Initializer-less destructuring declarations produce a source-spanned
TS1182-style diagnostic instead of a generic issue-247 unsupported syntax
boundary, and valid initialized destructuring declarations continue to parse and
build.

## Scope

In scope:

- [ ] Replace the issue-247 initializer-less array/object binding declaration path with a TS1182-style diagnostic while preserving initialized destructuring declarations.

Out of scope:

- Full noImplicitAny type inference for destructuring.
- Runtime semantics for all destructuring patterns beyond already-supported initialized cases.
- Destructuring assignment expressions.
- `for-in` / `for-of` destructuring heads.

## Affected paths

Expected:

- `crates/frontend/src/parser/binding_patterns.rs`
- `crates/frontend/src/diagnostic.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- backend-wasm
- module import/export handling
- unrelated destructuring assignment paths

## Acceptance criteria

- [ ] A focused parser/diagnostic test covers both `var [a];` and `var { b };`.
- [ ] `noImplicitAnyDestructuringVarDeclaration.ts` no longer reports generic `issue-247: binding patterns require an initializer`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend destructuring
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringVarDeclaration2.ts
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from
`issues/done/3535-implement-noImplicitAnyDestructuringVarDeclaration.md`.

Related but not duplicates:

- `issues/done/247-implement-destructuring-binding-pattern-parser.md` added
  general binding-pattern parser support but intentionally left unsupported
  forms on issue-linked diagnostics.
- `issues/done/251-implement-destructuring-binding-runtime-semantics.md` covers
  runtime semantics for supported initialized destructuring, not missing
  initializer diagnostics.

## Completion evidence

Fill when implemented.
