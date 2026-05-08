---
id: 3503
title: "Implement Newnamesinglobalaugmentations"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated bucket as superseded by the completed issue 400 ambient
declaration erasure/rejection boundary.

## Problem

Fresh triage shows the only affected reference file stops at
`declare global { ... }`, which the compiler already rejects with a precise
issue-400 `UnsupportedTypeScriptSyntax` diagnostic.

Problem: `newNamesInGlobalAugmentations` is not a separate implementation
slice; the current observable blocker duplicates the completed ambient global
declaration boundary.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newNamesInGlobalAugmentations1.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newNamesInGlobalAugmentations1.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Do not implement from this issue; the current
observed behavior is covered by
`issues/done/400-implement-ambient-declaration-erasure-boundary.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below.
- [x] Confirm whether existing open/done issues already cover this bucket.
- [x] Close the bucket as superseded by issue 400.
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence.

Out of scope:

- Direct implementation from this generated bucket
- Reopening the completed ambient declaration boundary
- Bare `global { ... }` handling, which is separately tracked by issue 5408

## Affected paths

Expected:

- `issues/done/400-implement-ambient-declaration-erasure-boundary.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing owner contains the relevant boundary contract.
- [x] Triage evidence includes failing path, diagnostic code, source context, visible symbols, and TypeScript AST evidence.
- [x] Completion evidence names the exact reference path and diagnostic boundary.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newNamesInGlobalAugmentations1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newNamesInGlobalAugmentations1.ts
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

- `reference/typescript/tests/cases/compiler/newNamesInGlobalAugmentations1.ts`

## Duplicate detection

- `issues/done/400-implement-ambient-declaration-erasure-boundary.md` owns the
  ambient global declaration rejection boundary.
- `issues/open/3318-implement-moduleAugmentationGlobal-import-export.md` and
  `issues/open/3319-implement-moduleAugmentationGlobal-parser-syntax.md` record
  the same closure rule for neighboring `declare global { ... }` generated
  buckets.
- `issues/open/5408-parse-bare-global-augmentation-blocks.md` is not an owner
  for this file because it is intentionally scoped to bare `global { ... }`,
  while this file uses `declare global { ... }`.

## Smart triage

Fresh triage on 2026-05-08:

```text
Diagnostic: UnsupportedTypeScriptSyntax / unsupported-feature-boundary
Path: reference/typescript/tests/cases/compiler/newNamesInGlobalAugmentations1.ts
message: issue-400: ambient global declarations are not supported in this erasure slice
```

The source contains an erased ambient namespace followed by a global
augmentation:

```ts
declare namespace M.M1 {
    export let x: number;
}
declare global {
    interface SymbolConstructor {
        observable: symbol;
    }
    class Cls {x}
    let [a, b]: number[];
    export import X = M.M1.x;
}
```

Compiler evidence:

```text
coverage: executed=1, build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedTypeScriptSyntax:1
unsupported_features=parser-syntax:1

ast/resolved:
UnsupportedTypeScriptSyntax: issue-400: ambient global declarations are not supported in this erasure slice at 139..145
```

The triage headline location also reported `73..79` at line 7 column 5, while
the compiler AST/resolved dump points to the `global` token at `139..145`.
Both outputs use the same issue-400 diagnostic and boundary.

TypeScript oracle evidence:

```text
diagnostics: []
topLevel: ExportDeclaration; ModuleDeclaration declare namespace M.M1;
ModuleDeclaration declare global; Symbol.observable; new Cls().x;
let c = a + b + X;
binding hints: x: number; c: number; number-add-fast-path for a + b + X
```

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newNamesInGlobalAugmentations1.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedTypeScriptSyntax:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newNamesInGlobalAugmentations1.ts
result: pass; current blocker is issue-400 ambient global declaration boundary
date: 2026-05-08
```

Remaining risks:

- none for this generated bucket; broader bare global augmentation handling is
  tracked separately by issue 5408.
