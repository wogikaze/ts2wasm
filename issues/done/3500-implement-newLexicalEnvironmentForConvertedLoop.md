---
id: 3500
title: "Implement Newlexicalenvironmentforconvertedloop"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: []
blocks: [5298]
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage newLexicalEnvironmentForConvertedLoop across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed after fresh triage showed the current first blocker is already owned by
`issues/open/5298-parse-for-of-array-binding-pattern-heads.md`.

## Problem

Reference test results show 1 cases fail in directory `newLexicalEnvironmentForConvertedLoop` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: newLexicalEnvironmentForConvertedLoop has a current parser failure at
the `for (const [value, i] of baz(set.values))` declaration head. The
implementation owner is issue 5298.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newLexicalEnvironmentForConvertedLoop.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newLexicalEnvironmentForConvertedLoop.ts --detail
```

## Desired final state

This generated bucket is superseded by
`issues/open/5298-parse-for-of-array-binding-pattern-heads.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing child issue 5298 contains exact reference-triage commands
- [x] Child issue includes failing path, diagnostic code, source context,
  visible symbols, parser evidence, and TypeScript oracle evidence
- [x] Child issue acceptance names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newLexicalEnvironmentForConvertedLoop.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newLexicalEnvironmentForConvertedLoop.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into: `issues/open/5298-parse-for-of-array-binding-pattern-heads.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/newLexicalEnvironmentForConvertedLoop.ts`

## Duplicate detection

- Exact owner found: `issues/open/5298-parse-for-of-array-binding-pattern-heads.md`.
- Completed destructuring parser/runtime issues 247, 251, and 252 explicitly
  left `for-in` / `for-of` destructuring heads out of scope.
- Issue 5461 covers unbraced nested `for..of` loop bodies, not declaration
  heads with array binding patterns.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage unknown unsupported: newLexicalEnvironmentForConvertedLoop

- Issue class: triage-needed
- Feature label: unknown-unsupported
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/newLexicalEnvironmentForConvertedLoop.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: const declarations require an initializer at 113..123
```

Focused coverage:

```text
suite=tsc
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0

reference/typescript/tests/cases/compiler/newLexicalEnvironmentForConvertedLoop.ts: UnsupportedSyntax: unknown-unsupported
```

Source context:

```ts
function foo(set: any) {
  for (const [value, i] of baz(set.values)) {
    const bar: any = [];
    (() => bar);

    set.values.push(...[]);
  }
};
```

Compiler evidence:

```text
tokens: ok; For, Const, LeftBracket, Ident("value"), Comma, Ident("i"),
RightBracket, Of, Ident("baz")
ast/resolved: fail with `const declarations require an initializer` at
the ArrayBindingPattern head
visible symbols before failure: functions baz and foo
```

TypeScript oracle evidence:

```text
TypeScript diagnostics: none
TypeScript AST path: FunctionDeclaration -> Block -> ForOfStatement ->
VariableDeclarationList -> VariableDeclaration -> ArrayBindingPattern
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newLexicalEnvironmentForConvertedLoop.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newLexicalEnvironmentForConvertedLoop.ts
result: pass; current blocker folded into issue 5298
date: 2026-05-08
```

Remaining risks:

- Issue 5298 may expose later closure-capture, spread-call, or loop-lowering
  blockers after the for-of array binding pattern head parses.
