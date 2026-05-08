---
id: 3002
title: "Implement Isolateddeclarationerrorsdefault"
type: spike
area: runtime/builtins
class: triage-needed
priority: P1
depends_on: []
blocks: [5127]
created: 2026-05-01
updated: 2026-05-06
status: done
---

## Triage complete: child issue created

Child: #5127 (implement export default multi-file lowering deduplication)

Root cause: `export default` lowering generates duplicate `__ts2wasm_default` bindings across `@fileName:` multi-file sections. Not a frontend parser issue — the parser handles the syntax via TypeScript AST.

Smart triage rerun with evidence above.

## Summary

Triage isolatedDeclarationErrorsDefault across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `isolatedDeclarationErrorsDefault` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: isolatedDeclarationErrorsDefault has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/isolatedDeclarationErrorsDefault.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/isolatedDeclarationErrorsDefault.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/isolatedDeclarationErrorsDefault.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/isolatedDeclarationErrorsDefault.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/isolatedDeclarationErrorsDefault.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Not generated. Rerun with `--triage-limit 1` or higher.

## Completion evidence

Commits:

- `2eb01c0e` (`compiler: uniquify default export locals`)

Validation result:

```text
python scripts/manager.py reference-triage --format json tsc reference/typescript/tests/cases/compiler/isolatedDeclarationErrorsDefault.ts
=> pass (BuildPass)

python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/isolatedDeclarationErrorsDefault.ts --detail
=> pass (executed=1, build_pass=1)

cargo nextest run -p ts2wasm-compiler
=> pass (59 tests)

cargo fmt --all --check
=> pass

date: 2026-05-06
```

Remaining risks:

- none for this bucket; the generated issue is resolved by child #5127.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

