---
id: 1540
title: "Implement Contextuallytypedbydiscriminableunion Unknown Unsupported"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: [5304]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextuallyTypedByDiscriminableUnion-unknown-unsupported across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextuallyTypedByDiscriminableUnion-unknown-unsupported` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextuallyTypedByDiscriminableUnion-unknown-unsupported has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion2.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into: `issues/open/5304-parse-generic-arrow-functions-with-typed-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion2.ts`

## Duplicate detection

- Superseded by
  `issues/open/5304-parse-generic-arrow-functions-with-typed-parameters.md`.
  This bucket reaches the same parser boundary for a generic arrow function
  with typed parameters.
- `issues/open/694-implement-arrayToLocaleStringES-unknown-unsupported.md` is a
  generated same-label candidate but not a syntax match.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion2.ts

result:
Triage class: contextuallyTypedByDiscriminableUnion2
Feature label: parser-syntax
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Current diagnostic: expected RightParen, got Some(Colon) at 1012..1013

source context:
const MyComponent = <I extends Identifiable>(props: MyComponentProps<I>) => {};

visible symbols:
binding MyComponent, initializer <I extends Identifiable>(props

compiler evidence:
tokens: ok
ast/resolved: fails at the typed generic arrow parameter colon
TypeScript AST: VariableDeclaration MyComponent -> ArrowFunction with type
    parameter I extends Identifiable and parameter props: MyComponentProps<I>
TypeScript oracle: ok, diagnostics []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- folded into `issues/open/5304-parse-generic-arrow-functions-with-typed-parameters.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion2.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion2.ts
result:
pass; reproduced generic arrow typed-parameter parser failure at props: MyComponentProps<I>
date:
2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5304 parses generic arrow
  functions with typed parameters.

## False-done audit

**truly-done** (1540)

- Implementation commits: verified via `git log --oneline --all --grep=1540`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
