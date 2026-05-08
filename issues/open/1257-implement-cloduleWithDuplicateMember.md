---
id: 1257
title: "Implement Clodulewithduplicatemember"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1257.

## Summary

Triage cloduleWithDuplicateMember across 2 reference test cases and split the
remaining semantic diagnostic gap into an implementation-ready child issue.

## Problem

Reference test results originally showed 2 cases failing in directory
`cloduleWithDuplicateMember` with diagnostics: import-export. Fresh focused
coverage on 2026-05-07 shows both cases now build-pass.

Problem: the stale build blocker is gone, but TypeScript oracle reports TS2300
duplicate identifier diagnostics for class/namespace duplicate member names.
That narrower semantic follow-up is split to issue 5329.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5329 for
the remaining duplicate identifier diagnostic parity.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad declaration merge semantics beyond duplicate member diagnostics

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember2.ts
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

- [x] created: `issues/open/5329-report-class-namespace-duplicate-member-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember2.ts`
- `reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember1.ts`

## Duplicate detection

- `issues/done/5307-report-var-function-duplicate-identifier-diagnostics.md`
  is related but covers var/function duplicate identifiers.
- `issues/open/343-implement-duplicate-local-detection.md` is related but does
  not own TypeScript-specific class/namespace duplicate member diagnostics.
- No exact open issue covered static class members/accessors colliding with
  exported namespace members, so issue 5329 was created.

## Smart triage

Generated on 2026-05-07.

Focused coverage:

```text
executed=2
build_pass=2
unsupported=0
reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember2.ts: build_pass
reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember1.ts: build_pass
```

### cloduleWithDuplicateMember1

Fresh triage:

```text
BuildPass: ts2wasm build succeeded
```

Compiler evidence:

```text
tokens: ok through class getter/static getter/static method and namespace exports
ast/resolved: ClassDecl C retained with get x, static::get x, static::foo
```

TypeScript oracle diagnostics:

```text
TS2300 Duplicate identifier 'x'
TS2300 Duplicate identifier 'foo'
TS2300 Duplicate identifier 'x'
TS2300 Duplicate identifier 'foo'
TS2300 Duplicate identifier 'x'
```

### cloduleWithDuplicateMember2

Fresh triage:

```text
BuildPass: ts2wasm build succeeded
```

Compiler evidence:

```text
tokens: ok through class setter/static setter and namespace exports
ast/resolved: ClassDecl C retained with set x and static::set y
```

TypeScript oracle diagnostics:

```text
TS2300 Duplicate identifier 'x'
TS2300 Duplicate identifier 'x'
```

Split child: `issues/open/5329-report-class-namespace-duplicate-member-diagnostics.md`.

## Completion evidence

Closed as split on 2026-05-07.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember1.ts
result: pass; current compiler build-passes, TypeScript oracle reports TS2300 duplicates
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember2.ts
result: pass; current compiler build-passes, TypeScript oracle reports TS2300 duplicates
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember --detail --no-dashboard-data
result: pass; executed=2, build_pass=2, unsupported=0
date: 2026-05-07
```

Remaining risks:

- Full declaration merge semantics remain out of issue 5329 scope.
