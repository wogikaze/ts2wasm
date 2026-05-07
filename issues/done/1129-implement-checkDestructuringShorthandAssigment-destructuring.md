---
id: 1129
title: "Implement Checkdestructuringshorthandassigment Destructuring"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5223]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage checkDestructuringShorthandAssigment-destructuring across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `checkDestructuringShorthandAssigment-destructuring` with diagnostics: destructuring. Fresh triage shows tokenization succeeds, but AST construction fails at the computed property name `[k]` after an object spread entry.

Problem: `checkDestructuringShorthandAssigment2.ts` is too broad for direct implementation. Its current observable blocker is now tracked by `issues/done/5223-report-iterator-type-only-value-use-diagnostics.md`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment2.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/done/5223-report-iterator-type-only-value-use-diagnostics.md`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the current observable blocker into issue 5223
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this issue and issue 5223

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
- [x] Issue 5223 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Issue 5223 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5223 acceptance names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment2.ts
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

- [x] created: `issues/done/5223-report-iterator-type-only-value-use-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment2.ts`

## Duplicate detection

- `issues/open/425-implement-destructuring.md` is a broad generated bucket and is not implementation-ready for this exact parser failure.
- `issues/done/247-implement-destructuring-binding-pattern-parser.md`, `issues/done/251-implement-destructuring-binding-runtime-semantics.md`, and `issues/done/252-implement-destructuring-assignment-pattern-parser.md` are related but do not cover computed property assignments after object spread in object literal initializers.

## Smart triage

### Smart triage: Triage destructuring: checkDestructuringShorthandAssigment2

- Issue class: `triage-needed`
- Feature label: `destructuring`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment2.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment2.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment2.ts --detail --no-dashboard-data
```

Source context:

```text
let o: any, k: any;
let { x } = { x: 1, ...o, [k]: 1 };
```

Current compiler failure:

```text
error: [UnsupportedSyntax] expected Dot, got Some(RightBracket) at 134..135
```

Compiler evidence:

- Tokens succeed and include `DotDotDot`, `Ident o`, `LeftBracket`, `Ident k`, `RightBracket`, `Colon`, `Number(1)`.
- AST construction fails at the computed property closing bracket.
- TypeScript AST parses the initializer as an `ObjectLiteralExpression` with a `PropertyAssignment` whose name is `ComputedPropertyName [k]`.

TypeScript oracle evidence:

```text
TS2353: Object literal may only specify known properties, and '[k]' does not exist in type '{ x: any; }'.
```

Resolution:

```text
The current blocker is now tracked by child issue 5223. It is narrower than the generated bucket: parse computed property assignments after object spread.
```

## Completion evidence

Fill only when moving to `done/`.

checkDestructuringShorthandAssigment2 triage is complete. The actionable
blocker is tracked by child issue 5223.

Commits:

- child issue: `issues/done/5223-report-iterator-type-only-value-use-diagnostics.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment2.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedSyntax destructuring
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment2.ts
result: pass; reproduced expected Dot / RightBracket parser failure and split to issue 5223
date: 2026-05-06
```

Remaining risks:

- none
