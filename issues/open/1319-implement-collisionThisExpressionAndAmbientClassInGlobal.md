---
id: 1319
title: "Implement Collisionthisexpressionandambientclassinglobal"
type: spike
area: frontend/resolver
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
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1319.

## Summary

Closed as superseded by `issues/done/062d-function-this-and-arguments.md` for the current oracle-matching top-level arrow `this` diagnostic.

## Problem

Reference test results previously showed 1 case failing in directory `collisionThisExpressionAndAmbientClassInGlobal` with name-resolution diagnostics. Fresh triage shows the current first blocker is the top-level arrow `this` expression, where ts2wasm reports the issue-062d unsupported `this` diagnostic and TypeScript reports TS7041 at the same span.

Problem: `collisionThisExpressionAndAmbientClassInGlobal.ts` is not a standalone implementation order; the current failure is an oracle-matching invalid global `this` capture diagnostic covered by issue 062d.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientClassInGlobal.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientClassInGlobal.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientClassInGlobal.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientClassInGlobal.ts
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

- `reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientClassInGlobal.ts`

## Duplicate detection

- `issues/done/062d-function-this-and-arguments.md` owns the current issue-linked unsupported `this` diagnostic policy for top-level or unsupported receiver forms.
- Ambient declaration and name-resolution issues are not the current first blocker in this runner view.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage ambient declaration: collisionThisExpressionAndAmbientClassInGlobal

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientClassInGlobal.ts`
```

Failure location:

```text
5 | var f = () => this;
                  ^^^^
error: [UnsupportedSyntax] issue-062d: `this` is only supported inside receiver-bound functions, class constructors, and instance methods in this milestone at 115..119
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=ambient-declaration:1
semantic_enabled=0
```

Compiler evidence:

```text
tokens: ok; includes declare class, arrow, this, new, and _this tokens
ast: ok; ambient class is erased, and runtime statements include `var f = () => this` and `var a = new _this()`
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
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientClassInGlobal.ts
result: pass; reproduced oracle-matching top-level arrow `this` diagnostic
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndAmbientClassInGlobal.ts --detail --no-dashboard-data
result: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=ambient-declaration:1
date: 2026-05-07
```

Remaining risks:

- The later `new _this()` behavior remains unproven because both ts2wasm and TypeScript stop at the preceding global `this` diagnostic.
