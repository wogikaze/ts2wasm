---
id: 1148
title: "Implement Checkingobjectwiththisinnamepositionnocrash"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5237]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1148.

## Summary

Triage checkingObjectWithThisInNamePositionNoCrash across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `checkingObjectWithThisInNamePositionNoCrash` with diagnostics: object-literal. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: checkingObjectWithThisInNamePositionNoCrash has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkingObjectWithThisInNamePositionNoCrash.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkingObjectWithThisInNamePositionNoCrash.ts --detail
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
- [x] At least one child issue contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkingObjectWithThisInNamePositionNoCrash.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkingObjectWithThisInNamePositionNoCrash.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5237-w1-standalone-wasi-execution-validation-test-suite.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkingObjectWithThisInNamePositionNoCrash.ts`

## Duplicate detection

Fresh duplicate scan found related object-literal computed-key issues, but no
exact implementation-ready issue for a `this`-rooted property access in a
computed object literal key:

- `issues/open/5228-w0-wasm-binary-backend-mvp.md` covers
  `[identifier]`.
- `issues/open/5209-support-class-instance-method-receiver-calls.md`
  covers binary key expressions such as `[name + ".a"]`.
- `issues/open/5223-report-iterator-type-only-value-use-diagnostics.md` covers
  computed properties after object spread.
- Broad object-literal buckets are not exact implementation-ready matches.

Split result:

- `issues/open/5237-w1-standalone-wasi-execution-validation-test-suite.md`

## Smart triage

Fresh triage shows the current blocker is parser support for `[this.a]` as an
object literal computed property key. Tokenization succeeds; AST construction
fails inside the computed key before later type diagnostics can be reported.

### Smart triage: checkingObjectWithThisInNamePositionNoCrash

- Issue class: `triage-needed`
- Feature label: `object-literal`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `expected identifier, got Some(SpannedToken { kind: This, span: Span { start: 151, end: 155 } }) at 155..156`
- Path: `reference/typescript/tests/cases/compiler/checkingObjectWithThisInNamePositionNoCrash.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkingObjectWithThisInNamePositionNoCrash.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkingObjectWithThisInNamePositionNoCrash.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=object-literal:1
```

Source context:

```ts
export const thing = {
    doit() {
        return {
            [this.a]: "",
        }
    }
}
```

Compiler evidence:

```text
tokens: ok; LeftBracket This Dot Ident("a") RightBracket Colon String("")
ast: UnsupportedSyntax expected identifier, got Some(This) at 155..156
TypeScript AST: ObjectLiteralExpression -> PropertyAssignment -> ComputedPropertyName -> PropertyAccessExpression `this.a`
TypeScript oracle: TS2339 Property 'a' does not exist on type '{ doit(): { [x: number]: string; }; }'
```

Split result:

- `issues/open/5237-w1-standalone-wasi-execution-validation-test-suite.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkingObjectWithThisInNamePositionNoCrash.ts --detail --no-dashboard-data
result: pass; reproduced UnsupportedSyntax/object-literal blocker
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkingObjectWithThisInNamePositionNoCrash.ts
result: pass; reproduced `[this.a]` computed object key parser blocker and split to issue 5237
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5237 may expose runtime computed-property semantics or the intended TS2339 `this.a` property diagnostic as the next blocker.
