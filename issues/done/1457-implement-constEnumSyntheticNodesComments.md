---
id: 1457
title: "Implement Constenumsyntheticnodescomments"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1457.

## Summary

Closed as split. Fresh triage shows the current first blocker in
`constEnumSyntheticNodesComments.ts` is not enum emit/comment semantics; the
lexer rejects the ES extended Unicode code point escape inside the string
literal `"\u{44}"`.

The focused implementation slice is
`issues/open/5353-parse-extended-unicode-string-escapes.md`.

## Problem

Reference test results previously grouped this file under an enum bucket.
Current compiler behavior tokenization fails before the enum/switch semantics
can be triaged because string literal lexing rejects `\u{44}`.

Problem: `constEnumSyntheticNodesComments.ts` is blocked by string-literal
extended Unicode escape parsing before enum synthetic-node/comment behavior can
be evaluated.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implementation proceeds through the focused
lexer issue `issues/open/5353-parse-extended-unicode-string-escapes.md`.

## Scope

In scope:

- [x] Inspect fresh smart triage for the affected file.
- [x] Confirm the current first blocker is not covered by an implementation-ready issue.
- [x] Split the current first blocker to a focused child issue.
- [x] Preserve exact reproduction commands and representative diagnostic/AST
      evidence in this closed issue and the child issue.

Out of scope:

- Direct implementation from this generated bucket.
- Full const-enum runtime/inlining.
- Enum synthetic-node comment emit fidelity.
- Semantic parity after the lexer blocker advances.

## Affected paths

Expected:

- `crates/frontend/src/lexer_strings.rs`
- focused lexer/parser tests
- fixtures if needed

Do not touch:

- backend/runtime emit
- enum lowering
- unrelated parser syntax

## Acceptance criteria

- [x] A focused child issue exists for the exact current blocker.
- [x] This closed issue includes failing path, diagnostic code, source context,
      visible symbols, and TypeScript AST evidence.
- [x] Completion evidence names the exact reference path and current
      diagnostic/stdout change.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5353-parse-extended-unicode-string-escapes.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts`

## Duplicate detection

- `issues/open/5018-implement-legacy-global-builtin.md` contains the same
  lexer diagnostic for test262 `escape('\u{10401}')`, but it is a broad
  generated bucket, not a focused implementation-ready child.
- `issues/open/4642-implement-unicodeStringLiteral.md` is another generated
  string-literal bucket without smart triage evidence.
- Existing enum issues such as `issues/open/428-implement-enum.md` and
  `issues/open/1406-implement-computedEnumMemberSyntacticallyString-parser-syntax.md`
  do not own this string-literal lexer blocker.

## Smart triage

### Smart triage: Triage enum: constEnumSyntheticNodesComments

- Issue class: `triage-needed`
- Feature label: `enum`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts
```

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=enum:1
```

Current diagnostic:

```text
UnsupportedSyntax: invalid unicode escape sequence at 329..332
tokens/ast/resolved: invalid unicode escape sequence at 329..332
```

Source context:

```ts
case En["\u{44}"]:
    return assert<3>(a);
```

Compiler evidence:

- Tokenization fails before AST construction.
- Visible symbols before failure include a bogus binding named `enum` for the
  earlier `const enum En { A, B, C, D }`, plus function `verify`.

TypeScript oracle evidence:

- TypeScript accepts the file with no diagnostics.
- TypeScript parses the current failure site as
  `CaseClause -> ElementAccessExpression -> StringLiteral "\"\\u{44}\""`.
- TypeScript infers `verify(a: En)` returns `0 | 1 | 2 | 3`.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts
result: pass; current first blocker split to issue 5353
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=enum:1
date: 2026-05-07
```

Remaining risks:

- After issue 5353 advances string-literal lexing, this file may expose
  const-enum parsing/binding, enum element access, generic type argument
  erasure, switch narrowing, or comment emit fidelity as later blockers.
