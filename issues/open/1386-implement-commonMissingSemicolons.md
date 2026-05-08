---
id: 1386
title: "Implement Commonmissingsemicolons"
type: spike
area: reference/triage
class: triage-needed
priority: P2
depends_on: []
blocks: [5240]
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed as superseded by
`issues/open/5240-w2-docs-audit-and-stale-entries.md`.

Fresh focused triage shows this generated bucket currently stops at the same
raw async-arrow parser boundary already owned by issue 5240.

## Problem

Reference test results originally showed 1 case failing in directory
`commonMissingSemicolons` with diagnostics: runtime-subset. Fresh focused
coverage on 2026-05-07 reports `UnsupportedSyntax` / `unknown-unsupported`.

Problem: `commonMissingSemicolons.ts` currently fails at
`const myAsyncArrow1 = async () => 3;` with raw parser diagnostic
`unsupported expression: ... Async` before later typo, missing-semicolon, or
class-member recovery diagnostics can be evaluated.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonMissingSemicolons.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonMissingSemicolons.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
ast/resolved: unsupported expression ... Async ... at 392..393
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5240.
After async arrow parsing advances, this reference path may need fresh triage
for the intended missing-semicolon and typo-diagnostic cases.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing async-arrow parser issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing issue 5240 contains the exact async-arrow parser boundary
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonMissingSemicolons.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonMissingSemicolons.ts
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

- [x] superseded by: `issues/open/5240-w2-docs-audit-and-stale-entries.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commonMissingSemicolons.ts`

## Duplicate detection

- `issues/open/5240-w2-docs-audit-and-stale-entries.md` owns
  `async () => expr` / `(async () => { ... })` parser support and records the
  same raw `unsupported expression: Async` failure family.
- Broader generated async buckets such as 751, 759, and 3758 already appear in
  5240 notes as same-boundary candidates.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage unknown unsupported: commonMissingSemicolons

- Issue class: triage-needed
- Feature label: unknown-unsupported
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/commonMissingSemicolons.ts
```

Source context:

```text
10 | // Arrow functions don't (yet?) parse as nicely as standalone functions.
11 | // Eventually it would be good to get them the same "did you mean" for typos such as "asyncd".
12 | const myAsyncArrow1 = async () => 3;
13 | const myAsyncArrow2 = asyncd () => 3;
14 |
15 | class MyClass1 {}
```

Compiler evidence:

```text
tokens: ok; Const, Ident("myAsyncArrow1"), Equal, Async, LeftParen, RightParen, Arrow are present
ast: UnsupportedSyntax unsupported expression ... Async ... at 392..393
resolved: same UnsupportedSyntax
visible symbols: async function declarations and binding myAsyncArrow1 before failure
```

TypeScript oracle:

```text
TypeScript accepts the valid async arrow and reports later typo diagnostics,
including TS2304 for asyncd and TS1005 at the typo arrow. The current compiler
does not reach those later diagnostics because it rejects the valid async arrow.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonMissingSemicolons.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonMissingSemicolons.ts
result: raw async-arrow parser boundary; superseded by issue 5240
date: 2026-05-07
```

Remaining risks:

- After issue 5240 advances this path, fresh triage may reveal separate
  typo-diagnostic or missing-semicolon recovery blockers.
