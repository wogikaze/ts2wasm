---
id: 3537
title: "Implement Noimplicitanyformethodparameters"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a stale generated bucket. Fresh focused coverage and triage show
`noImplicitAnyForMethodParameters.ts` now build-passes.

## Problem

Fresh triage shows the current compiler parses and resolves the non-ambient
class methods with untyped parameters:

```text
ClassDecl C -> method foo(a)
ClassDecl D -> method foo(a)
```

TypeScript oracle reports no diagnostics for the representative file.

Problem: the generated parser-syntax bucket is stale and no longer has a
compiler blocker to split.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyForMethodParameters.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyForMethodParameters.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=1 unsupported=0 blocked=0
triage: BuildPass / pass
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

This generated bucket is closed as superseded by current build-pass behavior.
No child issue is needed.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Confirm the representative now build-passes
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
- [x] The done issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] The done issue includes path, BuildPass diagnostic, source context, visible symbols, and parser/TypeScript AST evidence
- [x] No child issue is created because the representative has no current blocker

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyForMethodParameters.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyForMethodParameters.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only stale bucket closure.
- `cargo nextest run`; metadata-only stale bucket closure.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyForMethodParameters.ts`

## Duplicate detection

- `issues/open/5149-parse-trailing-comma-in-typed-class-method-parameters.md`
  is related to class method parameter parsing, but its scope is trailing
  commas after typed parameters in a different fixture.
- `issues/open/5234-track-array-typed-parameters-for-callback-methods.md` is
  related to class-method parameter metadata, but its scope is array-shaped
  typed parameters for callback methods.
- No child issue was created because this representative now build-passes.

## Smart triage

### Smart triage: Build pass: noImplicitAnyForMethodParameters

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/noImplicitAnyForMethodParameters.ts`

Source context:

```text
declare class A { private foo(a); }
declare class B { public foo(a); }
class C { private foo(a) { } }
class D { public foo(a) { } }
```

Compiler evidence:

```text
tokens: ok
ast: ok; ClassDecl C and ClassDecl D each contain method foo(a)
resolved: ok; class methods retain parameter a
```

TypeScript oracle:

```text
ok, diagnostics=[]
parameter hints: all four `a` parameters are `any`
```

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
