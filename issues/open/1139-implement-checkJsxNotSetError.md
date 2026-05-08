---
id: 1139
title: "Implement Checkjsxnotseterror"
type: spike
area: reference/triage
class: done
priority: P1
depends_on: [5230]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1139.

## Summary

Triage checkJsxNotSetError across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `checkJsxNotSetError` with diagnostics: regexp-literal. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: checkJsxNotSetError has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkJsxNotSetError.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkJsxNotSetError.ts --detail
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

- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- parser/resolver code unless `reference-triage` proves the failure happens before runtime lowering

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkJsxNotSetError.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkJsxNotSetError.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5230-w0-lowered-ir-span-requirement.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkJsxNotSetError.ts`

## Duplicate detection

Fresh duplicate scan found broad RegExp and JSX generated buckets, but no
implementation-ready child for the exact lexer/parser boundary where a JSX
closing tag is scanned as an unterminated RegExp literal.

Related but no-match:

- `issues/open/3777-implement-parseJsxElementInUnaryExpressionNoCrash-regexp-literal.md` is another untriaged generated bucket, not an executable child.
- `issues/open/3125-implement-jsxEmitWithAttributes.md` and adjacent JSX buckets are broad generated buckets.
- `issues/open/5020-implement-regexp-literal.md` owns real RegExp literal support; this representative is JSX syntax falling into RegExp fallback.

## Smart triage

Fresh coverage now labels the representative as `jsx`, while smart triage
still reports the concrete lexer diagnostic as `UnsupportedRegExp`. The exact
blocker is JSX element tokenization before RegExp fallback.

### Smart triage: checkJsxNotSetError

- Issue class: `triage-needed`
- Feature label: `jsx`
- Diagnostic: `UnsupportedRegExp` / `unsupported-feature-boundary`
- Current compiler message: `issue-202: unterminated RegExp literal`
- Path: `reference/typescript/tests/cases/compiler/checkJsxNotSetError.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsxNotSetError.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsxNotSetError.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=jsx:1
```

Source context:

```tsx
// @Filename: /foo.jsx
const Foo = () => (
    <div>foo</div>
);
export default Foo;
```

Compiler evidence:

```text
tokens: UnsupportedRegExp issue-202 at </div>
ast: same lexer failure
resolved/lowered: same lexer failure
TypeScript oracle: TS2304 for div, TS2552 for foo, TS1161 for the closing tag
```

Split result:

- `issues/open/5230-w0-lowered-ir-span-requirement.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsxNotSetError.ts --detail --no-dashboard-data
result: pass; reproduced UnsupportedSyntax/jsx bucket
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsxNotSetError.ts
result: pass; reproduced UnsupportedRegExp issue-202 at JSX closing tag and split to issue 5230
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5230 may expose JSX diagnostic parity or absolute import resolution blockers.
