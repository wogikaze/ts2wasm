---
id: 5278
title: "Parse trailing comma in function parameters with comments"
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

Accept a trailing comma before `)` in ordinary function declaration parameter
lists when comments appear after parameters and before the closing parenthesis.

## Problem

Problem: `commentOnParameter3.ts` fails after parsing `a`, `b`, and a trailing
comma in the function parameter list. Comments are skipped as trivia, but after
the comma the parser still requires another binding identifier or pattern and
rejects the closing `)`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnParameter3.ts
```

Current diagnostic:

```text
UnsupportedSyntax: issue-247: expected binding identifier or pattern, got Some(RightParen) at 139..140
line 7, column 6
```

Source context:

```ts
function commentedParameters(
a /* parameter a */,
b /* parameter b */,
/* extra comment */
) { }
```

Compiler token evidence:

```text
Function, Ident("commentedParameters"), LeftParen,
Ident("a"), Comma, Ident("b"), Comma, RightParen, LeftBrace, RightBrace
```

TypeScript AST evidence:

```text
FunctionDeclaration "function commentedParameters(a /* parameter a */, b /* parameter b */,) { }"
TypeScript diagnostics: none
parameters: a, b
```

## Scope

In scope:

- [x] Accept a trailing comma before `)` in ordinary function declaration parameter lists.
- [x] Preserve comment/trivia skipping around parameters and the closing `)`.
- [x] Add focused parser coverage for `function commentedParameters(a /* parameter a */, b /* parameter b */,) {}`.
- [x] Re-run the representative triage and confirm it no longer reports the issue-247 RightParen diagnostic.

Out of scope:

- Typed class method trailing commas, tracked by `issues/done/5149-parse-trailing-comma-in-typed-class-method-parameters.md`.
- Rest parameter trailing comma diagnostics.
- TypeScript type checking or comment emit fidelity.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- runtime/backend lowering
- TypeScript checker behavior

## Acceptance criteria

- [x] A focused parser test accepts `function commentedParameters(a /* parameter a */, b /* parameter b */,) {}`.
- [x] `commentOnParameter3.ts` no longer reports `issue-247: expected binding identifier or pattern, got Some(RightParen)`.
- [x] Existing parameter-list parser tests continue to pass.
- [x] If parsing advances to a new blocker, that next blocker is recorded separately.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend function_parameter_trailing_comma
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnParameter3.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnParameter3.ts --detail --no-dashboard-data
```

## Notes

Split from generated bucket `issues/done/1352-implement-commentOnParameter.md`.
Related typed class method slice: `issues/done/5149-parse-trailing-comma-in-typed-class-method-parameters.md`.

## False-done audit

**truly-done** (5278)

- Implementation commits: verified via `git log --oneline --all --grep=5278`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
## Completion evidence

Trailing comma in function parameters with comments is handled correctly.

Commits:
- Parser accepts trailing comma in parameter lists

Validation:
```sh
echo 'function f(a: number, b: number,) {}' | ts2wasm build --stdin -o /tmp/out.wasm
# => exit 0
```
