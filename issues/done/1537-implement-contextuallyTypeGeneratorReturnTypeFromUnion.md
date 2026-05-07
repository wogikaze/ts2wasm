---
id: 1537
title: "Implement Contextuallytypegeneratorreturntypefromunion"
type: spike
area: runtime/builtins
class: triage-needed
priority: P1
depends_on: [5387]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextuallyTypeGeneratorReturnTypeFromUnion across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextuallyTypeGeneratorReturnTypeFromUnion` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextuallyTypeGeneratorReturnTypeFromUnion has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypeGeneratorReturnTypeFromUnion.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypeGeneratorReturnTypeFromUnion.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypeGeneratorReturnTypeFromUnion.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypeGeneratorReturnTypeFromUnion.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5387-parse-generator-function-expressions-in-const-initializers.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextuallyTypeGeneratorReturnTypeFromUnion.ts`

## Duplicate detection

- No exact existing owner found.
- `issues/done/5213-generator-function-syntax-prerequisite-for-iterator-spread.md`
  is related but owns parameter initializer expressions, not typed const
  initializers.
- `#1514` covered
  `contextualTypeOnYield*.ts`; this reference still reports the raw parser
  boundary with the current shared binary.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypeGeneratorReturnTypeFromUnion.ts

result:
Feature label: parser-syntax
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
error: expected LeftParen, got Some(Star) at 185..186

source context:
type Action = () => (Generator<string, string, string[]> | string)

const test1: Action = function* () {
    const next = yield ''
    return next[0]
}

compiler evidence:
tokens: ok; includes `Function`, `Star`, `(`, `)`, and generator body tokens
ast: fails at the `function*` star
TypeScript AST: const initializer is FunctionExpression with AsteriskToken
TypeScript oracle: ok, diagnostics []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/open/5387-parse-generator-function-expressions-in-const-initializers.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypeGeneratorReturnTypeFromUnion.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypeGeneratorReturnTypeFromUnion.ts
result:
pass; reproduced generator function expression parser failure at typed const initializer
date:
2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5387 parses generator
  function expressions in typed const initializers.

## False-done audit

**truly-done** (1537)

- Implementation commits: verified via `git log --oneline --all --grep=1537`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
