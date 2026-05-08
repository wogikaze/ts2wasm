---
id: 5462
title: "Parse for-of assignment heads with nested object rest"
type: feature
area: frontend/parser
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Parse `for..of` loop heads whose left side is an assignment-target destructuring
pattern, including nested array/object rest such as
`for ([{ ...y }] of [[{ abc: 1 }]]) ;`.

Split from generated bucket
`issues/done/3485-implement-nestedObjectRest.md`.

## Problem

Problem: `nestedObjectRest.ts` parses the preceding destructuring assignment
enough to reach the loop, but the parser treats the `for` head
`[{ ...y }]` as an ordinary expression and expects a semicolon before `of`:

```text
UnsupportedSyntax: expected Semicolon, got Some(Of) at 136..138
```

TypeScript accepts the source with no diagnostics and represents the loop as a
`ForOfStatement`.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedObjectRest.ts
```

Representative source:

```ts
var x, y;

[{ ...x }] = [{ abc: 1 }];
for ([{ ...y }] of [[{ abc: 1 }]]) ;
```

Compiler evidence:

```text
tokens: ok; array/object rest assignment and for head tokenized
ast: fails before AST construction
resolved: fails with the same parser diagnostic
diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
message: expected Semicolon, got Some(Of) at 136..138
visible symbols before failure: x
```

TypeScript oracle evidence:

```text
TypeScript diagnostics: none
topLevel: var x,y; ExpressionStatement `[{ ...x }] = [{ abc: 1 }];`; ForOfStatement `for ([{ ...y }] of [[{ abc: 1 }]]) ;`
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedObjectRest.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=object-literal:1
```

## Desired final state

The parser recognizes assignment-target destructuring patterns in `for..of`
heads and does not misclassify `of` as an unexpected token after an expression.
The representative should advance past the parser boundary to either supported
lowering or a source-spanned destructuring/object-rest runtime subset
diagnostic.

## Scope

In scope:

- [ ] Parse `for ([target] of expr) statement` as a `for..of` loop head without
  a declaration list.
- [ ] Preserve nested object-rest assignment target syntax inside the array
  pattern, e.g. `[{ ...y }]`.
- [ ] Add focused parser coverage for `for ([{ ...y }] of [[{ abc: 1 }]]) ;`.
- [ ] Re-run `nestedObjectRest.ts` triage and record the next blocker if it
  advances beyond parsing.

Out of scope:

- Declaration heads such as `for (const [key, value] of expr)`, tracked by
  `issues/open/5298-parse-for-of-array-binding-pattern-heads.md`.
- Runtime lowering for dynamic-source object rest, tracked for declaration
  shapes by `issues/open/5452-lower-nested-object-rest-binding-from-narrowed-source.md`
  and broader issue-251 follow-ups.
- `for-in` destructuring heads.
- Full TypeScript object-rest semantic parity.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- focused frontend/parser tests

Do not touch:

- backend/runtime code unless parsing advances and a focused runtime blocker is
  split or explicitly implemented
- broad object-literal buckets

## Acceptance criteria

- [ ] `nestedObjectRest.ts` no longer reports
  `expected Semicolon, got Some(Of)` at the loop `of`.
- [ ] A focused parser test covers `for ([{ ...y }] of [[{ abc: 1 }]]) ;`.
- [ ] Existing destructuring assignment expression parsing for
  `[{ ...x }] = [{ abc: 1 }];` still parses.
- [ ] If the representative advances to an object-rest lowering blocker, this
  issue records that blocker before closure.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend parser
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedObjectRest.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedObjectRest.ts --detail --no-dashboard-data
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

This issue is parser-only. The source uses object rest, but current evidence
shows the first blocker is recognizing the `for..of` head form.

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
