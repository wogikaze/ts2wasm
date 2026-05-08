---
id: 4291
title: "Implement Stringmatchall"
type: spike
area: reference/triage
class: triage-needed
priority: P1
depends_on: []
blocks: [5129]
created: 2026-05-01
updated: 2026-05-06
status: done
---

## Triage complete: child issue created

Child: #5129 (`String.prototype.matchAll` literal RegExp lowering)

Root cause: builtin resolution rejects `String.prototype.matchAll` before
lowering/runtime. Parser and AST support already recognize the string receiver,
RegExp literal argument, spread array, and destructuring syntax used by the
reference case.

## Summary

Triage stringMatchAll across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `stringMatchAll` with diagnostics: builtin-api. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: stringMatchAll has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/stringMatchAll.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/stringMatchAll.ts --detail
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

- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- parser/resolver code unless `reference-triage` proves the failure happens before runtime lowering

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/stringMatchAll.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/stringMatchAll.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] completed: `issues/open/5129-implement-string-match-all-literal-regexp.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/stringMatchAll.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage builtin api: stringMatchAll

- Issue class: `triage-needed`
- Feature label: `builtin-api`
- Diagnostic: `UnsupportedBuiltin`
- Message: `String.prototype.matchAll is not supported in this milestone at 38..64`
- Path: `reference/typescript/tests/cases/compiler/stringMatchAll.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage --format json tsc reference/typescript/tests/cases/compiler/stringMatchAll.ts
```

Source context:

```text
1 | // @target: es2020
2 |
3 | const matches = "matchAll".matchAll(/\w/g);
4 | const array = [...matches];
5 | const { index, input } = array[0];
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/stringMatchAll.ts --detail
```

Result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Completion evidence

Commits:

- pending issue-split commit

Validation result:

```text
python scripts/manager.py reference-triage --format json tsc reference/typescript/tests/cases/compiler/stringMatchAll.ts
=> triaged UnsupportedBuiltin at 38..64

python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/stringMatchAll.ts --detail
=> pass as triage evidence (executed=1, build_pass=0, unsupported=1)

python scripts/manager.py update-issue-index --check
=> pending

python scripts/manager.py check issues
=> pending

date: 2026-05-06
```

Remaining risks:

- none for the triaged child issue; #5129 is complete.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

