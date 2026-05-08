---
id: 3602
title: "Implement Nonnullablereductionnonstrict"
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

Close the generated `nonNullableReductionNonStrict` bucket as stale. Fresh
focused coverage and smart triage show the only affected reference file is now
`build_pass`, so there is no current compiler blocker to split.

## Problem

Reference test results previously showed 1 case failing in directory
`nonNullableReductionNonStrict` with diagnostics: parser-syntax. Fresh evidence
on 2026-05-08 shows the file builds successfully and TypeScript reports no
oracle diagnostics.

Problem: the generated bucket no longer has a current compiler blocker; no
child implementation issue is needed.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonNullableReductionNonStrict.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nonNullableReductionNonStrict.ts --detail
```

## Desired final state

This generated bucket is closed as stale because the only affected reference
case currently reports `BuildPass` / `pass`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] No child issue created because fresh triage found no current compiler blocker
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is stale
- [x] This closed issue contains an exact `mise run reference-triage -- ...` command
- [x] This closed issue includes the reference path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and build-pass result

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nonNullableReductionNonStrict.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonNullableReductionNonStrict.ts
```

Not run:

- `cargo fmt --all --check`; no Rust code changed
- `cargo nextest run`; no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nonNullableReductionNonStrict.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Build pass: nonNullableReductionNonStrict

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/nonNullableReductionNonStrict.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonNullableReductionNonStrict.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 527,
  "lines": 18,
  "extension": ".ts",
  "first_code_line": "type Transform1<T> = ((value: string) => T) | (string extends T ? undefined : never);"
}
```

Failure location:

```json
{
  "code": "BuildPass",
  "message": "ts2wasm build succeeded",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "build-pass",
  "error_type": "pass"
}
```

Source context:

```ts
type Transform1<T> = ((value: string) => T) | (string extends T ? undefined : never);
type Transform2<T> = string extends T ? ((value: string) => T) | undefined : (value: string) => T;
```

Visible symbols before result:

```json
[
  {
    "kind": "binding",
    "name": "z",
    "line": 13,
    "column": 5,
    "initializer": "x!"
  },
  {
    "kind": "binding",
    "name": "z",
    "line": 17,
    "column": 5,
    "initializer": "x!"
  }
]
```

Parser/IR evidence:

```text
tokens: ok through conditional types, optional calls, and `x!` non-null assertions
ast: ok; type aliases are erased; `f1?.("hello")` and `f2?.("hello")` parse as OptionalCall; `let z = x!` lowers to Ident("x")
resolved: ok; functions `test`, `f1`, and `f2` resolve
TypeScript oracle: ok; diagnostics=[]; hints show both `z` bindings as `NonNullable<T>`
```

Focused coverage:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
reference/typescript/tests/cases/compiler/nonNullableReductionNonStrict.ts: build_pass
```

## Completion evidence

Commits:

- e3380c679

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonNullableReductionNonStrict.ts
result: pass; BuildPass / pass with no TypeScript oracle diagnostics
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonNullableReductionNonStrict.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no implementation changes
date: 2026-05-08
```

Remaining risks:

- none
