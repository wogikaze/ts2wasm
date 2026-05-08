---
id: 3491
title: "Implement Nestedtypevariableinfersliteral"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: [5372]
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nestedTypeVariableInfersLiteral across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed as superseded by
`issues/open/5372-parse-ambient-function-asi-with-constructor-types.md`. Fresh
triage shows this generated parser-syntax bucket reaches the same issue-400
ambient-function ASI boundary for generic `declare function` signatures.

## Problem

Reference test results show 1 cases fail in directory `nestedTypeVariableInfersLiteral` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: nestedTypeVariableInfersLiteral has 1 current parser failure, but the
blocker is already represented by issue 5372 rather than needing a new
generated-bucket implementation issue.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedTypeVariableInfersLiteral.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedTypeVariableInfersLiteral.ts --detail
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5372-parse-ambient-function-asi-with-constructor-types.md`.

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
- [x] Superseding issue contains an exact reference-triage command for the
  issue-400 ambient-function ASI family
- [x] This issue includes failing path, diagnostic code, source context,
  visible symbols, parser evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedTypeVariableInfersLiteral.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedTypeVariableInfersLiteral.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/open/5372-parse-ambient-function-asi-with-constructor-types.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nestedTypeVariableInfersLiteral.ts`

## Duplicate detection

- `issues/open/5372-parse-ambient-function-asi-with-constructor-types.md`
  owns the current issue-400 ambient-function ASI boundary for generic
  declaration-only function signatures.
- `issues/open/705-implement-asiAmbientFunctionDeclaration.md` is the older
  generated triage bucket for the minimal `declare function foo()` ASI case,
  but it is not implementation-ready.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage parser syntax: nestedTypeVariableInfersLiteral

- Issue class: triage-needed
- Feature label: parser-syntax
- Diagnostic: UnsupportedTypeScriptSyntax / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/nestedTypeVariableInfersLiteral.ts
```

Current compiler diagnostic:

```text
UnsupportedTypeScriptSyntax: issue-400: unterminated ambient function declaration at 77..84
```

Focused coverage:

```text
suite=tsc
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedTypeScriptSyntax:1
unsupported_features=parser-syntax:1
semantic_enabled=0

reference/typescript/tests/cases/compiler/nestedTypeVariableInfersLiteral.ts: UnsupportedTypeScriptSyntax: parser-syntax
```

Source context:

```ts
declare function direct<A extends string>(a: A | A[]): Record<A, string>
declare function nested<A extends string>(a: { fields: A }): Record<A, string>
declare function nestedUnion<A extends string>(a: { fields: A | A[] }): Record<A, string>

const directUnionSingle = direct("z")
const directUnionArray = direct(["z", "y"])
```

Compiler evidence:

```text
tokens: ok through `declare function direct<A extends string>(a: A | A[]): Record<A, string>`
ast/resolved: fail before AST construction with issue-400 unterminated ambient function declaration
visible symbols before failure: []
TypeScript oracle: ok, diagnostics=[]; top-level AST has the three FunctionDeclaration nodes followed by const bindings and hasZField calls
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedTypeVariableInfersLiteral.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedTypeScriptSyntax:1 unsupported_features=parser-syntax:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedTypeVariableInfersLiteral.ts
result: pass; issue-400 unterminated ambient function declaration; superseded by issue 5372
date: 2026-05-08
```

Remaining risks:

- Issue 5372 may expose a narrower generic-return or union-parameter parsing
  gap after the constructor-type representative advances.
