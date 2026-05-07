---
id: 1317
title: "Implement Collisionsuperandpropertynameasconstuctorparameter"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: [5334]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Closed as superseded by `issues/open/5334-parse-class-constructor-overload-signatures.md`.

## Problem

Reference test results previously showed 1 case failing in directory `collisionSuperAndPropertyNameAsConstuctorParameter` with type-assertion diagnostics. Fresh triage shows the current blocker is `DuplicateFunction: duplicate constructor definition` for class constructor overload signatures.

Problem: `collisionSuperAndPropertyNameAsConstuctorParameter.ts` is blocked by the same class constructor overload signature boundary already tracked by issue 5334.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionSuperAndPropertyNameAsConstuctorParameter.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionSuperAndPropertyNameAsConstuctorParameter.ts --detail
```

## Desired final state

This generated bucket is closed. Implementation should proceed through `issues/open/5334-parse-class-constructor-overload-signatures.md`.

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
- [x] Existing child issue contains exact `reference-triage` commands for the same constructor overload diagnostic family
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Existing child issue acceptance names the exact diagnostic/stdout change for constructor overload signatures

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionSuperAndPropertyNameAsConstuctorParameter.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionSuperAndPropertyNameAsConstuctorParameter.ts
```

Not run:

- `cargo fmt --all --check` (not run; issue metadata only)
- `cargo nextest run` (not run; issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5334-parse-class-constructor-overload-signatures.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionSuperAndPropertyNameAsConstuctorParameter.ts`

## Duplicate detection

- `issues/open/5334-parse-class-constructor-overload-signatures.md` owns the current `DuplicateFunction: duplicate constructor definition` blocker for bodyless class constructor overload signatures.
- `issues/open/5337-parse-rest-parameter-constructor-overload-signatures.md` is related but covers rest-parameter constructor overload signatures.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage duplicate function: collisionSuperAndPropertyNameAsConstuctorParameter

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/collisionSuperAndPropertyNameAsConstuctorParameter.ts`
```

Failure location:

```text
DuplicateFunction: duplicate constructor definition
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=DuplicateFunction:1
unsupported_features=duplicate-function:1
semantic_enabled=0
```

Source context:

```ts
class b3 extends a {
    constructor(_super: number); // no code gen - no error
    constructor(_super: string);// no code gen - no error
    constructor(_super: any) { // should be error
        super();
    }
}
```

Compiler evidence:

```text
tokens: ok; constructor overload signatures and parameter-property constructors are tokenized
ast: ok; class b3/b4 contain multiple Function name `constructor` members
resolved: fails during validation/resolution with DuplicateFunction duplicate constructor definition
```

TypeScript oracle:

```text
ok: true
diagnostics: []
parameter hints include constructor `_super` overload parameters and private parameter-property `_super`
```

## Completion evidence


Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionSuperAndPropertyNameAsConstuctorParameter.ts
result: DuplicateFunction duplicate constructor definition; superseded by issue 5334
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionSuperAndPropertyNameAsConstuctorParameter.ts --detail --no-dashboard-data
result: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=DuplicateFunction:1
date: 2026-05-07
```

Remaining risks:

- Later `_super` constructor parameter-property collision behavior remains unproven until issue 5334 advances past constructor overload signatures.
