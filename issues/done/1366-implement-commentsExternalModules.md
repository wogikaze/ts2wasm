---
id: 1366
title: "Implement Commentsexternalmodules"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: [5285]
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed after splitting the current exported variable initializer blocker into
`issues/open/5285-support-export-var-initializer-declarations.md`.

## Problem

Reference test results show 3 cases failing in directory
`commentsExternalModules` with import/export diagnostics. Fresh triage shows the
bucket contains two current failure families: one bare/non-local module
specifier diagnostic already covered by issue 232, and two `export var`
initializer declarations that stop at issue-055.

Problem: `commentsExternalModules2.ts` and `commentsExternalModules3.ts`
currently report `issue-055: unsupported variable export` at
`export var newVar = ...`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsExternalModules.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsExternalModules2.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsExternalModules3.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsExternalModules --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
commentsExternalModules.ts: UnsupportedModule import-export; resolved dump reaches issue-232 unsupported non-local module specifier `commentsExternalModules_0`
commentsExternalModules2.ts: UnsupportedSyntax import-export; issue-055 unsupported variable export at `export var newVar = ...`
commentsExternalModules3.ts: UnsupportedSyntax import-export; issue-055 unsupported variable export at `export var newVar = ...`
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5285-support-export-var-initializer-declarations.md`; the bare
module specifier behavior remains covered by `issues/done/232-resolve-local-relative-es-module-graph.md`.

## Scope

In scope:

- [x] Inspect the smart triage reports below
- [x] Confirm whether existing open/done issues already cover this bucket
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

- [x] Duplicate candidates below are confirmed as covered or split
- [x] Child issue contains exact `reference-triage` commands
- [x] Child issue includes failing paths, diagnostic code, source context, token/AST evidence, and TypeScript oracle evidence
- [x] Child issue acceptance names the exact reference paths and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsExternalModules --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsExternalModules.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsExternalModules2.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsExternalModules3.ts
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

- [x] `issues/open/5285-support-export-var-initializer-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsExternalModules.ts`
- `reference/typescript/tests/cases/compiler/commentsExternalModules2.ts`
- `reference/typescript/tests/cases/compiler/commentsExternalModules3.ts`

## Duplicate detection

- `issues/done/232-resolve-local-relative-es-module-graph.md` covers the
  intentional bare/non-local module specifier diagnostic seen in
  `commentsExternalModules.ts`.
- `issues/open/5283-support-entry-export-var-declarations.md` covers a simple
  typed `export var b: number;` declaration and is too narrow for initialized
  exported variables that depend on imported module values.
- `issues/open/5285-support-export-var-initializer-declarations.md` owns the
  current `export var newVar = new extMod...` blocker.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: commentsExternalModules window

coverage:
executed=3
unsupported=3
unsupported_diagcodes=UnsupportedSyntax:2,UnsupportedModule:1
unsupported_features=import-export:3
```

Representative failures:

```text
commentsExternalModules.ts: issue-232 unsupported non-local module specifier `commentsExternalModules_0`
commentsExternalModules2.ts: issue-055 unsupported variable export at 1313..1319
commentsExternalModules3.ts: issue-055 unsupported variable export at 1320..1326
```

Source context for child issue:

```ts
import extMod = require("commentsExternalModules2_0");
extMod.m1.fooExport();
export var newVar = new extMod.m1.m2.c();
extMod.m4.fooExport();
export var newVar2 = new extMod.m4.m2.c();
```

Compiler evidence:

```text
tokens: ok through import-equals and exported variable initializers
commentsExternalModules2.ts ast/resolved: issue-055 unsupported variable export
commentsExternalModules3.ts ast/resolved: issue-055 unsupported variable export
```

TypeScript oracle:

```text
commentsExternalModules2.ts: TS2307 for missing commentsExternalModules2_0, but exported variables are represented as FirstStatement nodes
commentsExternalModules3.ts: TS2307 for missing ./commentsExternalModules2_0, but exported variables are represented as FirstStatement nodes
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsExternalModules --detail --no-dashboard-data
result: executed=3, unsupported=3, unsupported_diagcodes=UnsupportedSyntax:2,UnsupportedModule:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsExternalModules.ts
result: issue-232 unsupported non-local module specifier, covered by done issue 232
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsExternalModules2.ts
result: issue-055 unsupported variable export; split to issue 5285
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsExternalModules3.ts
result: issue-055 unsupported variable export; split to issue 5285
date: 2026-05-07
```

Remaining risks:

- Package/bare specifier support remains out of scope per issue 232.
- Full external module execution and comment/declaration emit fidelity remain
  out of scope for the child issue.
