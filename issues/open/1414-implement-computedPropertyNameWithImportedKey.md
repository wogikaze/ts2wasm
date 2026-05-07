---
id: 1414
title: "Implement Computedpropertynamewithimportedkey"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [432]
blocks: [5229, 5167]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1414.

## Summary

Triage computedPropertyNameWithImportedKey across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case fails in directory `computedPropertyNameWithImportedKey` with diagnostics: import-export. Fresh triage on 2026-05-07 shows this bucket is not parser-owned: the AST accepts the imported computed binding parameter, and coverage stops at module graph resolution for the virtual `// @filename: /a.ts` section.

Problem: computedPropertyNameWithImportedKey has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/computedPropertyNameWithImportedKey.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertyNameWithImportedKey.ts --detail
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

- [x] Superseded by existing issue 5229 for local imports between `@filename` sections
- [x] Existing issue 5167 covers the likely next `Symbol()` builtin-call blocker
- [x] Completion evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertyNameWithImportedKey.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/computedPropertyNameWithImportedKey.ts
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

- `reference/typescript/tests/cases/compiler/computedPropertyNameWithImportedKey.ts`

## Duplicate detection

- Superseded by `issues/done/5229-w0-user-runtime-string-origin.md` for resolving `import { a } from "./a"` between `// @filename:` sections in the same reference file.
- Related next blocker: `issues/done/5167-support-global-symbol-builtin-call.md` for global `Symbol()`.
- Broad module umbrella `issues/open/432-implement-import-export.md` remains too coarse for direct implementation.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: function-resolution
Diagnostic: UnresolvedFunction / resolver-symbol
Path: reference/typescript/tests/cases/compiler/computedPropertyNameWithImportedKey.ts
Failure: unresolved function: `Symbol`
Visible symbols before failure:
- binding a, initializer Symbol()
- import ./a
- function fn, params { [a]: value }: any
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertyNameWithImportedKey.ts --detail --no-dashboard-data
result: executed=1, build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedModule:1, unsupported_features=import-export:1
date: 2026-05-07
```

Compiler evidence:

```text
tokens: ok; export const a = Symbol(); import { a } from "./a"; export function fn({ [a]: value }: any): string { return value; }
ast: ok; includes ImportNamed("./a") and function param "{[Ident { name: \"a\" }]: value}"
resolved/module_graph: UnsupportedModule issue-232 missing local module `./a` at 162..167
TypeScript oracle: TS2307 cannot find module './a' plus TS2395 merged declaration diagnostics
```

Decision:

```text
issue 5229 owns registering TypeScript reference `// @filename:` sections as virtual module paths and resolving local imports between them.
issue 5167 owns global Symbol() after the virtual import/module-graph blocker advances.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by `issues/done/5229-w0-user-runtime-string-origin.md`; related next blocker `issues/done/5167-support-global-symbol-builtin-call.md`.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- none
