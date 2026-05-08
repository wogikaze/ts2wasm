---
id: 1384
title: "Implement Commonjsimportclassexpression"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: [232]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1384.

## Summary

Closed as superseded by
`issues/open/232-resolve-local-relative-es-module-graph.md`.

Fresh focused coverage shows the current blocker is an oracle-matching missing
local module diagnostic for `./mod1`, not a standalone CommonJS class
expression implementation slice.

## Problem

Reference test results originally showed 1 case failing in directory
`commonJsImportClassExpression` with diagnostics: import-export. Fresh focused
triage on 2026-05-07 shows parsing reaches the import-equals form and module
graph validation reports the existing issue-232 missing local module diagnostic
for `./mod1`.

Problem: `commonJsImportClassExpression.ts` currently reports
`UnsupportedModule` for missing local module `./mod1`, matching TypeScript
TS2307 for the same specifier.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonJsImportClassExpression.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonJsImportClassExpression.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through module
graph/import-export follow-up work only if a later triage after issue-232
behavior changes exposes a new blocker.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing missing local module diagnostic behavior
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing issue 232 contains the exact missing local relative module diagnostic behavior
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, parser/TypeScript AST evidence, and TypeScript oracle evidence
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonJsImportClassExpression.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonJsImportClassExpression.ts
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

- [x] superseded by: `issues/open/232-resolve-local-relative-es-module-graph.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commonJsImportClassExpression.ts`

## Duplicate detection

- `issues/open/232-resolve-local-relative-es-module-graph.md` owns the exact
  current behavior: source-spanned missing local relative module diagnostics for
  static imports and import-equals module specifiers.
- Class expression lowering appears in one triage diagnostic path, but current
  focused coverage and resolved dump stop first at module graph validation for
  missing `./mod1`.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage import export: commonJsImportClassExpression

- Issue class: triage-needed
- Feature label: import-export
- Path: reference/typescript/tests/cases/compiler/commonJsImportClassExpression.ts
```

Source context:

```text
1 | // @target: es2015
2 | // @module: commonjs
3 | // @Filename: mod1.ts
4 | export = class {
5 |     chunk = 1
6 | }
7 |
8 | // @Filename: use.ts
9 | import Chunk = require('./mod1')
10 | declare var c: Chunk;
11 | c.chunk;
```

Compiler evidence:

```text
tokens: ok; Export Equal Class and Import Chunk = require("./mod1") are present
ast: ok; ClassExpr and ImportDefault/source "./mod1" are represented
resolved/module_graph: UnsupportedModule issue-232 missing local module `./mod1` at 140..148
```

TypeScript oracle:

```text
TS2307: Cannot find module './mod1' or its corresponding type declarations.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonJsImportClassExpression.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedModule:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonJsImportClassExpression.ts
result: issue-232 missing local module `./mod1`; TypeScript TS2307
date: 2026-05-07
```

Remaining risks:

- If virtual `// @Filename:` splitting later resolves `./mod1`, this path may
  reveal CommonJS `export = class` or class expression lowering blockers.
