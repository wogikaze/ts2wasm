---
id: 3319
title: "Implement Moduleaugmentationglobal Parser Syntax"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated parser-syntax bucket as superseded by the completed issue
400 ambient declaration erasure/rejection boundary.

## Problem

Fresh triage shows both affected files stop at `declare global { ... }`, which
the compiler already rejects with a precise issue-400
`UnsupportedTypeScriptSyntax` diagnostic.

Problem: `moduleAugmentationGlobal-parser-syntax` duplicates the completed
ambient global declaration boundary rather than describing a new executable
parser feature.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationGlobal --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationGlobal4.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationGlobal6.ts
```

## Desired final state

This generated bucket is closed. Do not implement from this issue; the current
observed behavior is covered by `issues/done/400-implement-ambient-declaration-erasure-boundary.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below.
- [x] Confirm whether existing open/done issues already cover this bucket.
- [x] Close the bucket as superseded by issue 400.
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence.

Out of scope:

- Direct implementation from this generated bucket.
- Reopening the completed ambient declaration boundary.
- Bare `global { ... }` handling, which is separately tracked by issue 5408.

## Affected paths

Expected:

- `issues/done/400-implement-ambient-declaration-erasure-boundary.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing owner contains the relevant boundary contract.
- [x] Triage evidence includes failing path, diagnostic code, source context, visible symbols, and TypeScript AST evidence.
- [x] Completion evidence names the exact reference paths and diagnostic boundary.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationGlobal --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationGlobal4.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationGlobal6.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleAugmentationGlobal4.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationGlobal6.ts`

## Duplicate detection

- `issues/done/400-implement-ambient-declaration-erasure-boundary.md` owns the
  ambient global declaration rejection boundary.

## Smart triage

### `moduleAugmentationGlobal4.ts`

The source contains two virtual sections with `declare global` declarations and
`export {};`:

```ts
declare global {
    interface Something {x}
}
export {};
```

The compiler tokenizes the file, then AST/resolved both report:

```text
UnsupportedTypeScriptSyntax: issue-400: ambient global declarations are not supported in this erasure slice at 109..115
```

TypeScript AST classifies the construct as `ModuleDeclaration`.

### `moduleAugmentationGlobal6.ts`

The source is the minimal form:

```ts
declare global {
    interface Array<T> { x }
}
```

The compiler tokenizes the file, then AST/resolved both report:

```text
UnsupportedTypeScriptSyntax: issue-400: ambient global declarations are not supported in this erasure slice at 47..53
```

TypeScript AST classifies the construct as `ModuleDeclaration`, and the
TypeScript oracle reports TS2669 for invalid global augmentation placement.

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationGlobal --detail --no-dashboard-data
result: pass; executed=11, build_pass=4, unsupported=7
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationGlobal4.ts
result: pass; current blocker is issue-400 ambient global declaration boundary
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationGlobal6.ts
result: pass; current blocker is issue-400 ambient global declaration boundary
date: 2026-05-08
```

Remaining risks:

- none for this generated bucket; broader global augmentation improvements are tracked separately by issue 5408.
