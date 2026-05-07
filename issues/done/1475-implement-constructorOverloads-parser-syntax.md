---
id: 1475
title: "Implement Constructoroverloads Parser Syntax"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: [5334, 5359]
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed by mapping the current `constructorOverloads-parser-syntax` failures to
focused constructor-diagnostic owners.

Fresh triage shows the bucket is no longer a parser-syntax failure. Tokens and
AST succeed for all affected files, and the current blocker is
`DuplicateFunction: duplicate constructor definition`.

## Problem

Reference test results originally showed 4 parser-syntax failures for
constructor overload cases. Current evidence shows two distinct current
blockers:

- `constructorOverloads2.ts` and `constructorOverloads3.ts`: valid bodyless
  constructor overload signatures are treated as duplicate constructor
  implementations; owned by issue 5334.
- `constructorOverloads1.ts` and `constructorOverloads8.ts`: invalid multiple
  constructor implementations need TS2392-equivalent source-spanned diagnostics;
  split to issue 5359.

Problem: this generated parser-syntax bucket needs focused ownership instead
of direct implementation.

## Current failure

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorOverloads --detail --no-dashboard-data
```

Observed affected files on 2026-05-07:

```text
constructorOverloads1.ts: DuplicateFunction duplicate constructor definition
constructorOverloads2.ts: DuplicateFunction duplicate constructor definition
constructorOverloads3.ts: DuplicateFunction duplicate constructor definition
constructorOverloads8.ts: DuplicateFunction duplicate constructor definition
```

Representative valid-overload reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads3.ts
```

Representative invalid-multiple-implementation reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads8.ts
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5334 for
valid bodyless constructor overload signatures and issue 5359 for TS2392-style
multiple constructor implementation diagnostics.

## Scope

In scope:

- [x] Inspect fresh triage for all affected files
- [x] Confirm parser succeeds and current failures are duplicate-constructor diagnostics
- [x] Map valid bodyless constructor overloads to issue 5334
- [x] Split invalid multiple constructor implementation diagnostics to issue 5359

Out of scope:

- Direct implementation from this generated bucket
- Broad parser-syntax work
- Runtime constructor dispatch or overload resolution

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/ir/src/`
- focused parser/resolver diagnostics

Do not touch:

- backend/runtime code unless focused implementation triage proves lowering is reached

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing issue 5334 owns valid bodyless constructor overload signature `DuplicateFunction`
- [x] Child issue 5359 owns invalid multiple constructor implementation TS2392 diagnostics
- [x] Exact reproduction commands and representative AST/oracle evidence are preserved

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorOverloads --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads2.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads3.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads8.ts
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

- [x] existing: `issues/open/5334-parse-class-constructor-overload-signatures.md`
- [x] created: `issues/open/5359-report-multiple-constructor-implementation-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorOverloads1.ts`
- `reference/typescript/tests/cases/compiler/constructorOverloads2.ts`
- `reference/typescript/tests/cases/compiler/constructorOverloads3.ts`
- `reference/typescript/tests/cases/compiler/constructorOverloads8.ts`

## Duplicate detection

- `issues/open/5334-parse-class-constructor-overload-signatures.md` is an
  exact owner for valid bodyless constructor signatures in
  `constructorOverloads2.ts` and `constructorOverloads3.ts`.
- No exact issue existed for TS2392-style multiple constructor implementation
  diagnostics in `constructorOverloads1.ts` / `constructorOverloads8.ts`, so
  issue 5359 was created.

## Smart triage

Generated 2026-05-07.

```text
constructorOverloads1.ts: DuplicateFunction; TypeScript TS2392 for multiple constructor implementations
constructorOverloads2.ts: DuplicateFunction; TypeScript ok, valid overload signatures
constructorOverloads3.ts: DuplicateFunction; TypeScript TS2377 later after valid overload signatures are accepted
constructorOverloads8.ts: DuplicateFunction; TypeScript TS2392 for multiple constructor implementations
```

## Completion evidence

Commits:

- filled by local commit that moves this issue to `done/`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorOverloads --detail --no-dashboard-data
result: pass; current affected files are duplicate-constructor diagnostics, not parser-syntax
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads1.ts
result: pass; reproduced DuplicateFunction and TS2392 oracle evidence
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads2.ts
result: pass; reproduced DuplicateFunction for TypeScript-ok valid overloads
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads3.ts
result: pass; reproduced DuplicateFunction before later TS2377 derived-constructor diagnostic
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads8.ts
result: pass; reproduced DuplicateFunction and TS2392 oracle evidence
date: 2026-05-07
```

Remaining risks:

- implementation remains tracked by issues 5334 and 5359
