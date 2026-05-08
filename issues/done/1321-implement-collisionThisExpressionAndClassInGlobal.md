---
id: 1321
title: "Implement Collisionthisexpressionandclassinglobal"
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
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1321.

## Summary

Closed as superseded by `issues/done/062d-function-this-and-arguments.md` for the current oracle-matching top-level arrow `this` diagnostic.

## Problem

Reference test results previously showed 1 case failing in directory `collisionThisExpressionAndClassInGlobal` with class diagnostics. Fresh triage shows class parsing succeeds and the current first blocker is the top-level arrow `this` expression, where ts2wasm reports the issue-062d unsupported `this` diagnostic and TypeScript reports TS7041 at the same span.

Problem: `collisionThisExpressionAndClassInGlobal.ts` is not a standalone implementation order; the current failure is an oracle-matching invalid global `this` capture diagnostic covered by issue 062d.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndClassInGlobal.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndClassInGlobal.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndClassInGlobal.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndClassInGlobal.ts
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

- `reference/typescript/tests/cases/compiler/collisionThisExpressionAndClassInGlobal.ts`

## Duplicate detection

- `issues/done/062d-function-this-and-arguments.md` owns the current issue-linked unsupported `this` diagnostic policy for top-level or unsupported receiver forms.
- Class syntax issues are not the current first blocker in this runner view; the class declaration parses successfully.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage class: collisionThisExpressionAndClassInGlobal

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/collisionThisExpressionAndClassInGlobal.ts`
```

Failure location:

```text
5 | var f = () => this;
                  ^^^^
error: [UnsupportedSyntax] issue-062d: `this` is only supported inside receiver-bound functions, class constructors, and instance methods in this milestone at 71..75
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

Compiler evidence:

```text
tokens: ok; includes class, _this, arrow, and this tokens
ast: ok; contains `ClassDecl _this` and `var f = () => this`
resolved/lowered: fails on top-level arrow `this` with issue-062d unsupported diagnostic
```

TypeScript oracle evidence:

```text
TS7041: The containing arrow function captures the global value of 'this'.
span: line 5, character 15, length 4
AST path: SourceFile -> FirstStatement -> VariableDeclaration -> ArrowFunction -> ThisKeyword
```

Resolution:

```text
The current compiler diagnostic is an expected unsupported global `this`
diagnostic at the same source span as TypeScript's TS7041. No new
implementation child is created from this generated bucket.
```

## Completion evidence


Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndClassInGlobal.ts
result: pass; reproduced oracle-matching top-level arrow `this` diagnostic
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndClassInGlobal.ts --detail --no-dashboard-data
result: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-07
```

Remaining risks:

- Coverage currently labels the unsupported diagnostic as `unknown-unsupported`, even though the current first blocker is top-level arrow `this`; future coverage classification cleanup can relabel oracle-matching invalid TypeScript diagnostics separately.
