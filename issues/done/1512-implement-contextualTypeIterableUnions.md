---
id: 1512
title: "Implement Contextualtypeiterableunions"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextualTypeIterableUnions across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypeIterableUnions` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypeIterableUnions has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeIterableUnions.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeIterableUnions.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as stale because the representative path now build-passes
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

- [x] Duplicate candidates below are confirmed; no child issue is needed
- [x] No child issue created because the compiler now reports build_pass
- [x] This issue includes path, diagnostic status, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and build-pass result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeIterableUnions.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeIterableUnions.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current compiler build has no parser blocker on this path

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypeIterableUnions.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh coverage and triage on 2026-05-07 show the generated parser-syntax bucket
is stale. The focused reference path now build-passes.

Current status:

```text
BuildPass: ts2wasm build succeeded
```

Focused coverage:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
unsupported_diagcodes=
unsupported_features=
```

Compiler evidence:

```text
tokens: ok through `declare class DMap<K, V>` with `Iterable<[K, V]> | undefined`
ast: ok; `new DMap([["1", 2]])` plus iterable union let declarations parse
resolved: ok; DMap new expression and i1/i2/i3 array/object expressions resolve
TypeScript oracle: ok, diagnostics=[]
```

No child issue was created because there is no current compiler blocker in the
build path.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: cargo build -p ts2wasm-cli
result: pass
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeIterableUnions.ts --detail --no-dashboard-data
result: pass; build_pass=1, unsupported=0, blocked=0
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeIterableUnions.ts
result: pass; BuildPass, TypeScript oracle diagnostics=[]
date: 2026-05-07
```

Remaining risks:

- none
