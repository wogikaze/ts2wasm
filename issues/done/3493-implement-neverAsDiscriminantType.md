---
id: 3493
title: "Implement Neverasdiscriminanttype"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [432]
blocks: [5277]
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage neverAsDiscriminantType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed as superseded by
`issues/open/5277-parse-export-enum-declarations-to-enum-boundary.md`. Fresh
triage shows the first current blocker is the shared `export enum` issue-055
static export boundary, before the later `never` discriminant behavior.

## Problem

Reference test results show 1 cases fail in directory `neverAsDiscriminantType` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: neverAsDiscriminantType has 1 current reference failure, but the first
blocker is already represented by issue 5277 rather than needing a new
generated-bucket implementation issue.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/neverAsDiscriminantType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/neverAsDiscriminantType.ts --detail
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5277-parse-export-enum-declarations-to-enum-boundary.md`.

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
  `export enum` issue-055 boundary
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/neverAsDiscriminantType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/neverAsDiscriminantType.ts
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

- [x] superseded by: `issues/open/5277-parse-export-enum-declarations-to-enum-boundary.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/neverAsDiscriminantType.ts`

## Duplicate detection

- `issues/open/5277-parse-export-enum-declarations-to-enum-boundary.md` owns
  the current `export enum` issue-055 static export boundary.
- `issues/open/432-implement-import-export.md` is the broad import/export
  generated bucket and is not the narrow implementation owner.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage import export: neverAsDiscriminantType

- Issue class: triage-needed
- Feature label: import-export
- Diagnostic: UnsupportedModule / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/neverAsDiscriminantType.ts
```

Current compiler diagnostic:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 976..982
```

Focused coverage:

```text
suite=tsc
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0

reference/typescript/tests/cases/compiler/neverAsDiscriminantType.ts: UnsupportedSyntax: unknown-unsupported
```

Source context:

```ts
export enum GatewayOpcode {
    DISPATCH = 0,
    HEARTBEAT = 1,
    IDENTIFY = 2,
}
```

Compiler evidence:

```text
tokens: ok through the earlier `never` discriminant type aliases and functions
ast/resolved: fail at `export enum GatewayOpcode` with issue-055 static export
visible symbols before failure: functions `f1(foo: Foo1)` and `f2(foo: Foo2)`
TypeScript oracle: ok, diagnostics=[]; AST includes EnumDeclaration with ExportKeyword
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/neverAsDiscriminantType.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/neverAsDiscriminantType.ts
result: pass; issue-055 static export at export enum GatewayOpcode; superseded by issue 5277
date: 2026-05-08
```

Remaining risks:

- After issue 5277 advances, this reference may expose a later mapped type,
  enum, async function, or `never` discriminant semantic blocker.
