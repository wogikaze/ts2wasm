---
id: 5416a
title: "Report invalid block after member expression statement"
type: bug
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

`moduleKeywordRepeatError.ts` contains invalid source `module.module { }`.
The parser currently accepts `module.module` as an expression statement and
drops the following block, allowing resolver to report `UnresolvedName:
module`. TypeScript reports a syntax diagnostic at the `{`.

## Problem

The current parser path loses the malformed block boundary after a member
expression statement:

```ts
module.module { }
```

Fresh triage reports:

```text
UnresolvedName: unresolved name: `module`
```

Problem: the parser should reject or recover from a block immediately following
a completed member-expression statement, so this malformed syntax is reported at
the source boundary instead of being misclassified as a name-resolution failure.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts --detail --no-dashboard-data
```

Observed coverage:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts: UnresolvedName: name-resolution
```

Compiler evidence:

```text
tokens: Ident("module"), Dot, Ident("module"), LeftBrace, RightBrace
ast: Expr(Member(Ident module, property module))
resolved: UnresolvedName unresolved name `module`
```

TypeScript oracle:

```text
TS2591: Cannot find name 'module'.
TS1005: ';' expected. at line 4 character 15, the `{` after `module.module`
```

## Desired final state

The parser reports or recovers from a block after a completed member-expression
statement without classifying the construct as a resolver/name-resolution
blocker.

## Scope

In scope:

- [ ] Detect `ExpressionStatement` followed immediately by `{` on the same
      statement boundary when the block is not a valid labeled/function/class
      construct.
- [ ] Report a source-spanned parser diagnostic at the `{` for
      `module.module { }`, or recover while preserving an explicit syntax
      diagnostic.
- [ ] Add a focused parser or CLI fixture for `module.module { }`.
- [ ] Re-run the representative reference triage and record the next diagnostic.

Out of scope:

- Adding Node `module` global types.
- Implementing namespace/module keyword deprecation diagnostics.
- Broad ASI changes unrelated to expression statements followed by `{`.
- Lowering arbitrary expression-plus-block syntax.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- focused frontend/parser tests or CLI fixture tests

Do not touch:

- resolver/builtin logic unless a focused parser regression proves the source is
  already represented correctly
- backend/runtime code

## Acceptance criteria

- [ ] `moduleKeywordRepeatError.ts` no longer reports `UnresolvedName:
      unresolved name: module` as its first compiler blocker.
- [ ] A focused test proves `module.module { }` reports or preserves a syntax
      diagnostic at the `{`.
- [ ] Valid member-expression statements such as `module.module;` still parse as
      expression statements.
- [ ] Adjacent valid block statements after semicolons remain accepted.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend parser
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleKeywordRepeatError.ts --detail --no-dashboard-data
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

- [x] none

## Notes

Split from generated bucket `issues/open/3345-implement-moduleKeywordRepeatError.md`.
This issue intentionally targets the current observable misclassification. If
later semantic diagnostics need exact TS1005/TS2591 parity, split that after the
parser stops hiding the malformed block behind name resolution.

## Completion evidence

Fill when implemented.
