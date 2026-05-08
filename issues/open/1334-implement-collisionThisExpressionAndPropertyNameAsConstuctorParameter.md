---
id: 1334
title: "Implement Collisionthisexpressionandpropertynameasconstuctorparameter"
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
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1334.

## Summary

Closed as superseded by `issues/open/5334-parse-class-constructor-overload-signatures.md` for the current class constructor overload `DuplicateFunction` blocker.

## Problem

Fresh triage confirms this generated bucket is too broad for direct implementation. The current first blocker is not the later `_this` property-name/parameter collision behavior. Tokens and AST advance through ordinary constructors and parameter-property constructors, then validation/resolution stops on bodyless constructor overload signatures:

```ts
class Foo4 {
    constructor(_this: number);
    constructor(_this: string);
    constructor(_this: any) {
        var lambda = () => {
            return x => this;
        }
    }
}
```

The failing `DuplicateFunction: duplicate constructor definition` boundary is already tracked by issue 5334.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndPropertyNameAsConstuctorParameter.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndPropertyNameAsConstuctorParameter.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5334 instead of splitting a duplicate child
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in issue 5334 and this closure

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
- [x] Superseding issue 5334 contains exact constructor overload diagnostic evidence for this path
- [x] Superseding issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndPropertyNameAsConstuctorParameter.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndPropertyNameAsConstuctorParameter.ts
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

- `reference/typescript/tests/cases/compiler/collisionThisExpressionAndPropertyNameAsConstuctorParameter.ts`

## Duplicate detection

- `issues/open/5334-parse-class-constructor-overload-signatures.md` owns the current `DuplicateFunction: duplicate constructor definition` blocker for bodyless constructor overload signatures.
- `issues/open/5337-parse-rest-parameter-constructor-overload-signatures.md` is related but covers rest-parameter constructor overload signatures.
- Later `_this` parameter-property collision and lexical `this` behavior remains unproven until issue 5334 advances past the constructor overload failure.

## Smart triage

Generated 2026-05-07.

Command:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndPropertyNameAsConstuctorParameter.ts
```

Result:

```text
Smart triage: Triage duplicate function: collisionThisExpressionAndPropertyNameAsConstuctorParameter
Feature label: duplicate-function
Diagnostic: DuplicateFunction / compiler-diagnostic
Message: duplicate constructor definition
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndPropertyNameAsConstuctorParameter.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=DuplicateFunction:1
unsupported_features=duplicate-function:1
semantic_enabled=0
```

Compiler evidence:

- Tokens: ok through `Foo2`, `Foo3`, `Foo4`, and `Foo5`, including ordinary `_this` constructor parameters, `private _this` parameter properties, constructor overload signatures, and nested arrow returns.
- AST: ok; includes nested `ArrowFn` bodies and parameter-property lowering shape for `private _this`.
- Resolved: fails during validation/resolution with `DuplicateFunction: duplicate constructor definition`.
- TypeScript oracle: accepts the file with no diagnostics and reports parameter/binding hints for each `_this`, `lambda`, and nested arrow parameter.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closure commit pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndPropertyNameAsConstuctorParameter.ts
result: pass; reproduced constructor overload DuplicateFunction and updated issue 5334
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndPropertyNameAsConstuctorParameter.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0 unsupported_diagcodes=DuplicateFunction:1 unsupported_features=duplicate-function:1
date: 2026-05-07
```

Remaining risks:

- Later `_this` parameter-property collision and lexical `this` behavior remains unproven until issue 5334 advances past the constructor overload failure.
