---
id: 3594
title: "Implement Noninferrabletypepropagation Parser Syntax"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5345]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nonInferrableTypePropagation-parser-syntax across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage confirms this generated bucket is still a parser-syntax blocker,
but the blocker is not non-inferrable type propagation itself. The parser stops
at `declare const es: Either<string, number>[];` with the existing
issue-400 ambient const generic annotation boundary owned by issue 5345.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonInferrableTypePropagation2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nonInferrableTypePropagation2.ts --detail
```

## Desired final state

This generated bucket is superseded by
`issues/open/5345-parse-generic-ambient-const-type-annotations.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5345
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Superseding issue 5345 contains exact `reference-triage` evidence
- [x] Superseding issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonInferrableTypePropagation2.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonInferrableTypePropagation2.ts
```

Not run:

- cargo fmt --all --check: metadata-only issue close
- cargo nextest run: metadata-only issue close

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nonInferrableTypePropagation2.ts`

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/done/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/663-implement-arrayAssignmentTest-parser-syntax.md` - Implement Arrayassignmenttest Parser Syntax (same feature label, title overlap)
- `issues/open/734-implement-assignmentCompatability-parser-syntax.md` - Implement Assignmentcompatability Parser Syntax (same feature label, title overlap)
- `issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md` - Implement Asyncfunctionreturntype Parser Syntax (same feature label, title overlap)
- `issues/done/767-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)
- Superseded by `issues/open/5345-parse-generic-ambient-const-type-annotations.md`
  after fresh triage identified the exact `issue-400` ambient const generic
  annotation boundary.

## Smart triage

Generated on 2026-05-08.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonInferrableTypePropagation2.ts --detail --no-dashboard-data
result: unsupported=1; unsupported_features=parser-syntax:1; build_pass=0
```

Smart triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonInferrableTypePropagation2.ts
diagnostic: UnsupportedTypeScriptSyntax
message: issue-400: unterminated ambient variable declaration at 804..811
location: line 33, column 33
source: declare const es: Either<string, number>[];
visible symbols: filter, es
tokens: ok through interface/type aliases and ambient declarations
ast: fail before AST with issue-400
typescript oracle: ok; diagnostics=[]
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonInferrableTypePropagation2.ts --detail --no-dashboard-data
result: unsupported parser-syntax; superseded by issue 5345
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonInferrableTypePropagation2.ts
result: issue-400 ambient const generic annotation boundary at `Either<string, number>[]`
date: 2026-05-08
```

Remaining risks:

- After issue 5345, this fixture may expose the intended non-inferrable type
  propagation semantics, likely alongside issue 3595 or 3888.
