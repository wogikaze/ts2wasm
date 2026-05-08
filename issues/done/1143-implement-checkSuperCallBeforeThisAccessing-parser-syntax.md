---
id: 1143
title: "Implement Checksupercallbeforethisaccessing Parser Syntax"
type: spike
area: frontend/syntax
class: done
priority: P1
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1143.

## Summary

Triage checkSuperCallBeforeThisAccessing-parser-syntax across 5 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 5 cases fail in directory `checkSuperCallBeforeThisAccessing-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: checkSuperCallBeforeThisAccessing-parser-syntax has 5 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing1.ts --detail
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
- [x] Existing child issue contains exact `reference-triage` commands
- [x] Existing child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Existing child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 10
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] updated: 

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing1.ts`
- `reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing4.ts`
- `reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing3.ts`
- `reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing2.ts`
- `reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing5.ts`

## Duplicate detection

Fresh triage found only one current unsupported case in this generated bucket:
`checkSuperCallBeforeThisAccessing4.ts`. The current diagnostic is the same
non-identifier call-callee lowering boundary tracked by
.

Related but no-match:

- Broad parser syntax buckets are not exact because tokens and AST already
  succeed for the representative failing file.
- Broad class buckets are not exact because the current failure is a call
  expression lowering boundary.

## Smart triage

Fresh triage shows four of the five original files now build, and the remaining
unsupported file is an arrow-function IIFE callee in a derived constructor.

### Smart triage: checkSuperCallBeforeThisAccessing bucket

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing --detail --no-dashboard-data
```

Coverage result:

```text
executed=9
build_pass=7
unsupported=2
unsupported_diagcodes=UnsupportedSyntax:2
unsupported_features=unknown-unsupported:2
```

For the five files owned by this bucket:

```text
checkSuperCallBeforeThisAccessing1.ts: build_pass
checkSuperCallBeforeThisAccessing2.ts: build_pass
checkSuperCallBeforeThisAccessing3.ts: build_pass
checkSuperCallBeforeThisAccessing4.ts: UnsupportedSyntax: unknown-unsupported
checkSuperCallBeforeThisAccessing5.ts: build_pass
```

Representative failing reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing4.ts
```

Source context:

```ts
class Derived extends Based {
    constructor() {
        (() => {
            this;  // No error
        })();
        super();
        super();
        this.x = 10;
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; the failing expression is Call(callee=ArrowFn, args=[])
resolved/lowered: UnsupportedSyntax: only identifier calls are supported in expression context at 248..301
TypeScript oracle: ok, no diagnostics
```

Split result:

- folded into 

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing --detail --no-dashboard-data
result: pass; confirmed 1/2/3/5 are build_pass and 4 is current unsupported in this bucket
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing4.ts
result: pass; reproduced non-identifier call callee boundary and folded into issue 5163
date: 2026-05-06
```

Remaining risks:

- `checkSuperCallBeforeThisAccessing9.ts` remains unsupported through issue 5233, but it belongs to issue 1142 rather than this bucket.
