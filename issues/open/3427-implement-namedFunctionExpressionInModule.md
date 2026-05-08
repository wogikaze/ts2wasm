---
id: 3427
title: "Implement Namedfunctionexpressioninmodule"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Closed as a stale generated bucket.

Fresh focused coverage and triage show
`namedFunctionExpressionInModule.ts` now returns `build_pass`, and the
TypeScript oracle reports no diagnostics. There is no current compiler blocker
to split into an implementation-ready child issue.

## Problem

Reference test results show 1 cases fail in directory `namedFunctionExpressionInModule` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: namedFunctionExpressionInModule had 1 generated reference failure and
needed smart-triage evidence before implementation starts.

Disposition: no child issue created because the current result is build-pass
and no TypeScript oracle diagnostic is pending for this reference.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionInModule.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionInModule.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionInModule.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionInModule.ts
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

- `reference/typescript/tests/cases/compiler/namedFunctionExpressionInModule.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionInModule.ts --detail --no-dashboard-data

result:
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
reference/typescript/tests/cases/compiler/namedFunctionExpressionInModule.ts: build_pass
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionInModule.ts

result:
BuildPass: ts2wasm build succeeded
TypeScript oracle reports ok with no diagnostics.
```

Source context:

```ts
namespace Variables{
    var x = function bar(a, b, c) {
    }
    x(1, 2, 3);
}
```

Compiler evidence:

```text
tokens: ok through namespace Variables, var x = function bar(a, b, c), and x(1, 2, 3)
ast: ok; namespace body is erased and the final dump is empty
resolved: ok; final resolved dump is empty
coverage: executed=1, build_pass=1, unsupported=0
```

TypeScript oracle evidence:

```text
ok; no diagnostics
binding x has type (a: any, b: any, c: any) => void
parameters a, b, and c are any
```

## Completion evidence

Closed as stale build-pass bucket; no implementation child created.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namedFunctionExpressionInModule.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namedFunctionExpressionInModule.ts
result: pass; BuildPass with TypeScript oracle ok/no diagnostics
date: 2026-05-08
```

Remaining risks:

- none
