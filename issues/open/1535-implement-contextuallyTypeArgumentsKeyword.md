---
id: 1535
title: "Implement Contextuallytypeargumentskeyword"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5386]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1535.

## Summary

Triage contextuallyTypeArgumentsKeyword across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextuallyTypeArgumentsKeyword` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextuallyTypeArgumentsKeyword has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypeArgumentsKeyword.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypeArgumentsKeyword.ts --detail
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
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypeArgumentsKeyword.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypeArgumentsKeyword.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5386-bind-dom-settimeout-global.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextuallyTypeArgumentsKeyword.ts`

## Duplicate detection

- No exact existing owner found.
- `issues/done/412-implement-arguments-object.md` explains that many
  arguments-object buckets were classifier artifacts. This reference currently
  fails earlier on DOM global name resolution, not arguments-object semantics.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypeArgumentsKeyword.ts

result:
Feature label: name-resolution
Diagnostic: UnresolvedName / resolver-symbol
error: [UnresolvedName] unresolved name: `setTimeout` at 189..199

source context:
// @lib: es2017, dom
// @Filename: foo.js
const x = {
    bar() {
        setTimeout(function() { arguments }, 0);
    }
}

compiler evidence:
tokens: ok
ast: ok; object method `bar` contains `Call(Ident setTimeout, FunctionExpr, Number 0)`
resolved: fails at `setTimeout`
TypeScript oracle: ok, diagnostics []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/open/5386-bind-dom-settimeout-global.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypeArgumentsKeyword.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedName:1, unsupported_features=name-resolution:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypeArgumentsKeyword.ts
result:
pass; reproduced UnresolvedName for DOM global `setTimeout`
date:
2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5386 binds or precisely
  diagnoses the DOM `setTimeout` global.
