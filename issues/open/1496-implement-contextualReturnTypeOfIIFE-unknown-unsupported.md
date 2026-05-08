---
id: 1496
title: "Implement Contextualreturntypeofiife Unknown Unsupported"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextualReturnTypeOfIIFE-unknown-unsupported across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualReturnTypeOfIIFE-unknown-unsupported` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualReturnTypeOfIIFE-unknown-unsupported has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5240-w2-docs-audit-and-stale-entries.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE.ts`

## Duplicate detection

- `issues/open/694-implement-arrayToLocaleStringES-unknown-unsupported.md` - Implement Arraytolocalestringes Unknown Unsupported (same feature label, title overlap)

## Smart triage

Fresh triage on 2026-05-07 shows this generated unknown-unsupported bucket is
the raw async-arrow parser boundary already owned by
`issues/open/5240-w2-docs-audit-and-stale-entries.md`.

Current diagnostic:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 91, end: 96 } }) at 97..98
```

Source context:

```ts
const test1: Promise<[one: number, two: string]> = (async () => {
    return [1, 'two'];
})();
```

Compiler evidence:

```text
tokens: ok through LeftParen, Async, LeftParen, RightParen, Arrow, body, call
ast/resolved: fail before AST construction with raw Async parser error
TypeScript oracle: diagnostics=[]; ArrowFunction under ParenthesizedExpression
```

No child issue was created because 5240 already accepts `(async () => { ... })`
in expression position and explicitly asks reference cases to advance past the
raw `unsupported expression: Async` parser failure.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` (filled by commit that moves this issue)

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE.ts
result: pass; raw async-arrow parser blocker folded into issue 5240
date: 2026-05-07
```

Remaining risks:

- Contextual return type and Promise behavior remain hidden until issue 5240
  advances this file past the async-arrow parser boundary.
