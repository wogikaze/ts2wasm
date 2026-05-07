---
id: 3360
title: "Implement Modulepreservetoplevelawait"
type: maintenance
area: reference/triage
class: superseded
priority: P2
depends_on: [5146]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as superseded by
`issues/open/5146-report-for-await-context-errors-before-async-runtime.md`.
Fresh triage shows the current first blocker is the same `for await...of`
context-before-runtime diagnostic family.

## Problem

Reference test results show 1 case failing in directory
`modulePreserveTopLevelAwait` with diagnostics: runtime-subset. Fresh triage
shows:

```text
UnsupportedRuntimeSubset: issue-230: `for await...of` async iteration requires Promise and async iterator runtime semantics, which are not supported in this milestone at 84..93
```

Problem: this generated bucket duplicates the existing context-diagnostic owner
in issue 5146.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/modulePreserveTopLevelAwait1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserveTopLevelAwait1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5146
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closure

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing issue 5146 owns the current `for await...of` context-before-runtime diagnostic
- [x] This closure includes failing path, diagnostic code, source context, visible symbols, parser/TypeScript AST evidence
- [x] No child issue is needed from 3360 because the current blocker is already implementation-ready in issue 5146

## Validation

Required commands for this closure:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserveTopLevelAwait1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserveTopLevelAwait1.ts
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Not run:

- Cargo gates; no Rust source changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by issue 5146

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/modulePreserveTopLevelAwait1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserveTopLevelAwait1.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserveTopLevelAwait1.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/modulePreserveTopLevelAwait1.ts: UnsupportedSyntax: import-export
```

Current diagnostic:

```text
UnsupportedRuntimeSubset: issue-230: `for await...of` async iteration requires Promise and async iterator runtime semantics, which are not supported in this milestone at 84..93
```

Source context:

```ts
for await (const x of []) {}
await Promise.resolve();

export {};
```

Compiler evidence:

```text
tokens: ok; For, Await, const binding, array literal, top-level Await, Promise.resolve, and export {} are present
ast/resolved: fail at `for await` with issue-230 runtime-subset diagnostic
visible symbols: []
```

TypeScript oracle:

```text
AST topLevel includes ForOfStatement `for await (const x of []) {}`,
ExpressionStatement `await Promise.resolve();`, and ExportDeclaration
`export {};`.
Diagnostics include TS1432 for top-level `for await` context and TS1378 for
top-level `await` context.
```

Superseding issue:

- `issues/open/5146-report-for-await-context-errors-before-async-runtime.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserveTopLevelAwait1.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserveTopLevelAwait1.ts
result: pass; current blocker is for-await context-before-runtime diagnostic, superseded by issue 5146
date: 2026-05-08
```

Remaining risks:

- After issue 5146 lands, this reference may expose issue 5147's plain
  top-level `await` context diagnostic, top-level await module configuration,
  Promise runtime support, or module-preserve emit parity blockers.
