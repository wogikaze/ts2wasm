---
id: 1261
title: "Implement Clodulesderivedclasses"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1261.

## Summary

Triage clodulesDerivedClasses across 1 reference test case and split the
remaining semantic diagnostic gap into an implementation-ready child issue.

## Problem

Reference test results originally showed 1 case failing in directory
`clodulesDerivedClasses` with diagnostics: import-export. Fresh focused coverage
on 2026-05-07 shows the case now build-passes.

Problem: the stale build blocker is gone, but TypeScript oracle reports TS2417
for a namespace-augmented static-side inheritance mismatch. That narrower
semantic follow-up is split to issue 5331.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/clodulesDerivedClasses.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/clodulesDerivedClasses.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5331 for
the remaining namespace-augmented static-side inheritance diagnostic parity.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Full TypeScript structural type checking

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
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/clodulesDerivedClasses.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/clodulesDerivedClasses.ts
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

- [x] created: `issues/open/5331-report-class-namespace-static-side-inheritance-diagnostic.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/clodulesDerivedClasses.ts`

## Duplicate detection

- `issues/open/5314-report-non-constructor-local-class-heritage.md` is related
  but covers local non-constructor heritage values.
- `issues/open/5315-report-class-extends-interface-diagnostics.md` is related
  but covers class-extends-interface diagnostics.
- `issues/done/5225-w0-typed-wat-writer.md` is related but
  covers qualified heritage implementation.
- No exact open issue covered namespace-augmented static-side inheritance
  compatibility, so issue 5331 was created.

## Smart triage

Generated on 2026-05-07.

Fresh focused coverage:

```text
executed=1
build_pass=1
unsupported=0
reference/typescript/tests/cases/compiler/clodulesDerivedClasses.ts: build_pass
```

Fresh triage:

```text
### Smart triage: Build pass: clodulesDerivedClasses

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/clodulesDerivedClasses.ts
```

Compiler evidence:

```text
tokens: ok through class Shape, namespace Shape.Utils, class Path extends Shape, and namespace Path.Utils
ast/resolved: ok; ClassDecl Shape and ClassDecl Path extends Shape retained
```

TypeScript oracle evidence:

```text
TS2417: Class static side 'typeof Path' incorrectly extends base class static side 'typeof Shape'.
Types of property 'Utils' are incompatible.
Property 'convert' is missing in type 'typeof Path.Utils' but required in type 'typeof Shape.Utils'.
```

Additional oracle diagnostics TS2564 and TS2322 are out of the split child scope.

Split child:
`issues/open/5331-report-class-namespace-static-side-inheritance-diagnostic.md`.

## Completion evidence

Closed as split on 2026-05-07.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/clodulesDerivedClasses.ts
result: pass; current compiler build-passes, TypeScript oracle reports TS2417
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/clodulesDerivedClasses.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0
date: 2026-05-07
```

Remaining risks:

- TS2564 strict property initialization and TS2322 null assignability diagnostics
  remain outside issue 5331 scope.
