---
id: 1023
title: "Implement Awaitedtypecrash"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage awaitedTypeCrash across 1 failing reference test case and split this generated bucket into a smaller implementation-ready child issue.

## Problem

Reference test results show 1 case fails in directory `awaitedTypeCrash`. Fresh triage shows the concrete blocker is parser syntax for a generic async generator declaration.

Problem: awaitedTypeCrash has 1 reference failure that is now tracked by child issue 5148.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitedTypeCrash.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitedTypeCrash.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/done/5148-parse-generic-async-generator-declarations.md`.

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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue 5148 contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/awaitedTypeCrash.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitedTypeCrash.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5148-parse-generic-async-generator-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/awaitedTypeCrash.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: awaitedTypeCrash

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/awaitedTypeCrash.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/awaitedTypeCrash.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Less) at 115..116",
  "span_start": 115,
  "span_end": 116,
  "line": 5,
  "column": 22,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @target: esnext
3 |
4 | // https://github.com/microsoft/TypeScript/issues/51984
5 | async function* f<T extends Promise<never>>(): AsyncGenerator<T, void, void> { }
```

Compiler evidence:

```text
tokens: Async Function Star Ident("f") Less Ident("T") Extends Ident("Promise") Less Ident("never") RightShift ...
AST/resolved: parser fails before AST with expected LeftParen at the type-parameter `<`.
TypeScript oracle: ok, no diagnostics.
```

## Completion evidence

Commits:

- pending local commit

Validation result:

```text
command:
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/awaitedTypeCrash.ts
result:
pass; emitted UnsupportedSyntax parser-syntax report for generic async generator declaration; split to issue 5148
date:
2026-05-06
```

Remaining risks:

- none
