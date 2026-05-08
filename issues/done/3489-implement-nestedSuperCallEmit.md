---
id: 3489
title: "Implement Nestedsupercallemit"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nestedSuperCallEmit across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed after refreshed evidence showed this generated name-resolution bucket is
stale. The representative now build-passes in ts2wasm, and the TypeScript
oracle reports no diagnostics.

## Problem

Reference test results show 1 cases fail in directory `nestedSuperCallEmit` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: nestedSuperCallEmit has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedSuperCallEmit.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedSuperCallEmit.ts --detail
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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedSuperCallEmit.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedSuperCallEmit.ts
```

Not run:

- `cargo fmt --all --check` (metadata-only issue closure)
- `cargo nextest run` (metadata-only issue closure)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nestedSuperCallEmit.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

2026-05-08 fresh result:

```text
### Smart triage: Build pass: nestedSuperCallEmit

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/nestedSuperCallEmit.ts`
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

reference/typescript/tests/cases/compiler/nestedSuperCallEmit.ts: build_pass
```

Source context:

```ts
abstract class Foo {
    constructor(shouldThrow: boolean) {
        if (shouldThrow) {
            throw new Error('Please retry');
        } else {
            console.log('OK');
        }
    }
}

class Bar extends Foo {
    constructor() {
        try {
            super(true);
        } catch (e: unknown) {
            console.log('Error: ' + (e as Error).message);
            super(false);
        }
    }
}
```

Visible symbols before completion:

```json
[
  {
    "kind": "class",
    "name": "Foo",
    "line": 5,
    "column": 10
  },
  {
    "kind": "class",
    "name": "Bar",
    "line": 15,
    "column": 1
  }
]
```

Compiler evidence:

- tokens: ok; abstract class, typed constructor parameter, `try`/`catch`,
  nested `super(true)` and catch-block `super(false)`
- ast: ok; `Bar extends Foo`, constructor body, try/catch, console log, and
  both `super(...)` calls parsed
- resolved: ok; `Foo`/`Bar` classes resolved, constructor parameter resolved,
  catch binding resolved, both `super(...)` calls retained as calls

TypeScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "typescriptVersion": "6.0.3"
  }
}
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedSuperCallEmit.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=1 unsupported=0 semantic_enabled=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedSuperCallEmit.ts
result: pass; BuildPass / pass; TypeScript oracle ok with no diagnostics
date: 2026-05-08
```

Remaining risks:

- semantic execution is not enabled for this case; this closure removes the
  generated build blocker only, not broader semantic parity work.
