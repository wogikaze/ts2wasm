---
id: 1529
title: "Implement Contextualtypingoftooshortoverloads"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5195]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1529.

## Summary

Triage contextualTypingOfTooShortOverloads across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypingOfTooShortOverloads` with diagnostics: method-call. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypingOfTooShortOverloads has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfTooShortOverloads.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfTooShortOverloads.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfTooShortOverloads.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfTooShortOverloads.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/done/5195-support-callable-interface-typed-local-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypingOfTooShortOverloads.ts`

## Duplicate detection

- Superseded by `issues/done/5195-support-callable-interface-typed-local-calls.md`.
  The first current blocker is a call to a local `var use: Overload;` whose
  interface type has call signatures.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfTooShortOverloads.ts

result:
Feature label: function-resolution
Diagnostic: UnresolvedFunction / resolver-symbol
resolved/lowering failure:
error: [UnsupportedTypeScriptSyntax] issue-5195: callable interface-typed local `use` is not callable — the variable is never assigned at 96..117

source context:
var use: Overload;
use((req, res) => {});

interface Overload {
    (handler1: (req1: string) => void): void;
    (handler2: (req2: number, res2: number) => void): void;
}

compiler evidence:
tokens: ok
ast: ok; `Let use = Undefined`, `Call(Ident use, ArrowFn req,res)`, and later `app.use(...)`
visible symbols: local bindings `use` and `app`
TypeScript oracle: TS2454 for `use`, TS2454 for `app`, and TS2687 for duplicate `method` modifiers
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/done/5195-support-callable-interface-typed-local-calls.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfTooShortOverloads.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedFunction:1, unsupported_features=function-resolution:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfTooShortOverloads.ts
result:
pass; reproduced issue-5195 callable interface-typed local call at `use(...)`
date:
2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5195 implements or
  precisely diagnoses callable interface-typed local calls.
