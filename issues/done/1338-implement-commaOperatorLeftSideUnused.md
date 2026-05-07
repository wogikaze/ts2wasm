---
id: 1338
title: "Implement Commaoperatorleftsideunused"
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

## Summary

Closed as superseded by `issues/open/5274-parse-general-comma-expressions.md` for the current comma expression parser failure in a switch `case` label.

## Problem

Fresh triage confirms this generated bucket is too broad for direct implementation. The current first blocker is not TS2695 diagnostic compatibility across the full file. Parsing stops at the switch case label:

```ts
switch (arr.length) {
  case 0, 1:
    return "zero or one";
}
```

The parser expects a colon immediately after `0` and reports `UnsupportedSyntax: expected Colon, got Some(Comma) at 179..180`. This is the same comma-expression parser family already tracked by issue 5274.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/commaOperatorLeftSideUnused.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/commaOperatorLeftSideUnused.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5274 instead of splitting a duplicate child
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in issue 5274 and this closure

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
- [x] Superseding issue 5274 contains exact parser failure evidence for this path
- [x] Superseding issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the exact comma-expression parser change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/commaOperatorLeftSideUnused.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/commaOperatorLeftSideUnused.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected: issue metadata only

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commaOperatorLeftSideUnused.ts`

## Duplicate detection

- `issues/open/5274-parse-general-comma-expressions.md` owns the current comma-expression parser failure, now including `case 0, 1:`.
- TS2695 unused-left-side diagnostic compatibility and the later parenthesized comma expressions remain unproven until issue 5274 advances past the case-label parser boundary.

## Smart triage

Generated 2026-05-07.

Command:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commaOperatorLeftSideUnused.ts
```

Result:

```text
Smart triage: Triage parser syntax: commaOperatorLeftSideUnused
Feature label: parser-syntax
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: expected Colon, got Some(Comma) at 179..180
Failure location: line 11, column 5
```

Source context:

```text
 8 |   switch(arr.length) {
 9 |     // Should error
10 |     case 0, 1:
11 |       return 'zero or one';
12 |     default:
13 |       return 'more than one';
14 |   }
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commaOperatorLeftSideUnused.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

Compiler evidence:

- Tokens: ok through typed globals, function `fn`, `let arr: any[] = []`, switch expression, and `case 0, 1:`.
- AST/resolved: fail with `UnsupportedSyntax: expected Colon, got Some(Comma) at 179..180`.
- TypeScript oracle: reports TS2695 "Left side of comma operator is unused and has no side effects" at the `0` operand, proving the case label parses as a comma expression.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closure commit pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commaOperatorLeftSideUnused.ts
result: pass; reproduced comma expression parser failure in switch case label and updated issue 5274
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commaOperatorLeftSideUnused.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-07
```

Remaining risks:

- TS2695 unused-left-side diagnostic compatibility and the later parenthesized comma expressions remain unproven until issue 5274 advances past the case-label parser boundary.
