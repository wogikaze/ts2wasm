---
id: 1040
title: "Implement Basetypeprivatememberclash"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage baseTypePrivateMemberClash across 1 failing reference test case and split this generated bucket into an implementation-ready child issue.

## Problem

Fresh smart triage shows the parser now accepts the source and erases the type-only interface/private typed fields before backend emission. TypeScript reports TS2320 for an interface extending two classes with non-identical private members.

Problem: `baseTypePrivateMemberClash` is not a standalone implementation order; the executable frontend/resolver diagnostic slice is split to issue 5158.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypePrivateMemberClash.ts
```

Current compiler diagnostic:

```text
BackendIo: wat2wasm failed
```

## Desired final state

This generated bucket is closed as superseded by `issues/open/5158-report-interface-private-member-clash.md`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one observable frontend/resolver diagnostic behavior into a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/frontend/src/diagnostic.rs`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue 5158 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic change

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
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypePrivateMemberClash.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no owned Rust implementation changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5158-report-interface-private-member-clash.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/baseTypePrivateMemberClash.ts`

## Duplicate detection

- No existing open implementation-ready issue matched the exact `interface Z extends X, Y` private-member clash diagnostic.

## Smart triage

### Smart triage: Triage backend io: baseTypePrivateMemberClash

- Issue class: `triage-needed`
- Feature label: `backend-io`
- Diagnostic: `BackendIo` / `backend-io`
- Path: `reference/typescript/tests/cases/compiler/baseTypePrivateMemberClash.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypePrivateMemberClash.ts
```

Source context:

```text
2 | class X {
3 |     private m: number;
4 | }
5 | class Y {
6 |     private m: string;
7 | }
9 | interface Z extends X, Y { }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "X",
    "line": 2,
    "column": 1
  },
  {
    "kind": "class",
    "name": "Y",
    "line": 5,
    "column": 1
  }
]
```

Parser/resolver evidence:

```text
tokens: ok
ast: ok; contains empty ClassDecl entries for X and Y after TypeScript field/interface erasure
resolved: ok; X and Y have no private_fields because typed private declarations are erased
wat: generated, then final build fails in wat2wasm
```

TypeScript oracle evidence:

```text
TS2320: Interface 'Z' cannot simultaneously extend types 'X' and 'Y'.
  Named property 'm' of types 'X' and 'Y' are not identical.
```

Resolution:

```text
Issue 5158 now owns the concrete frontend/resolver diagnostic contract. Definite-assignment diagnostics are intentionally out of scope.
```

## Completion evidence

Commits:

- superseded by `issues/open/5158-report-interface-private-member-clash.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypePrivateMemberClash.ts
result: pass; reproduced current BackendIo state and TypeScript TS2320 oracle evidence
date: 2026-05-06
```

Remaining risks:

- Issue 5158 still needs implementation.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

