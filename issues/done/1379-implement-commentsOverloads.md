---
id: 1379
title: "Implement Commentsoverloads"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: [5289]
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed after splitting the first executable blocker into
`issues/open/5289-validate-comments-overloads-top-level-functions.md`.

Fresh triage shows the first blocker is not parser syntax. The file parses to
AST, then `validate_ast` reports `DuplicateFunction` for a valid commented
top-level function overload signature group.

## Problem

Reference test results originally showed 1 case failing in directory
`commentsOverloads` with diagnostics: parser-syntax. Fresh focused triage on
2026-05-07 shows tokenization and AST parsing succeed, but duplicate-function
validation treats the second bodyless overload signature as a duplicate
implementation.

Problem: `commentsOverloads.ts` currently reports `DuplicateFunction` for the
second bodyless `function f1(b: string): number;` overload signature.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOverloads.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOverloads.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=DuplicateFunction:1
unsupported_features=duplicate-function:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5289-validate-comments-overloads-top-level-functions.md`; later
interface, method, or constructor overload blockers in the same file should be
triaged separately after the top-level overload blocker advances.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split this bucket into a focused top-level overload validation issue
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue contains the exact commented top-level overload diagnostic family
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, parser AST evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOverloads.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOverloads.ts
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

- [x] created/updated: `issues/open/5289-validate-comments-overloads-top-level-functions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsOverloads.ts`

## Duplicate detection

- `issues/open/5289-validate-comments-overloads-top-level-functions.md` owns
  the exact first blocker: commented top-level overload signatures followed by
  one implementation, currently rejected as `DuplicateFunction`.
- `issues/open/5280-validate-commented-top-level-function-overloads.md` is
  related, but folding this larger reference file into it made the issue too
  large for readiness gates.
- `issues/done/5200-validate-top-level-function-overload-implementations.md`
  is broader and related, but 5280 is the narrower comments/trivia work order.
- Class method and constructor overloads later in `commentsOverloads.ts` are
  adjacent but not the first blocker reached by current triage.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage duplicate function: commentsOverloads

- Issue class: triage-needed
- Feature label: duplicate-function
- Diagnostic: DuplicateFunction / compiler-diagnostic
- Path: reference/typescript/tests/cases/compiler/commentsOverloads.ts
```

Source context:

```text
5 | /** this is signature 1*/
6 | function f1(/**param a*/a: number): number;
7 | function f1(b: string): number;
8 | function f1(aOrb: any) {
9 |     return 10;
10 | }
```

Compiler evidence:

```text
tokens: ok
ast: ok; Function f1(a), Function f1(b), Function f1(aOrb) with body
validate_ast: DuplicateFunction for second bodyless overload signature at 166..174
visible symbols before failure: function f1
```

TypeScript oracle:

```text
ok: true
diagnostics: []
hints include f1 overload signatures and implementation
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOverloads.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=DuplicateFunction:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOverloads.ts
result: DuplicateFunction for valid commented top-level overload signatures; split to issue 5289
date: 2026-05-07
```

Remaining risks:

- Later interface call signatures, class method overloads, and constructor
  overloads in this reference file have not been reached yet.
