---
id: 1480
title: "Implement Constructorstaticparamnameerrors"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: [5362]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1480.

## Summary

Closed as superseded by
`issues/open/5362-report-strict-mode-static-constructor-parameter-name.md`.

Fresh triage shows the explicit `'use strict'` variant reaches the same
`constructor(static)` parser failure and TypeScript TS1213 oracle diagnostic as
issue 1479.

## Problem

Reference test results originally showed one parser-syntax failure. Current
triage shows:

```text
UnsupportedSyntax: issue-247: expected binding identifier or pattern, got Some(Static)
```

TypeScript parses the parameter and reports TS1213 because `static` is a
reserved word in strict mode.

Problem: this generated bucket is a duplicate of the focused issue 5362 and
should not be implemented directly.

## Current failure

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorStaticParamNameErrors.ts --detail --no-dashboard-data
```

Observed:

```text
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Fresh triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorStaticParamNameErrors.ts
```

Compiler evidence:

```text
tokens: ok; String("use strict") and Static token at 159..165
ast: fails before AST construction
diagnostic: issue-247 expected binding identifier or pattern, got Some(Static)
```

TypeScript oracle evidence:

```text
TS1213: Identifier expected. 'static' is a reserved word in strict mode. Class definitions are automatically in strict mode.
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5362.

## Scope

In scope:

- [x] Inspect fresh triage for `constructorStaticParamNameErrors.ts`
- [x] Confirm it matches issue 5362's strict-mode `static` parameter-name diagnostic
- [x] Preserve explicit `'use strict'` evidence in issue 5362

Out of scope:

- Direct implementation from this generated bucket
- Invalid constructor parameter modifiers such as `static a`, tracked by issue 5355
- Broad parser-syntax work

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/diagnostic.rs`
- focused parser tests

Do not touch:

- backend/runtime code for this issue-metadata closure

## Acceptance criteria

- [x] Fresh triage records the exact parser failure
- [x] Issue 5362 is updated with the explicit strict-mode variant
- [x] Existing issue 5355 remains related but not exact

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorStaticParamNameErrors.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorStaticParamNameErrors.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing: `issues/open/5362-report-strict-mode-static-constructor-parameter-name.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorStaticParamNameErrors.ts`

## Duplicate detection

- `issues/open/5362-report-strict-mode-static-constructor-parameter-name.md`
  owns both implicit class strict-mode and explicit `'use strict'` forms of
  `constructor(static)`.

## Smart triage

Generated 2026-05-07.

```text
Path: reference/typescript/tests/cases/compiler/constructorStaticParamNameErrors.ts
Compiler: UnsupportedSyntax issue-247 expected binding identifier or pattern, got Some(Static)
TypeScript: TS1213 strict-mode reserved word parameter name
```

## Completion evidence

Commits:

- filled by local commit that moves this issue to `done/`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorStaticParamNameErrors.ts --detail --no-dashboard-data
result: pass; unsupported=1, UnsupportedSyntax parser failure reproduced
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorStaticParamNameErrors.ts
result: pass; issue-247 parser failure and TS1213 oracle evidence captured
date: 2026-05-07
```

Remaining risks:

- implementation remains tracked by issue 5362
