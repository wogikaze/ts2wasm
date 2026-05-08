---
id: 5457
title: "Support String.prototype.match with string pattern argument"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Support the TypeScript/JavaScript-compatible
`String.prototype.match(string)` shape by treating a string argument as a
plain RegExp pattern for the current supported subset.

Split from generated bucket
`issues/done/3471-implement-narrowingWithNonNullExpression.md`.

## Problem

Problem: `narrowingWithNonNullExpression.ts` parses the non-null and optional
index expressions, but lowering stops at `const m = ''.match('');` because
`String.prototype.match` currently accepts only a RegExp literal or
`new RegExp("plain")` argument.

The current compiler reports:

```text
UnsupportedRegExp: issue-051: String.prototype.match supports only RegExp literal or new RegExp("plain") arguments in this subset at 29..41
```

TypeScript accepts this source and infers `m` as `RegExpMatchArray | null`.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingWithNonNullExpression.ts
```

Representative source:

```ts
// @target: es2015
const m = ''.match('');
m! && m[0];
m?.[0]! && m[0];
```

Compiler evidence:

```text
tokens: ok; String(""), Dot, Ident("match"), LeftParen, String(""), RightParen
ast: ok; Let m = Call(Member(String("").match), [String("")])
ast: ok; `m!` is represented as Ident("m") before `&&`
ast: ok; `m?.[0]!` is represented as OptionalIndex before `&&`
resolved: fails during lower_program
diagnostic: UnsupportedRegExp / unsupported-feature-boundary
message: issue-051: String.prototype.match supports only RegExp literal or new RegExp("plain") arguments in this subset
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingWithNonNullExpression.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Desired final state

`''.match('')` lowers through the existing RegExp plain-pattern runtime path
instead of reporting issue-051. The representative reference should advance
past the current match-argument diagnostic to the next non-null, optional
index, match-array, narrowing, or runtime blocker.

## Scope

In scope:

- [ ] Accept a string literal argument for `String.prototype.match` by
  reusing the existing `new RegExp("plain")` validation and lowering path.
- [ ] Preserve the existing rejection for unsupported non-plain patterns.
- [ ] Add focused lowering or CLI coverage for `"abc".match("b")` and an
  empty-string pattern case if supported by the existing runtime matcher.
- [ ] Re-run the representative triage and record any later blocker.

Out of scope:

- Full RegExp syntax or match-array shape completeness, tracked by the RegExp
  issues around issue 051 and `issues/open/5020-implement-regexp-literal.md`.
- `String.prototype.match()` arity relaxation, tracked by
  `issues/open/5136-fix-arity-validation-regexp-string-prototype.md`.
- General string-to-RegExp coercion for dynamic values or non-literal
  arguments.
- Non-null assertion semantics beyond confirming that this representative
  advances past the current built-in boundary.

## Affected paths

Expected:

- `crates/ir/src/lowered/program_builtins.rs`
- `crates/cli/tests/`
- `fixtures/core-semantics/`

Do not touch:

- lexer/parser non-null assertion handling unless fresh triage after this
  built-in fix proves a parser-owned blocker remains
- broad RegExp runtime syntax support beyond plain literal-backed matching

## Acceptance criteria

- [ ] `"abc".match("b")` no longer reports issue-051 and produces the same
  observable result as the current plain RegExp literal path.
- [ ] The empty-string pattern case used by
  `narrowingWithNonNullExpression.ts` either builds through this boundary or
  reports a narrower documented runtime/plain-pattern diagnostic.
- [ ] Existing `String.prototype.match(/plain/)` and
  `String.prototype.match(new RegExp("plain"))` fixtures/tests still pass.
- [ ] Unsupported string patterns that the current plain matcher cannot handle
  continue to report an issue-linked diagnostic instead of compiling with
  incorrect semantics.
- [ ] `narrowingWithNonNullExpression.ts` no longer reports the current
  `String.prototype.match supports only RegExp literal or new RegExp("plain")`
  diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli regexp
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingWithNonNullExpression.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingWithNonNullExpression.ts --detail --no-dashboard-data
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

Related but distinct:

- `issues/open/5020-implement-regexp-literal.md` is a broad generated RegExp
  triage bucket and should not be selected directly for this narrow
  string-argument slice.
- `issues/done/051-implement-regexp.md` implemented constrained
  `String.prototype.match` support for direct RegExp literal and
  `new RegExp("plain")` arguments, but fresh evidence shows literal string
  arguments are still rejected.

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
