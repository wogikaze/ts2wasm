---
id: 5280
title: "Validate commented top-level function overloads"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Handle top-level TypeScript function overload signatures that include leading
comments/trivia before a single implementation declaration.

## Problem

Problem: `commentOnSignature1.ts` parses successfully, but `validate_ast`
stops at a `DuplicateFunction` diagnostic for the second bodyless `foo`
overload signature even though TypeScript accepts the overload group.

This is related to issue 5200, but the comment/trivia reference also contains
additional constructor and class method overloads later in the file. Keeping it
separate preserves a smaller work order and avoids broadening 5200 beyond the
readiness gate.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnSignature1.ts
```

Current diagnostic:

```text
DuplicateFunction: duplicate function definition: `foo` at 231..239
line 12, column 10
```

Source context:

```ts
/*! Don't keep this pinned comment */
function foo(n: number): void;
// Don't keep this comment.
function foo(s: string): void;
function foo(a: any): void {
}
```

Compiler evidence:

```text
tokens: ok
ast: ok
Function foo(n), Function foo(s), Function foo(a) with body
validate_ast: DuplicateFunction for second bodyless overload signature
```

TypeScript oracle:

```text
ok: true
diagnostics: []
```

## Scope

In scope:

- [ ] Distinguish commented bodyless top-level function overload signatures from duplicate concrete implementations.
- [ ] Accept two commented/bodyless `foo` overload signatures followed by one `foo` implementation.
- [ ] Preserve comments as trivia; no comment emit fidelity is required in this slice.
- [ ] Re-run `commentOnSignature1.ts` and record the next blocker if constructor or class method overloads appear.

Out of scope:

- General top-level overload validation without comments, tracked by `issues/done/5200-validate-top-level-function-overload-implementations.md`.
- Class constructor overload signatures.
- Class method overload signatures.
- Comment emit fidelity.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- backend runtime lowering
- unrelated duplicate-function diagnostics

## Acceptance criteria

- [ ] `commentOnSignature1.ts` no longer reports `DuplicateFunction` for the second bodyless top-level `foo` overload signature.
- [ ] A focused fixture covers a commented overload group with two signatures and one implementation.
- [ ] Existing multiple concrete function implementation diagnostics still report duplicate-function.
- [ ] Any later constructor or class method overload blocker from this reference is recorded separately.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnSignature1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnSignature1.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/done/1354-implement-commentOnSignature.md`.
Related top-level overload issue: `issues/done/5200-validate-top-level-function-overload-implementations.md`.
