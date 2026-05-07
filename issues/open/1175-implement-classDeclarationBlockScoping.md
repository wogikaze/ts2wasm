---
id: 1175
title: "Implement Classdeclarationblockscoping"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5249, 5250]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1175.

## Summary

Closed after splitting the two current representative blockers:

- `issues/open/5249-scope-block-local-class-declarations.md`
- `issues/open/5250-parse-class-declarations-in-nested-block-statements.md`

## Problem

Problem: `classDeclarationBlockScoping` is a generated bucket with two distinct
current blockers: block-local class scoping and nested-block class parsing.

## Current failure

Fresh coverage and triage were run for both representative paths.

### classDeclarationBlockScoping1

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classDeclarationBlockScoping1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclarationBlockScoping1.ts
```

Result:

```text
DuplicateLocal: duplicate local variable: `C` at 43..59
```

### classDeclarationBlockScoping2

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classDeclarationBlockScoping2.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclarationBlockScoping2.ts
```

Result:

```text
UnsupportedSyntax: expected Comma, got Some(Ident("C")) at 90..91
```

## Desired final state

Issues 5249 and 5250 own the executable work. This generated bucket should not
be implemented directly.

## Scope

In scope:

- [x] Refresh both representative paths.
- [x] Split one issue per observable first blocker.
- [x] Close the generated bucket.

Out of scope:

- Implementing the split child issues in this cleanup.
- Full TypeScript class block-scoping semantic parity.

## Acceptance criteria

- [x] `classDeclarationBlockScoping1.ts` evidence is captured in issue 5249.
- [x] `classDeclarationBlockScoping2.ts` evidence is captured in issue 5250.
- [x] 1175 is moved to `done/`.

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classDeclarationBlockScoping1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclarationBlockScoping1.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classDeclarationBlockScoping2.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclarationBlockScoping2.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Completion evidence

Completed by split to issues 5249 and 5250 on 2026-05-06.
