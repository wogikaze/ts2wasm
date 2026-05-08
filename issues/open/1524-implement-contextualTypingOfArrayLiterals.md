---
id: 1524
title: "Implement Contextualtypingofarrayliterals"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1524.

## Summary

Triage contextualTypingOfArrayLiterals across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypingOfArrayLiterals` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypingOfArrayLiterals has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfArrayLiterals1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfArrayLiterals1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfArrayLiterals1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfArrayLiterals1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5380-report-array-literal-index-signature-element-mismatch.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypingOfArrayLiterals1.ts`

## Duplicate detection

- No exact owner found. Broad issue-211/interface receiver issues exist, but
  this bucket's first actionable blocker is the earlier TypeScript TS2322
  array-literal element mismatch against a numeric index signature.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfArrayLiterals1.ts

result:
UnsupportedSyntax: issue-211: unknown receiver class for method `getDate` at 131..143
feature_label: class
source context: r2.getDate(), where r2 is initialized from x3[1]
tokens: ok
AST: ok; interface is erased, x3 is an Array literal with new Date() and 1
resolved/lowered: issue-211 unknown receiver class for method `getDate`
TypeScript oracle: TS2322 at the numeric literal `1`
```

Representative source:

```ts
interface I {
   [x: number]: Date;
}

var x3: I = [new Date(), 1];
var r2 = x3[1];
r2.getDate();
```

The generated bucket was split to
`issues/open/5380-report-array-literal-index-signature-element-mismatch.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- child issue: `issues/open/5380-report-array-literal-index-signature-element-mismatch.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfArrayLiterals1.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, UnsupportedSyntax/unknown-unsupported
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfArrayLiterals1.ts
result:
pass; reproduced issue-211 getDate fallback and TypeScript TS2322 oracle diagnostic
date:
2026-05-07
```

Remaining risks:

- Implementation remains open in 5380.
