---
id: 1548
title: "Implement Contextuallytypedsymbolnamedproperties"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5228]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextuallyTypedSymbolNamedProperties across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextuallyTypedSymbolNamedProperties` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextuallyTypedSymbolNamedProperties has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedSymbolNamedProperties.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedSymbolNamedProperties.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedSymbolNamedProperties.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedSymbolNamedProperties.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into: `issues/open/5228-parse-simple-computed-object-literal-keys.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextuallyTypedSymbolNamedProperties.ts`

## Duplicate detection

- Superseded by
  `issues/open/5228-parse-simple-computed-object-literal-keys.md`, which owns
  the same simple identifier computed object-literal key parser boundary.
- `issues/done/402-computed-symbol-iterator-prerequisite-for-spread.md` is
  related but only completed the narrow `[Symbol.iterator]` object-key shape.
- `issues/open/5209-parse-computed-object-literal-property-expressions.md`
  owns broader computed expression keys, not simple identifier keys.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedSymbolNamedProperties.ts

result:
Triage class: contextuallyTypedSymbolNamedProperties
Feature label: parser-syntax
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Current diagnostic: expected Dot, got Some(RightBracket) at 395..396

source context:
f(ab, {
    [A]: ap => { ap.description },
    [B]: bp => { bp.description },
})

visible symbols:
binding A = Symbol("A")
binding B = Symbol("B")
binding ab

compiler evidence:
tokens: ok; computed object key tokens are LeftBracket Ident("A") RightBracket Colon
ast/resolved: fails with expected Dot at the closing bracket of `[A]`
TypeScript AST: CallExpression -> ObjectLiteralExpression -> PropertyAssignment
     -> ComputedPropertyName -> Identifier A
TypeScript oracle: ok, diagnostics []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- folded into `issues/open/5228-parse-simple-computed-object-literal-keys.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedSymbolNamedProperties.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedSymbolNamedProperties.ts
result:
pass; reproduced simple computed object-literal key parser failure at `[A]`
date:
2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5228 parses simple
  identifier computed object-literal keys.
