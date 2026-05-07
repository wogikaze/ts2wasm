---
id: 3347
title: "Implement Modulemembermissingerrorisrelative"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as superseded by `issues/open/5229-resolve-imports-between-filename-sections.md`.
Fresh triage for `moduleMemberMissingErrorIsRelative.ts` reaches the existing
virtual `@Filename` sibling import resolution blocker before missing exported
member diagnostics become actionable.

## Problem

Reference test results show 1 case failing in directory
`moduleMemberMissingErrorIsRelative` with diagnostics: import-export. Fresh
coverage reports `UnsupportedModule`, and the resolved dump shows the current
blocker is virtual local module lookup:

```text
issue-232: missing local module `./foo`
```

Problem: this generated bucket is not directly implementable until local
specifier resolution can target sibling `// @filename:` sections. That work is
already tracked by issue 5229.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleMemberMissingErrorIsRelative.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleMemberMissingErrorIsRelative.ts --detail
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
- [x] Existing owner `issues/open/5229-resolve-imports-between-filename-sections.md` contains the implementation-ready acceptance criteria
- [x] This closed bucket preserves the exact reference path, diagnostic, source context, visible import, parser AST, and TypeScript oracle evidence
- [x] No new child issue was needed because the blocker matches an existing open owner

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleMemberMissingErrorIsRelative.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleMemberMissingErrorIsRelative.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5229-resolve-imports-between-filename-sections.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleMemberMissingErrorIsRelative.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleMemberMissingErrorIsRelative.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleMemberMissingErrorIsRelative.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/moduleMemberMissingErrorIsRelative.ts: UnsupportedModule: import-export
```

Source context:

```text
// @module: commonjs
// @target: es2015
// @filename: folder/foo.ts
export {};
// @filename: folder/bar.ts
import {nosuch} from './foo';
```

Compiler evidence:

```text
tokens: ok for `export {};` and `import {nosuch} from "./foo";`
ast: ok; ExportNamed with no specifiers, ImportNamed nosuch from "./foo"
visible symbols: import "./foo"
resolved: issue-232 missing local module `./foo`; tried on-disk candidates under reference/typescript/tests/cases/compiler
```

TypeScript oracle:

```text
TS2307: Cannot find module './foo' or its corresponding type declarations.
```

Superseded by:

- `issues/open/5229-resolve-imports-between-filename-sections.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleMemberMissingErrorIsRelative.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedModule/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleMemberMissingErrorIsRelative.ts
result: pass; reproduced issue-232 missing virtual local module boundary owned by issue 5229
date: 2026-05-08
```

Remaining risks:

- After issue 5229 resolves sibling `@Filename` imports, this reference may
  advance to the intended missing exported member diagnostic for `nosuch`.
