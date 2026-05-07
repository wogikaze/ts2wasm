---
id: 1447
title: "Implement Constenumerrors"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5351]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Closed this generated const-enum errors bucket after splitting the current
large decimal integer literal lexer boundary to
`issues/open/5351-accept-large-decimal-integer-number-literals.md`.

## Problem

Reference test results show 1 cases fail in directory `constEnumErrors` with diagnostics: enum. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: constEnumErrors has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constEnumErrors.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constEnumErrors.ts --detail
```

## Desired final state

This generated bucket is closed. Implement the current lexer boundary from
`issues/open/5351-accept-large-decimal-integer-number-literals.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one observable lexer boundary into an implementation-ready child issue
- [x] Preserve exact reproduction commands and representative diagnostic evidence

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
- [x] Child issue contains exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, and TypeScript evidence
- [x] Child issue acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constEnumErrors.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constEnumErrors.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5351-accept-large-decimal-integer-number-literals.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constEnumErrors.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-07.

- Path: `reference/typescript/tests/cases/compiler/constEnumErrors.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `invalid number literal: number too large to fit in target type at 524..540`
- Current failing source: `A = 9007199254740992`
- Compiler dumps fail before token output, AST, or resolved construction.
- TypeScript oracle parses the file and reports later enum diagnostics including TS2567, TS2651, TS2474, TS2475, TS2476, TS2477, and TS2478.
- Superseding child: `issues/open/5351-accept-large-decimal-integer-number-literals.md`

Nearby non-owners:

- `issues/open/5171-accept-unsigned-32-bit-hex-literals.md` covers large hex masks.
- `issues/open/5216-accept-large-decimal-exponent-number-literals.md` covers exponent notation.
- `issues/open/300-support-abc451-large-integer-number-boundary.md` covers runtime representation after parsing.
- `issues/open/5184-parse-const-enum-declarations.md` covers the const-enum parser boundary, not the current lexer failure.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumErrors.ts
result: pass; current blocker identified as large decimal integer literal lexing, split to issue 5351
date: 2026-05-07
```

Remaining risks:

- Later triage may expose const-enum parser, enum semantic diagnostics, or runtime number-model work after issue 5351 accepts the literal.
