---
id: 5269
title: "Parse optional class property declarations"
type: feature
area: frontend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Teach the class member parser to accept TypeScript optional class property
declarations such as `p5?: number;` in class declarations.

This is the first blocker in `classUsedBeforeInitializedVariables.ts` and also
matches the previously recorded `status?: number;` parser gap in issue 3437.

## Problem

The lexer emits `Question` for optional class fields, but class member parsing
expects the next token after a property name to be a method parameter list and
rejects `?`.

Problem: `reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts`
reports `expected LeftParen, got Some(Question)` at `p5?: number;`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Question) at 111..112
```

Source context:

```ts
class Test {
    p1 = 0;
    p2 = this.p1;
    p3 = this.p4;
    p4 = 0;
    p5?: number;

    p6?: string;
}
```

Smart triage evidence:

```text
tokens: ok; Ident("p5"), Question, Colon, Ident("number"), Semicolon
AST: fails with expected LeftParen, got Some(Question)
resolved: same parser failure
TypeScript oracle: parses; later expected diagnostics include TS2729
```

## Desired final state

The parser accepts optional class property declarations, erases the TypeScript
type annotation, and continues parsing later class members without treating the
property as a method.

## Scope

In scope:

- [ ] Parse `name?: Type;` class property declarations.
- [ ] Preserve existing parsing for initialized class fields and methods.
- [ ] Add focused parser/frontend coverage for optional class fields with and
      without following initialized members.
- [ ] Re-run the representative triage and split any later class property or
      control-flow/type diagnostic blocker separately if outside this parser
      slice.

Out of scope:

- Full TypeScript definite-assignment/control-flow diagnostics.
- Optional parameter syntax outside class property declarations.
- Computed optional property names.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- runtime ABI
- package/module resolution
- unrelated class lowering

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts` no longer reports `expected LeftParen, got Some(Question)` at `p5?: number;`.
- [ ] A focused parser/frontend test accepts `class Test { p5?: number; p6?: string; }`.
- [ ] Existing initialized class fields such as `p1 = 0;` and methods continue to parse.
- [ ] Any next blocker from the same reference path is recorded in this issue or split to a follow-up if outside this scope.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classUsedBeforeInitializedVariables.ts --detail --no-dashboard-data
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

Split from generated bucket
`issues/done/1243-implement-classUsedBeforeInitializedVariables.md`.
Related generated bucket with the same parser gap:
`issues/done/3437-implement-narrowByBooleanComparison.md`.

Additional superseded bucket:

- `issues/done/3437-implement-narrowByBooleanComparison.md` reaches the same
  optional class property parser boundary for `status?: number;` in
  `class WebError extends URIError`. Fresh triage on 2026-05-08 reports
  `UnsupportedSyntax: expected LeftParen, got Some(Question) at 1079..1080`;
  TypeScript parses the file with no diagnostics, so the current compiler
  blocker is still parser support for `name?: Type;` class property
  declarations.

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
