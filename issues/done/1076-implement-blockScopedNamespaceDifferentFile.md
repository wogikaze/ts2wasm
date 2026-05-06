---
id: 1076
title: "Implement Blockscopednamespacedifferentfile"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5187]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage blockScopedNamespaceDifferentFile across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `blockScopedNamespaceDifferentFile` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: blockScopedNamespaceDifferentFile has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedNamespaceDifferentFile.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedNamespaceDifferentFile.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedNamespaceDifferentFile.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedNamespaceDifferentFile.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5187-lower-namespace-only-multi-section-files.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/blockScopedNamespaceDifferentFile.ts`

## Duplicate detection

- Generic scope-analysis buckets are not matches; they share only the broad feature label.
- Existing namespace/internal-module issue-399 buckets are broader than this empty multi-section body blocker.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/blockScopedNamespaceDifferentFile.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `multi-section file has no module bodies`
- Source context: `// @Filename: test.ts` contains `namespace C { export class Name { ... } }`; `// @Filename: typings.d.ts` contains `declare namespace A { namespace AA { ... } }`.
- Compiler evidence: tokens include namespace/class/declaration content, but AST and resolved dumps are empty because the multi-section lowering path drops sections with no static module body.
- TypeScript oracle: `TS2729: Property 'AA' is used before its initialization.` at the `A.AA` static member initializers.
- Superseding child: `issues/open/5187-lower-namespace-only-multi-section-files.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedNamespaceDifferentFile.ts
result: pass; current blocker is an empty multi-section body path before namespace/scope diagnostics, split to issue 5187
date: 2026-05-06
```

Remaining risks:

- none
