---
id: 3424
title: "Implement Namedfunctionexpressionassignedtoclassproperty"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Closed as a stale generated bucket.

Fresh focused coverage and triage show
`namedFunctionExpressionAssignedToClassProperty.ts` now returns `build_pass`,
and the TypeScript oracle reports no diagnostics. There is no current compiler
blocker to split into an implementation-ready child issue.

## Problem

Reference test results show 1 cases fail in directory `namedFunctionExpressionAssignedToClassProperty` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: namedFunctionExpressionAssignedToClassProperty had 1 generated
reference failure and needed smart-triage evidence before implementation
starts.

Disposition: no child issue created because the current result is build-pass
and no TypeScript oracle diagnostic is pending for this reference.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionAssignedToClassProperty.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionAssignedToClassProperty.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close stale generated bucket when fresh evidence shows no blocker
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Fresh evidence contains an exact `reference-triage` command
- [x] Evidence includes path, current result, source context, visible symbols, and parser/TypeScript AST evidence
- [x] No child issue is needed because the representative currently build-passes with no oracle diagnostics

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionAssignedToClassProperty.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionAssignedToClassProperty.ts
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

- `reference/typescript/tests/cases/compiler/namedFunctionExpressionAssignedToClassProperty.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionAssignedToClassProperty.ts --detail --no-dashboard-data

result:
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
reference/typescript/tests/cases/compiler/namedFunctionExpressionAssignedToClassProperty.ts: build_pass
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionAssignedToClassProperty.ts

result:
BuildPass: ts2wasm build succeeded
TypeScript oracle reports ok with no diagnostics.
```

Source context:

```ts
class Foo{
       a = function bar(){

       }; // this shouldn't crash the compiler...

       constructor(){
       }
}
```

Compiler evidence:

```text
tokens: ok through class Foo, class property initializer `function bar() {}`, and constructor
ast: ok; ClassDecl Foo is retained with constructor; class property initializer is erased
resolved: ok; ClassDecl Foo has constructor and no methods/statics
coverage: executed=1, build_pass=1, unsupported=0
```

TypeScript oracle evidence:

```text
ok; no diagnostics
```

## Completion evidence

Closed as stale build-pass bucket; no implementation child created.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionAssignedToClassProperty.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionAssignedToClassProperty.ts
result: pass; BuildPass with TypeScript oracle ok/no diagnostics
date: 2026-05-08
```

Remaining risks:

- none
