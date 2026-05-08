---
id: 5460
title: "Report invalid global class member syntax"
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

Report a TypeScript-specific syntax diagnostic for `global x` inside a class
body instead of treating `global` as a method name and failing with a generic
`expected LeftParen` parser error.

Split from generated bucket
`issues/done/3480-implement-nestedGlobalNamespaceInClass.md`.

## Problem

Problem: `nestedGlobalNamespaceInClass.ts` contains invalid source intended to
prove the parser does not crash:

```ts
class C {
    global x
}
```

ts2wasm currently tokenizes the class body and fails while parsing `global` as a
method-like class member:

```text
UnsupportedSyntax: expected LeftParen, got Some(Ident("x")) at 74..75
```

TypeScript recovers and reports syntax/global-augmentation diagnostics at
`global`, followed by `';' expected` at `x`.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedGlobalNamespaceInClass.ts
```

Representative source:

```ts
class C {
    global x
}
```

Compiler evidence:

```text
tokens: ok; Class C { Ident("global") Ident("x") }
ast: fails before AST construction
resolved: fails with the same parser diagnostic
diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
message: expected LeftParen, got Some(Ident("x")) at 74..75
```

TypeScript oracle evidence:

```text
TS1068 at `global`: Unexpected token. A constructor, method, accessor, or property was expected.
TS2669 at `global`: global augmentations must be directly nested in modules.
TS2670 at `global`: global augmentations should have `declare`.
TS1005 at `x`: ';' expected.
TS2304 at `x`: Cannot find name 'x'.
TS1128 at `}`: Declaration or statement expected.
AST recovery: ClassDeclaration `class C {`, ModuleDeclaration `global`, ExpressionStatement `x`
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedGlobalNamespaceInClass.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Desired final state

The parser recognizes `global` followed by an identifier inside a class body as
invalid TypeScript syntax and emits a source-spanned diagnostic at `global` or
the following unexpected identifier. The representative should no longer report
the generic method-call parser expectation as its first blocker.

## Scope

In scope:

- [ ] Detect `global x` or equivalent invalid `global` class-member syntax in a
  class body.
- [ ] Emit a source-spanned syntax diagnostic aligned with TS1068 or an internal
  equivalent.
- [ ] Preserve ordinary valid class methods named `global`, such as
  `class C { global() {} }`.
- [ ] Add focused parser coverage for `class C { global x }`.
- [ ] Re-run the representative triage and record any next blocker.

Out of scope:

- Top-level or module-level `global { ... }` augmentation parsing, tracked by
  `issues/open/5408-parse-bare-global-augmentation-blocks.md`.
- Invalid `const` class members, tracked by
  `issues/open/5354-report-invalid-const-class-members.md`.
- General class-field parsing or class-member modifier support.
- Exact multi-diagnostic recovery parity after the first syntax diagnostic.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- focused frontend/parser tests

Do not touch:

- resolver/builtin global handling
- backend/runtime code
- namespace/module lowering logic

## Acceptance criteria

- [ ] `nestedGlobalNamespaceInClass.ts` no longer reports
  `expected LeftParen, got Some(Ident("x"))` as its first compiler blocker.
- [ ] A focused parser test covers `class C { global x }`.
- [ ] A valid method named `global` still parses.
- [ ] The diagnostic is source-spanned at `global` or `x` and classified as
  syntax, not name resolution.
- [ ] If parsing advances to a new blocker, this issue records that blocker
  before closure.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend class
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedGlobalNamespaceInClass.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedGlobalNamespaceInClass.ts --detail --no-dashboard-data
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

This issue is intentionally narrower than broad namespace/global augmentation
work. The observable parser boundary is inside a class body before any valid
namespace or ambient module syntax can be formed.

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
