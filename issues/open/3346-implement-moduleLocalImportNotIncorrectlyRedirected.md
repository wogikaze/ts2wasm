---
id: 3346
title: "Implement Modulelocalimportnotincorrectlyredirected"
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

Closed as superseded by `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`.
Fresh triage for `moduleLocalImportNotIncorrectlyRedirected.ts` stops in a
virtual `node_modules/troublesome-lib/package.json` section before import
redirection or module resolution semantics become reachable.

## Problem

Reference test results show 1 case failing in directory
`moduleLocalImportNotIncorrectlyRedirected` with diagnostics: import-export.
Fresh triage confirms this generated bucket is currently blocked by
reference-harness handling of virtual `package.json` sections:

```text
UnsupportedSyntax: expected Semicolon, got Some(Colon) at 139..140
```

Problem: the compiler tokenizes and parses the JSON body of
`// @filename: node_modules/troublesome-lib/package.json` as TypeScript source.
The current blocker is already owned by issue 5402.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleLocalImportNotIncorrectlyRedirected.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleLocalImportNotIncorrectlyRedirected.ts --detail
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
- [x] Existing owner `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md` contains the implementation-ready acceptance criteria
- [x] This closed bucket preserves the exact reference path, diagnostic, source context, token evidence, and TypeScript AST/oracle evidence
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleLocalImportNotIncorrectlyRedirected.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleLocalImportNotIncorrectlyRedirected.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleLocalImportNotIncorrectlyRedirected.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleLocalImportNotIncorrectlyRedirected.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleLocalImportNotIncorrectlyRedirected.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/moduleLocalImportNotIncorrectlyRedirected.ts: UnsupportedSyntax: import-export
```

Current diagnostic:

```text
UnsupportedSyntax: expected Semicolon, got Some(Colon) at 139..140
```

Source context:

```text
// @module: commonjs
// @target: es2015
// @noImplicitReferences: true
// @filename: node_modules/troublesome-lib/package.json
{
  "name": "troublesome-lib",
  "typings": "lib/index.d.ts",
  "version": "0.0.1"
}
```

Compiler evidence:

```text
tokens: ok; JSON object body is tokenized as LeftBrace, String("name"), Colon, String("troublesome-lib"), ...
ast/resolved: fail at the first JSON property colon
visible symbols: []
```

TypeScript oracle:

```text
TS1005/TS2695 diagnostics are reported for the package.json body.
Later diagnostics include TS2307 for `./utilities/positioning`, `./positioning/index`, and `troublesome-lib`, but those are hidden until package.json sections stop being parsed as executable source.
```

Superseded by:

- `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleLocalImportNotIncorrectlyRedirected.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleLocalImportNotIncorrectlyRedirected.ts
result: pass; reproduced package.json colon parse boundary owned by issue 5402
date: 2026-05-08
```

Remaining risks:

- After issue 5402 skips package metadata sections, this reference may advance
  to local virtual import resolution, export-star handling, exported enum
  declarations, or package resolution for `troublesome-lib`.
