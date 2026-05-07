---
id: 1516
title: "Implement Contextualtypeshouldbeliteral"
type: spike
area: reference/triage
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextualTypeShouldBeLiteral across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypeShouldBeLiteral` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypeShouldBeLiteral has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeShouldBeLiteral.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeShouldBeLiteral.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with an existing implementation-ready issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the owner issue

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
- [x] Owner issue contains an exact `mise run reference-triage -- ...` command
- [x] Owner issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Owner issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeShouldBeLiteral.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeShouldBeLiteral.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5218-support-nested-function-closures-capturing-this.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypeShouldBeLiteral.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Date: 2026-05-07

Command:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeShouldBeLiteral.ts
```

Result: superseded by
`issues/open/5218-support-nested-function-closures-capturing-this.md`.

Current diagnostic:

```text
UnsupportedTypeScriptSyntax: issue-5179: 'this' implicitly has type 'any' because it does not have a type annotation in nested function `method`
feature_label: parser-syntax
```

Source context:

```ts
foo({
    type: 'y',
    value: 'done',
    method() {
        this;
        this.type;
        this.value;
    }
});
```

Compiler evidence:

- tokens: ok
- ast: ok; the object literal contains property `method` as a `FunctionExpr`
  with `This`, `This.type`, and `This.value` expressions in its body
- visible symbols include functions `foo` and `foo2`, plus bindings `xy`,
  `xyz`, and `test`
- resolved/lowered: fails at the implicit-this/object-literal method boundary
- TypeScript oracle: ok, diagnostics `[]`

Duplicate review:

- `issues/open/5179-report-implicit-this-before-closure-runtime-guard.md` is
  related but not exact: it owns cases where TypeScript reports TS2683 before a
  runtime closure guard. This reference is accepted by TypeScript.
- `issues/open/5218-support-nested-function-closures-capturing-this.md` is the
  exact implementation owner: it already covers object literal/contextual
  receiver function values that read `this`; this bucket adds the method
  shorthand/contextual-object form.
- Broad parser buckets such as `issues/open/059-implement-parser-syntax-extensions.md`
  and `issues/open/1519-implement-contextualTyping-parser-syntax.md` are less
  specific than the existing 5218 owner.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/open/5218-support-nested-function-closures-capturing-this.md`

Validation result:

```text
command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeShouldBeLiteral.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, current failure is UnsupportedSyntax unknown-unsupported
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeShouldBeLiteral.ts
result: pass; reproduced issue-5179 implicit-this/object-literal method boundary and superseded by issue 5218
date: 2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5218 supports or precisely
  diagnoses object literal methods/functions that read `this`.
