---
id: 3574
title: "Implement Nounusedlocals Parser Syntax"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: [5192, 5167]
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage noUnusedLocals-parser-syntax across 3 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage split the three affected paths:

```text
noUnusedLocals_writeOnlyProperty.ts: build_pass
noUnusedLocals_selfReference.ts: issue-5011 class `P` value use
noUnusedLocals_writeOnlyProperty_dynamicNames.ts: unresolved function `Symbol`
```

Problem: the original parser-syntax bucket is stale or superseded by existing
class-value and Symbol builtin owner issues.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty.ts --detail
```

## Desired final state

This generated bucket is closed as stale/superseded. Do not implement directly
from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm current behavior for all three affected paths
- [x] Fold remaining blockers into existing issues 5192 and 5167
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

- [x] Duplicate candidates below are confirmed as no-match, stale, or superseded
- [x] Existing issue 5192 owns `issue-5011` class constructor value evidence
- [x] Existing issue 5167 owns global `Symbol()` call resolution
- [x] The stale build-pass representative is documented

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty_dynamicNames.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty_dynamicNames.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only stale/superseded bucket close.
- `cargo nextest run`; metadata-only stale/superseded bucket close.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into: `issues/open/5192-support-first-class-class-constructor-values.md`
- [x] folded into: `issues/open/5167-support-global-symbol-builtin-call.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty.ts`
- `reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference.ts`
- `reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty_dynamicNames.ts`

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
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)

## Smart triage

### Build pass: noUnusedLocals writeOnlyProperty

```text
path: reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty.ts
coverage: build_pass=1 unsupported=0
triage: BuildPass / pass
TypeScript oracle: diagnostics=[]
```

### Triage class: noUnusedLocals selfReference

```text
path: reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference.ts
diagnostic: UnsupportedSyntax
message: issue-5011: class `P` cannot be used as a value — class runtime is not yet supported at 591..592
source: expression statement `P;`
owner: issues/open/5192-support-first-class-class-constructor-values.md
```

Compiler evidence:

```text
tokens: ok through export {}, functions, classes, enum/type/interface/namespace syntax
ast: ok; `P;` is Expr Ident("P")
resolved: issue-5011 at identifier `P`
TypeScript oracle: diagnostics=[]
```

### Triage function resolution: noUnusedLocals writeOnlyProperty dynamicNames

```text
path: reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty_dynamicNames.ts
diagnostic: UnresolvedFunction
message: unresolved function: `Symbol`
source: const x = Symbol("x")
owner: issues/open/5167-support-global-symbol-builtin-call.md
```

Compiler evidence:

```text
tokens/ast: ok through Symbol("x"), computed private fields, this[x] write, this[y] read
resolved/lowered: UnresolvedFunction for global Symbol
TypeScript oracle: TS2564 definite-assignment diagnostics for computed fields
```

## Completion evidence

Status: done

Commits:

- this local issue-cleanup commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty.ts --detail --no-dashboard-data
result: pass; representative path reports build_pass
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty.ts
result: pass; BuildPass and TypeScript oracle diagnostics=[]
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference.ts --detail --no-dashboard-data
result: pass; current blocker is issue-5011 class value use
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_selfReference.ts
result: pass; folded class value blocker into issue 5192
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty_dynamicNames.ts --detail --no-dashboard-data
result: pass; current blocker is unresolved global Symbol call
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUnusedLocals_writeOnlyProperty_dynamicNames.ts
result: pass; folded Symbol blocker into issue 5167
date: 2026-05-08
```

Remaining risks:

- noUnusedLocals-specific behavior remains hidden for two paths until issues 5192 and 5167 advance.
