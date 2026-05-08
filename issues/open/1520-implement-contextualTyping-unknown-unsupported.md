---
id: 1520
title: "Implement Contextualtyping Unknown Unsupported"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1520.

## Summary

Triage contextualTyping-unknown-unsupported across 14 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 14 cases fail in directory `contextualTyping-unknown-unsupported` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTyping-unknown-unsupported has 14 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTyping18.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTyping18.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm the current reference window is stale
- [x] Close the bucket without creating child issues because the representative numbered contextualTyping files build-pass
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

- [x] Duplicate candidates below are confirmed as stale/no-match
- [x] Representative command contains exact `mise run reference-triage -- ...` command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] No child issue is required because the exact representative path now build-passes

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 28
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTyping18.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTyping18.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTyping18.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping25.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping26.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping27.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping34.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping36.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping35.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping38.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping37.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping40.ts`
- ... and 4 more files

## Duplicate detection

- `issues/open/694-implement-arrayToLocaleStringES-unknown-unsupported.md` - Implement Arraytolocalestringes Unknown Unsupported (same feature label, title overlap)

## Smart triage

Date: 2026-05-07

Command:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTyping18.ts
```

Result: build pass.

Current diagnostic:

```text
BuildPass: ts2wasm build succeeded
feature_label: build-pass
```

Coverage evidence:

The broader contextual typing coverage run on 2026-05-07 shows the numbered
`contextualTypingN.ts` cases listed in this generated bucket now build-pass,
including:

- `contextualTyping18.ts`
- `contextualTyping25.ts`
- `contextualTyping26.ts`
- `contextualTyping27.ts`
- `contextualTyping34.ts`
- `contextualTyping36.ts`
- `contextualTyping35.ts`
- `contextualTyping38.ts`
- `contextualTyping37.ts`
- `contextualTyping40.ts`
- `contextualTyping41.ts`

The same broader run still has unsupported diagnostics for separately named
contextual typing files such as
`contextualTypingFunctionReturningFunction.ts`,
`contextualTypingOfArrayLiterals1.ts`,
`contextualTypingTwoInstancesOfSameTypeParameter.ts`, and
`contextualTypingWithFixedTypeParameters1.ts`; those are not this numbered
`contextualTypingN.ts` stale bucket and remain tracked by neighboring issues.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closed as stale build-pass bucket

Validation result:

```text
command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTyping --detail --no-dashboard-data
result: pass; numbered contextualTypingN paths listed in this bucket are build_pass, while separately named contextualTyping feature files remain tracked elsewhere
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTyping18.ts
result: pass; BuildPass
date: 2026-05-07
```

Remaining risks:

- Other contextual typing paths outside this generated numbered bucket still
  have open unsupported diagnostics and remain tracked separately.
