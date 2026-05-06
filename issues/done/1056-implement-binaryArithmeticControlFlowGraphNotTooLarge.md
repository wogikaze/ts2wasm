---
id: 1056
title: "Implement Binaryarithmeticcontrolflowgraphnottoolarge"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5171]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage binaryArithmeticControlFlowGraphNotTooLarge across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `binaryArithmeticControlFlowGraphNotTooLarge` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: binaryArithmeticControlFlowGraphNotTooLarge has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5171-accept-unsigned-32-bit-hex-literals.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts`

## Duplicate detection

- `issues/done/243-implement-numeric-literal-separator-parser.md` is not a match: it covers numeric separators, not large hexadecimal literal magnitude.
- `issues/open/059-implement-parser-syntax-extensions.md` is only a broad parser umbrella and is not an implementation-ready owner for this exact failure.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Failure: `invalid number literal: number too large to fit in target type at 298..308`
- Source context: `d = ((a & 0xefcdab89) | (~a & 0x98badcfe)) + blocks[1] + 271733878;`
- Visible symbols before failure: `foo`, `a`
- Compiler evidence: token, AST, and resolved dumps fail at lexing before any parser or control-flow evidence is available.
- TypeScript oracle: accepts the file with no diagnostics; AST path reaches `FirstLiteralToken` for `0xefcdab89` inside a binary expression.
- Superseding child: `issues/open/5171-accept-unsigned-32-bit-hex-literals.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts
result: pass; current blocker identified as large hexadecimal numeric literal lexing, split to issue 5171
date: 2026-05-06
```

Remaining risks:

- The reference file is large and likely has additional bitwise, shift, compound assignment, and control-flow blockers after issue 5171 advances past the first lexer failure.
