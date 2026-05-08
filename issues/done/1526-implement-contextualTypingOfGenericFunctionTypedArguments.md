---
id: 1526
title: "Implement Contextualtypingofgenericfunctiontypedarguments"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1526.

## Summary

Triage contextualTypingOfGenericFunctionTypedArguments across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypingOfGenericFunctionTypedArguments` with diagnostics: type-system. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypingOfGenericFunctionTypedArguments has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfGenericFunctionTypedArguments1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfGenericFunctionTypedArguments1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfGenericFunctionTypedArguments1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfGenericFunctionTypedArguments1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/done/5383-classify-number-parameter-tofixed-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypingOfGenericFunctionTypedArguments1.ts`

## Duplicate detection

- No exact owner found. `issues/done/5202-parse-member-call-explicit-type-arguments.md`
  owns the older explicit member-call type argument parser boundary, but this
  representative now parses and reaches `x.toFixed()` lowering.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfGenericFunctionTypedArguments1.ts

result:
UnsupportedSyntax: issue-211: unknown receiver class for method `toFixed` at 405..416
source context: var f = (x: number) => { return x.toFixed() };
tokens: ok
AST: ok; typed arrow body contains Call(Member(Ident("x"), "toFixed"))
resolved/lowered: issue-211 unknown receiver class for method `toFixed`
TypeScript oracle: TS2345 at callback arguments to `_.forEach<number>`
```

Representative source:

```ts
var f = (x: number) => { return x.toFixed() };
var r5 = _.forEach<number>(c2, f);
var r6 = _.forEach<number>(c2, (x) => { return x.toFixed() });
```

The generated bucket was split to
`issues/done/5383-classify-number-parameter-tofixed-calls.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- child issue: `issues/done/5383-classify-number-parameter-tofixed-calls.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfGenericFunctionTypedArguments1.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, UnsupportedSyntax/type-system
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfGenericFunctionTypedArguments1.ts
result:
pass; reproduced issue-211 at number-typed parameter `x.toFixed()`
date:
2026-05-07
```

Remaining risks:

- Implementation remains open in 5383. After this blocker advances, explicit
  generic member callback assignability or Combinators interface semantics may
  need separate triage.
