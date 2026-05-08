---
id: 1437
title: "Implement Conflictingtypeparametersymboltransfer"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1437.

## Summary

Triage conflictingTypeParameterSymbolTransfer across 1 failing reference test
case and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case fails in directory
`conflictingTypeParameterSymbolTransfer` with diagnostics: parser-syntax. Fresh
triage on 2026-05-07 shows the first blocker is instance class field ASI after
an initializer, split into issue 5308.

Problem: conflictingTypeParameterSymbolTransfer has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingTypeParameterSymbolTransfer.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conflictingTypeParameterSymbolTransfer.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

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
- [x] Child issue 5308 contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/conflictingTypeParameterSymbolTransfer.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingTypeParameterSymbolTransfer.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5308-parse-asi-after-instance-class-field-initializers.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/conflictingTypeParameterSymbolTransfer.ts`

## Duplicate detection

- `issues/done/5254-parse-asi-between-static-class-fields.md` is related but
  static-only; this case is instance class field ASI after an initializer.
- Split into `issues/done/5308-parse-asi-after-instance-class-field-initializers.md`.

## Smart triage

### Smart triage: Triage parser syntax: conflictingTypeParameterSymbolTransfer

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/conflictingTypeParameterSymbolTransfer.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingTypeParameterSymbolTransfer.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 597,
  "lines": 35,
  "extension": ".ts",
  "first_code_line": "class Base<U> { }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected property name, got Equal at 453..457",
  "span_start": 453,
  "span_end": 457,
  "line": 27,
  "column": 14,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
20 | class Foo<t> extends Leg {
21 |     t = {} as t
22 |
23 |     // should allow this access since t was declared as a property on Foo
24 |     foo = this.t
25 | }
```

Visible symbols before failure include parsed class declarations `Base`, `C2`,
`Leg`, and `Foo`, showing the blocker is after generic class parsing begins.

Compiler dumps:

- tokens: ok
- ast/resolved: fail with `UnsupportedSyntax: expected property name, got Equal`

TypeScript oracle:

```text
TS2304: Cannot find name 'U'.
TS2564: Property 'data' has no initializer and is not definitely assigned in the constructor.
```

Coverage result:

```text
command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conflictingTypeParameterSymbolTransfer.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_features=unknown-unsupported:1
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split into `issues/done/5308-parse-asi-after-instance-class-field-initializers.md`

Validation result:

```text
command: python scripts/manager.py update-issue-index; python scripts/manager.py update-issue-index --check; python scripts/manager.py check-issue-health; python scripts/manager.py check-issue-readiness -- --fail-ready-below 80; git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
