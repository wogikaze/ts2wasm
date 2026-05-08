---
id: 3498
title: "Implement Newexpressionwithtypeparameterconstrainedtooutertypeparameter"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage newExpressionWithTypeParameterConstrainedToOuterTypeParameter across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed after refreshed evidence showed this generated semantic bucket is
stale. The representative now build-passes in ts2wasm.

## Problem

Reference test results show 1 cases fail in directory `newExpressionWithTypeParameterConstrainedToOuterTypeParameter` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: newExpressionWithTypeParameterConstrainedToOuterTypeParameter no
longer has a current build failure in the representative coverage window.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] At least one exact `reference-triage` command is recorded below
- [x] This issue includes failing path, diagnostic code, source context,
  visible symbols, parser evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/newExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts`

## Duplicate detection

- Fresh triage found no current compiler blocker.
- `issues/open/5195-support-callable-interface-typed-local-calls.md` owns the
  analogous callable interface local call path, but this representative uses a
  construct signature through `new i("")` and currently build-passes.
- `issues/open/5245-parse-interface-construct-signatures.md` owns parser
  support for interface construct signatures. This representative already
  parses, erases the interface, and resolves the `new i("")` expression.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Build pass: newExpressionWithTypeParameterConstrainedToOuterTypeParameter

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/newExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts
```

Focused coverage:

```text
suite=tsc
executed=1
build_pass=1
mismatch=0
runtime_error=0
fail=0
unsupported=0
blocked=0
semantic_enabled=0

reference/typescript/tests/cases/compiler/newExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts: build_pass
```

Source context:

```ts
interface I<T> {
    new <U extends T>(u: U): U;
}
var i: I<string>;
var y = new i(""); // y should be string
```

Compiler evidence:

```text
tokens: ok through interface construct signature, `var i: I<string>`, and
`new i("")`
ast: ok; interface is erased, `i` is initialized as Undefined, `y` is New(i, "")
resolved: ok; `New { class_name: "i", args: [String("")] }`
visible symbols: local bindings `i` and `y`
```

TypeScript oracle evidence:

```text
TS2454: Variable 'i' is used before being assigned.
binding hint: i has type I<string>
binding hint: y has type ""
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=1 unsupported=0 semantic_enabled=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newExpressionWithTypeParameterConstrainedToOuterTypeParameter.ts
result: pass; BuildPass / pass; TypeScript oracle reports TS2454 for `i`
date: 2026-05-08
```

Remaining risks:

- semantic execution is not enabled for this case; this closure removes the
  generated build blocker only, not TS2454 definite-assignment diagnostic parity
  for construct-signature locals.
