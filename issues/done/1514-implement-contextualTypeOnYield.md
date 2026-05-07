---
id: 1514
title: "Implement Contextualtypeonyield"
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

Triage contextualTypeOnYield across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `contextualTypeOnYield` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypeOnYield has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeOnYield1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeOnYield1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Implement the narrow parser boundary for generator function expressions
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

- [x] Duplicate candidates below are confirmed; no child issue is needed after the narrow parser fix
- [x] Focused parser coverage covers `function*` expressions
- [x] This issue includes failing paths, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference paths and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeOnYield1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeOnYield1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current compiler build has no parser blocker on these paths

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypeOnYield1.ts`
- `reference/typescript/tests/cases/compiler/contextualTypeOnYield2.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 showed both affected paths hit the same parser
boundary for generator function expressions:

```text
contextualTypeOnYield1.ts: UnsupportedSyntax: expected LeftParen, got Some(Star) at 167..168
contextualTypeOnYield2.ts: UnsupportedSyntax: expected LeftParen, got Some(Star) at 146..147
```

Source shape:

```ts
const f: FuncOrGeneratorFunc = function*() {
  yield (num) => console.log(num);
}

const g: OrGen = function* () {
  return (num) => console.log(num);
}
```

The parser already supported `function*` declarations by erasing generator
bodies. This slice applies the same narrow erasure to `function*` expressions:
the `*` is consumed, params are parsed, and the balanced body is skipped into an
empty `FunctionExpr` body.

After the fix, both paths build-pass:

```text
contextualTypeOnYield1.ts: build_pass=1, unsupported=0, blocked=0
contextualTypeOnYield2.ts: build_pass=1, unsupported=0, blocked=0
```

TypeScript oracle diagnostics are empty for both paths.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: cargo test -p ts2wasm-frontend parses_generator_function_expression_as_erased_body
result: pass
date: 2026-05-07

command: cargo fmt --all --check
result: pass
date: 2026-05-07

command: cargo nextest run -p ts2wasm-frontend
result: pass; 173 passed
date: 2026-05-07

command: cargo build -p ts2wasm-cli
result: pass
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeOnYield1.ts --detail --no-dashboard-data
result: pass; build_pass=1, unsupported=0, blocked=0
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeOnYield2.ts --detail --no-dashboard-data
result: pass; build_pass=1, unsupported=0, blocked=0
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeOnYield1.ts
result: pass; BuildPass, TypeScript oracle diagnostics=[]
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeOnYield2.ts
result: pass; BuildPass, TypeScript oracle diagnostics=[]
date: 2026-05-07
```

Remaining risks:

- Generator expression runtime semantics remain erased in this parser slice, matching existing generator declaration handling. Semantic/runtime generator behavior is outside this issue cleanup.
