---
id: 3531
title: "Implement Noerrorsincallback"
type: spike
area: frontend/semantics
class: superseded
priority: P1
depends_on: [5471]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence shows the old arrow-function
blocker is stale; the remaining actionable gap is a false build-pass semantic
diagnostic, split to issue 5471.

## Problem

Older reference results reported an `arrow-function` blocker for
`noErrorsInCallback.ts`. Fresh triage on 2026-05-08 now parses and resolves the
class, top-level `new Bar({})`, zero-argument arrow callback, and nested
`new Bar({})`; ts2wasm reports `BuildPass`.

TypeScript still reports TS2345 for both `new Bar({})` calls because `{}` is not
assignable to the constructor parameter `foo: string`. That false build-pass
semantic gap is now tracked by
`issues/open/5471-report-constructor-argument-type-diagnostics.md`.

Problem: the generated arrow-function bucket is stale; the remaining
constructor argument type diagnostic gap has been split to issue 5471.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noErrorsInCallback.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noErrorsInCallback.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1, build_pass=1, unsupported=0, blocked=0, semantic_enabled=0
triage: BuildPass / pass
```

Source context:

```ts
class Bar {
    constructor(public foo: string) { }
}
var one = new Bar({}); // Error
[].forEach(() => {
    var two = new Bar({}); // No error?
});
```

Compiler evidence:

```text
tokens: ok through class Bar, top-level new Bar({}), [].forEach, zero-argument arrow, and nested new Bar({})
ast: ok; constructor parameter property is represented, both New expressions are present
resolved: ok; nested arrow body contains Let two = New Bar({})
TypeScript oracle: TS2345 at line 5 character 19 and line 7 character 23
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5471-report-constructor-argument-type-diagnostics.md`. Do not
implement directly from this bucket.

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
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noErrorsInCallback.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noErrorsInCallback.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only issue split.
- `cargo nextest run`; metadata-only issue split.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5471-report-constructor-argument-type-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noErrorsInCallback.ts`

## Duplicate detection

- No exact existing owner found for constructor argument TS2345-style type
  diagnostics.
- `issues/open/5286-preserve-class-constructor-parameters-for-new-arity.md`
  is related but covers lowering-time constructor arity metadata, not this
  false build-pass type compatibility diagnostic.
- `issues/open/5188-report-block-scoped-function-call-arity-diagnostics.md`
  is related but covers function call arity, not class constructor argument
  type compatibility.

## Smart triage

### Smart triage: Build pass: noErrorsInCallback

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/noErrorsInCallback.ts`

TypeScript oracle diagnostics:

```text
TS2345: Argument of type '{}' is not assignable to parameter of type 'string'. at line 5, character 19
TS2345: Argument of type '{}' is not assignable to parameter of type 'string'. at line 7, character 23
```

Split to issue 5471 because the current actionable gap is semantic diagnostic
parity, not arrow-function parsing or lowering.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
