---
id: 1220
title: "Implement Classinconvertedloopes"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: [5264]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed by splitting the current typed const declaration parser blocker to
`issues/open/5264-parse-typed-const-declarations-before-initializers.md`.

## Problem

Reference test results show 1 case failing in directory
`classInConvertedLoopES` with diagnostics: unknown-unsupported. Fresh triage
shows the current blocker occurs before loop/class semantics, at the leading
typed const declaration.

Problem: `const classesByRow: Record<string, object> = {};` reports
`const declarations require an initializer` because the parser does not consume
the TypeScript type annotation before checking for `=`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classInConvertedLoopES5.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classInConvertedLoopES5.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing open/done issues do not cover the exact current boundary
- [x] Split one observable behavior into an implementation-ready child issue
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

- [x] Duplicate candidates below are confirmed as no-match for the exact current boundary
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classInConvertedLoopES5.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classInConvertedLoopES5.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5264-parse-typed-const-declarations-before-initializers.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classInConvertedLoopES5.ts`

Source context:

```ts
const classesByRow: Record<string, object> = {};
for (const row of ['1', '2', '3', '4', '5']) {
  class RowClass {
    row = row;
    static factory = () => new RowClass();
  }
  classesByRow[row] = RowClass;
}
```

## Duplicate detection

- `issues/done/5184-parse-const-enum-declarations.md` is related only by the
  generic missing-initializer diagnostic; it covers `const enum`, not typed
  variable declarations with initializers.
- `issues/done/1444-implement-constDeclarations-unknown-unsupported.md` is a
  generated bucket for a different reference and is not an exact owner for
  `classInConvertedLoopES5.ts`.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classInConvertedLoopES5.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classInConvertedLoopES5.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: const declarations require an initializer at 31..43
Source: const classesByRow: Record<string, object> = {};
tokens: ok; colon, Record<string, object>, equals, object literal
AST/resolved: fail at classesByRow
TypeScript oracle: ok, diagnostics=[]
TypeScript AST path: VariableDeclarationList -> VariableDeclaration ->
  Identifier classesByRow
```

Split issue:

- `issues/open/5264-parse-typed-const-declarations-before-initializers.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5264-parse-typed-const-declarations-before-initializers.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classInConvertedLoopES5.ts
result: pass; current blocker split to issue 5264
date: 2026-05-06
```

Remaining risks:

- This bucket name mentions converted loops/classes, but the current first
  blocker is the typed const declaration before the loop.
