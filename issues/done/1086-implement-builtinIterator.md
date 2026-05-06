---
id: 1086
title: "Implement Builtiniterator"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5191]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage builtinIterator across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `builtinIterator` with diagnostics: unknown-unsupported. Fresh smart triage shows the current compiler blocker is the parser rejecting the leading-decimal numeric literal `.5` in `Math.random() < .5`.

Problem: `builtinIterator` is not yet an iterator implementation order; it must first be split at the parser blocker that prevents triage from reaching the intended iterator diagnostics.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/builtinIterator.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/builtinIterator.ts --detail
```

## Desired final state

This generated bucket is superseded by child issue `5191`, which owns the current leading-decimal numeric literal parser blocker. Do not implement directly from this bucket.

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
- [x] At least one child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/builtinIterator.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/builtinIterator.ts
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

- [x] created: `issues/open/5191-parse-leading-decimal-numeric-literals.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/builtinIterator.ts`

## Duplicate detection

- No exact existing leading-decimal numeric literal issue was found by path/title/feature scan.
- Iterator protocol and builtin issues are not exact matches because the current compiler failure occurs before AST construction reaches iterator semantics.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/builtinIterator.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `unsupported expression: Some(SpannedToken { kind: Dot, span: Span { start: 355, end: 356 } }) at 356..357`
- Source context: `done: Math.random() < .5,`
- Compiler evidence: tokens succeed; AST construction fails on the `Dot` token that begins `.5`; visible-symbol extraction reaches earlier `Iterator.from`, `map`, `filter`, and `isZero` declarations.
- TypeScript oracle: accepts `.5` syntax and reports later TS2693/TS2339/TS2689 diagnostics for `Iterator` type/value and iterator helper usage.
- Split child: `issues/open/5191-parse-leading-decimal-numeric-literals.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/builtinIterator.ts
result: pass; current blocker identified as leading-decimal numeric literal parsing, split to issue 5191
date: 2026-05-06
```

Remaining risks:

- none
