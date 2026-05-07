---
id: 1174
title: "Implement Classblockscoping"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: [5248]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1174.

## Summary

Closed after splitting the current class-expression lowering blocker to
`issues/open/5248-lower-class-expressions.md`.

## Problem

Problem: `classBlockScoping.ts` is not a direct block-scoping implementation
order. Fresh triage shows the first blocker is `ClassExpr` lowering.

## Current failure

Fresh coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classBlockScoping.ts --detail --no-dashboard-data
```

Result:

```text
executed=1
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Fresh triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classBlockScoping.ts
```

Result:

```text
UnsupportedSyntax: issue-313: class expression lowering not yet implemented
```

The AST contains `Assign { name: "Foo", expr: ClassExpr { name: "Foo", ... } }`
inside the `if` branch. The pipeline stops during lowering before block-scoping
semantics are observable.

## Desired final state

Issue 5248 owns the first executable blocker. This generated bucket should not
be implemented directly.

## Scope

In scope:

- [x] Refresh coverage and triage evidence.
- [x] Confirm the current failure is class-expression lowering.
- [x] Split the focused implementation issue.

Out of scope:

- Implementing class expression lowering in this cleanup issue.
- Block-scoping semantic parity after class expressions lower.

## Acceptance criteria

- [x] Fresh triage identifies `class expression lowering not yet implemented`.
- [x] Issue 5248 owns the focused implementation work.
- [x] 1174 is moved to `done/`.

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classBlockScoping.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classBlockScoping.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Completion evidence

Completed by split to issue 5248 on 2026-05-06.
