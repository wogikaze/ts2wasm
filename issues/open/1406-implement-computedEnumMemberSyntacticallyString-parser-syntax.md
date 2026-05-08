---
id: 1406
title: "Implement Computedenummembersyntacticallystring Parser Syntax"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: [5229]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1406.

## Summary

Closed as superseded by
`issues/open/5229a-resolve-imports-between-filename-sections.md`.

Fresh triage shows the original parser-syntax label is stale:
`computedEnumMemberSyntacticallyString2.ts` now parses and builds far enough to
hit module graph validation for a virtual `@filename` import.

## Problem

Reference test results originally showed 1 case failing in directory
`computedEnumMemberSyntacticallyString-parser-syntax` with diagnostics:
parser-syntax. Fresh focused coverage now reports `UnsupportedModule` for
`import { BAR } from './bar';` in a virtual `./foo.ts` section even though the
same reference file later declares `// @filename: ./bar.ts`.

Problem: 1406 is not a standalone parser implementation order. The current
first blocker is the virtual section module-resolution behavior already owned
by issue 5229.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString2.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString2.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implementation should proceed through issue
5229, which owns local imports between TypeScript reference `@filename`
sections.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5229
- [x] Preserve exact reproduction commands and representative evidence

Out of scope:

- Direct implementation from this generated bucket
- Full enum transform/runtime support

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
- [x] Existing issue 5229 owns the current virtual `@filename` local import blocker
- [x] This issue includes failing path, diagnostic code, source context, compiler evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString2.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString2.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only
- `cargo nextest run`; issue metadata only

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/open/5229a-resolve-imports-between-filename-sections.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString2.ts`

## Duplicate detection

- `issues/open/5229a-resolve-imports-between-filename-sections.md` owns the
  current first blocker: local imports between TypeScript reference
  `// @Filename:` / `// @filename:` virtual sections.
- `issues/open/5296-parse-double-dot-numeric-literal-property-access.md` covers
  the sibling `computedEnumMemberSyntacticallyString.ts` parser blocker and is
  not needed for this `2.ts` file.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Build pass: computedEnumMemberSyntacticallyString2

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString2.ts
```

Source context:

```ts
// @filename: ./foo.ts
import { BAR } from './bar';
const LOCAL = 'LOCAL';

enum Foo {
  A = `${BAR}`,
  B = LOCAL,
}

// @filename: ./bar.ts
export const BAR = 'bar';
```

Compiler evidence:

```text
tokens: ok; includes Import, Ident("BAR"), String("./bar"), enum tokens, and export const BAR
ast: ok; ImportNamed("./bar"), Let LOCAL, and ExportDecl BAR are represented
resolved/module_graph: UnsupportedModule issue-232 missing local module `./bar`
```

TypeScript oracle evidence:

```text
TS2307: Cannot find module './bar' or its corresponding type declarations.
TS18033: Type 'string' is not assignable to type 'number' as required for computed enum member values.
TS2448/TS2454: BAR used before declaration/assignment in the raw single-file oracle view.
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
semantic_enabled=0
```

## Completion evidence

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString2.ts
result: pass; build step succeeds, resolved dump reports issue-232 missing local module `./bar`
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString2.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, blocked=0
date: 2026-05-07
```

Remaining risks:

- After issue 5229 resolves virtual section imports, this reference file is
  expected to expose enum computed-member diagnostics.
