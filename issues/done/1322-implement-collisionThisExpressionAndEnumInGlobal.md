---
id: 1322
title: "Implement Collisionthisexpressionandenuminglobal"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Closed as superseded by `issues/done/062d-function-this-and-arguments.md` for the current oracle-matching top-level arrow `this` diagnostic.

## Problem

Reference test results previously showed 1 case failing in directory `collisionThisExpressionAndEnumInGlobal` with parser-syntax diagnostics. Fresh triage shows the current first blocker is the top-level arrow `this` expression, where ts2wasm reports the issue-062d unsupported `this` diagnostic and TypeScript reports TS7041 at the same span.

Problem: `collisionThisExpressionAndEnumInGlobal.ts` is not a standalone implementation order; the current failure is an oracle-matching invalid global `this` capture diagnostic covered by issue 062d. The broader enum declaration support remains tracked separately by `issues/open/428-implement-enum.md`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndEnumInGlobal.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndEnumInGlobal.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/done/062d-function-this-and-arguments.md` for the current unsupported top-level `this` diagnostic. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 062d's unsupported top-level `this` diagnostic behavior
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] This closed issue contains an exact `reference-triage` command
- [x] This closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndEnumInGlobal.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndEnumInGlobal.ts
```

Not run:

- `cargo fmt --all --check` (not run; issue metadata only)
- `cargo nextest run` (not run; issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionThisExpressionAndEnumInGlobal.ts`

## Duplicate detection

- `issues/done/062d-function-this-and-arguments.md` owns the current issue-linked unsupported `this` diagnostic policy for top-level or unsupported receiver forms.
- `issues/open/428-implement-enum.md` remains the broad enum support tracker, but this file's current first compiler diagnostic is the top-level arrow `this` span rather than an actionable enum parser/lowering blocker.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage enum: collisionThisExpressionAndEnumInGlobal

- Issue class: `triage-needed`
- Feature label: `enum`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/collisionThisExpressionAndEnumInGlobal.ts`
```

Failure location:

```text
7 | var f = () => this;
                  ^^^^
error: [UnsupportedSyntax] issue-062d: `this` is only supported inside receiver-bound functions, class constructors, and instance methods in this milestone at 111..115
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=enum:1
semantic_enabled=0
```

Compiler evidence:

```text
tokens: ok; includes enum, _this, enum members, arrow, and this tokens
ast: ok; contains `var f = () => this`; the enum declaration is not represented in the current AST dump
resolved/lowered: fails on top-level arrow `this` with issue-062d unsupported diagnostic
```

TypeScript oracle evidence:

```text
TS7041: The containing arrow function captures the global value of 'this'.
span: line 7, character 15, length 4
AST path: SourceFile -> FirstStatement -> VariableDeclaration -> ArrowFunction -> ThisKeyword
Top-level AST also includes `EnumDeclaration` for `enum _this { _thisVal1, _thisVal2 }`.
```

Resolution:

```text
The current compiler diagnostic is an expected unsupported global `this`
diagnostic at the same source span as TypeScript's TS7041. No new
implementation child is created from this generated bucket.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndEnumInGlobal.ts
result: pass; reproduced oracle-matching top-level arrow `this` diagnostic
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndEnumInGlobal.ts --detail --no-dashboard-data
result: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=enum:1
date: 2026-05-07
```

Remaining risks:

- Coverage labels the file as `enum` because the source contains an enum declaration, and the AST dump does not retain that declaration today. This closure does not implement or close enum support; broader enum work remains tracked by `issues/open/428-implement-enum.md`.
