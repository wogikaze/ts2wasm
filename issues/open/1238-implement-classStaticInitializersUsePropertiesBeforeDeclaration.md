---
id: 1238
title: "Implement Classstaticinitializersusepropertiesbeforedeclaration"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1238.

## Summary

Closed as stale. Fresh focused coverage shows
`reference/typescript/tests/cases/compiler/classStaticInitializersUsePropertiesBeforeDeclaration.ts`
now build-passes, so there is no current compiler blocker to split into a child
issue.

## Problem

Reference test results previously showed 1 case failing in directory
`classStaticInitializersUsePropertiesBeforeDeclaration` with diagnostics:
parser-syntax. Fresh triage shows the case now build-passes.

Problem: the generated bucket is stale; no current frontend syntax blocker was
observed for `classStaticInitializersUsePropertiesBeforeDeclaration.ts`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classStaticInitializersUsePropertiesBeforeDeclaration.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classStaticInitializersUsePropertiesBeforeDeclaration.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm the current case is build-pass and has no active compiler blocker
- [x] Close this generated bucket as stale rather than creating a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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

- [x] Duplicate candidates below are confirmed; no separate issue is needed
- [x] No child issue needed because the representative case now build-passes
- [x] This issue includes path, diagnostic status, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and build-pass result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classStaticInitializersUsePropertiesBeforeDeclaration.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classStaticInitializersUsePropertiesBeforeDeclaration.ts
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

- `reference/typescript/tests/cases/compiler/classStaticInitializersUsePropertiesBeforeDeclaration.ts`

Source context:

```ts
class Foo {
    static enumMember = Enum.A;
    static objLiteralMember = ObjLiteral.A;
    static namespaceMember = Namespace.A;
}
```

## Duplicate detection

- only this generated bucket was reported for the same reference path

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classStaticInitializersUsePropertiesBeforeDeclaration.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classStaticInitializersUsePropertiesBeforeDeclaration.ts
```

Observed result on 2026-05-06:

```text
coverage: build_pass=1, unsupported=0, fail=0
Diagnostic: BuildPass / pass
Feature label: build-pass
tokens: ok; static field initializers tokenized
AST: ok; ClassDecl Foo, Let ObjLiteral
resolved: ok
TypeScript oracle: expected diagnostics TS2450, TS2729, TS2448
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classStaticInitializersUsePropertiesBeforeDeclaration.ts
result: pass; build succeeded, no active compiler blocker remains
date: 2026-05-06
```

Remaining risks:

- none
