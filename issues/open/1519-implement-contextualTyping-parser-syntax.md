---
id: 1519
title: "Implement Contextualtyping Parser Syntax"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1519.

## Summary

Triage contextualTyping-parser-syntax across 9 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 9 cases fail in directory `contextualTyping-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTyping-parser-syntax has 9 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTyping10.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTyping10.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm the current reference window is stale
- [x] Close the bucket without creating child issues because all affected files build-pass
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

- [x] Duplicate candidates below are confirmed as stale
- [x] Representative command contains exact `mise run reference-triage -- ...` command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] No child issue is required because the exact affected paths now build-pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 18
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTyping10.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTyping10.ts
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

- `reference/typescript/tests/cases/compiler/contextualTyping10.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping11.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping15.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping14.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping12.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping24.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping3.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping4.ts`
- `reference/typescript/tests/cases/compiler/contextualTyping5.ts`

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/open/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/663-implement-arrayAssignmentTest-parser-syntax.md` - Implement Arrayassignmenttest Parser Syntax (same feature label, title overlap)
- `issues/open/734-implement-assignmentCompatability-parser-syntax.md` - Implement Assignmentcompatability Parser Syntax (same feature label, title overlap)
- `issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md` - Implement Asyncfunctionreturntype Parser Syntax (same feature label, title overlap)
- `issues/open/767-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/open/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/open/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)

## Smart triage

Date: 2026-05-07

Command:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTyping10.ts
```

Result: build pass.

Current diagnostic:

```text
BuildPass: ts2wasm build succeeded
feature_label: build-pass
```

Representative source context:

```ts
class foo { public bar:{id:number;}[] = [{id:1}, {id:2}]; }
```

Compiler evidence:

- tokens: ok
- ast: ok; class declaration `foo` parses and erases the TypeScript-only field
  type annotation
- resolved: ok; class `foo` is present with no constructor/methods/statics
- TypeScript oracle: ok, diagnostics `[]`

Coverage evidence:

The broader contextual typing coverage run on 2026-05-07 shows all nine paths
listed in this bucket as `build_pass`:

- `contextualTyping10.ts`
- `contextualTyping11.ts`
- `contextualTyping15.ts`
- `contextualTyping14.ts`
- `contextualTyping12.ts`
- `contextualTyping24.ts`
- `contextualTyping3.ts`
- `contextualTyping4.ts`
- `contextualTyping5.ts`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closed as stale build-pass bucket

Validation result:

```text
command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTyping --detail --no-dashboard-data
result: pass; the nine paths listed in this bucket are all build_pass
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTyping10.ts
result: pass; BuildPass with TypeScript diagnostics []
date: 2026-05-07
```

Remaining risks:

- Other contextual typing paths outside this generated bucket still have open
  unsupported diagnostics and remain tracked separately.
