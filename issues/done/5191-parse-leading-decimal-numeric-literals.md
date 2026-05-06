---
id: 5191
title: "Parse leading decimal numeric literals"
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

`builtinIterator.ts` reaches an object literal method body and then fails while parsing `Math.random() < .5`. The lexer emits `Dot` followed by `Number(5)`, but the expression parser does not accept that token pair as a leading-decimal numeric literal.

## Problem

The current parser has targeted support for fractional token sequences with an integer part such as `1.5`, but it does not parse JavaScript/TypeScript leading-decimal numeric literals such as `.5`.

Representative source:

```ts
const iteratorFromBare = Iterator.from({
  next() {
    return {
      done: Math.random() < .5,
      value: "a string",
    };
  },
});
```

Problem: `builtinIterator.ts` stops at a generic `unsupported expression` parser diagnostic before it can reach the intended `Iterator` type/value and iterator-helper diagnostics reported by TypeScript.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/builtinIterator.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Dot, span: Span { start: 355, end: 356 } }) at 356..357
```

Compiler evidence:

- Tokens succeed through the object literal body and include `Math`, `Dot`, `random`, call parentheses, `Less`, `Dot`, `Number(5)`.
- AST construction fails on the `Dot` token in `.5`.
- Visible-symbol extraction reaches earlier declarations such as `iterator`, `mapped`, `filtered`, `isZero`, `zero`, and `iteratorFromBare`.

TypeScript oracle evidence:

```text
TS2693: 'Iterator' only refers to a type, but is being used as a value here.
TS2339: Property 'map' does not exist on type 'Generator<number, void, unknown>'.
TS2689: Cannot extend an interface 'Iterator'. Did you mean 'implements'?
```

The oracle accepts the `.5` syntax and reports later iterator-related diagnostics.

## Desired final state

The parser accepts leading-decimal numeric literals such as `.5` in expression position. The representative reference should advance past the current `Dot` parser blocker.

## Scope

In scope:

- [x] Parse `Dot` followed by `Number` as a decimal numeric literal in primary-expression position.
- [x] Preserve member access parsing for `object.property` and numeric property access diagnostics for `object.5`.
- [x] Add focused parser coverage for `Math.random() < .5`.
- [x] Re-run the representative triage and confirm the current `unsupported expression ... Dot` blocker is gone.

Out of scope:

- Full floating-point runtime semantics beyond the existing frontend number-literal model.
- Iterator helper builtins such as `Iterator.from`, `map`, `filter`, and `flatMap`.
- TypeScript type/value diagnostics for `Iterator`.
- General decimal/exponent numeric model work not needed to parse `.5`.

## Affected paths

Expected:

- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/frontend/src/lexer_tests.rs`

Do not touch:

- `crates/backend-wasm/src/` unless triage proves parsing advances to a backend blocker.
- Iterator runtime/builtin implementation.

## Acceptance criteria

- [x] `parse_program("const done = Math.random() < .5;")` succeeds.
- [x] Parser tests distinguish `.5` from member access so `Math.random().x` still parses as property access.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/builtinIterator.ts` no longer reports `unsupported expression` on the `Dot` token at `355..356`.
- [x] If triage advances, any remaining `Iterator` type/value or iterator-helper diagnostics are recorded as separate follow-up issues instead of expanding this slice.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/builtinIterator.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] completed: `issues/done/5222-parse-ambient-generic-variable-type-annotations.md`

## Notes

Split from generated bucket `1086` on 2026-05-06. Existing iterator protocol and builtin issues are not exact matches because this bucket currently stops before iterator semantics.

## Completion evidence

Commits:

- `5c84d451`

Validation result:

```text
command: cargo nextest run -p ts2wasm-frontend
result: 192 passed, 2 skipped
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/builtinIterator.ts
result: Dot parser blocker resolved, advances to ambient generic variable declaration diagnostic tracked by issue 5222
date: 2026-05-06
```

Remaining risks:

- Triage advances to separate ambient generic variable declaration blocker (issue 5222)

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

