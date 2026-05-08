---
id: 3586
title: "Implement Nodenextpackageselfnamewithoutdirdecldircompositenesteddirs"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5292]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nodeNextPackageSelfNameWithOutDirDeclDirCompositeNestedDirs across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory
`nodeNextPackageSelfNameWithOutDirDeclDirCompositeNestedDirs`.

Fresh triage shows this bucket is currently blocked before package self-name
or nested-directory resolution: the harness parses the virtual `tsconfig.json`
section as TypeScript source and fails at the first `compilerOptions` JSON
property colon. The later `package.json`, relative export
`./src/thing.js`, package self-name `@this/package`, `outDir`,
`declarationDir`, and `composite` behavior are not actionable until issue
5292 skips config metadata sections.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDirDeclDirCompositeNestedDirs.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDirDeclDirCompositeNestedDirs.ts --detail
```

## Desired final state

This generated bucket is superseded by issue 5292. After 5292 lands, rerun
the focused triage command and open a narrower issue only if the next failure
is no longer the virtual `tsconfig.json` body.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5292

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
- [x] Superseding issue 5292 contains exact reproduction commands and acceptance criteria
- [x] This issue records failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDirDeclDirCompositeNestedDirs.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDirDeclDirCompositeNestedDirs.ts
```

Not run:

- cargo fmt --all --check: metadata-only issue close
- cargo nextest run: metadata-only issue close

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDirDeclDirCompositeNestedDirs.ts`

## Duplicate detection

- Superseded by `issues/open/5292-skip-tsconfig-filename-sections-in-reference-harness.md`

## Smart triage

Generated on 2026-05-08.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDirDeclDirCompositeNestedDirs.ts --detail --no-dashboard-data
result: UnsupportedSyntax: module-resolution
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDirDeclDirCompositeNestedDirs.ts
headline: UnsupportedSyntax: expected Semicolon, got Some(Colon) at 21..22
source: tsconfig.json section starts with `{`; failure is on `"compilerOptions": {`
tokens: tsconfig.json object tokens, including `"compilerOptions"`, `"module"`, `"outDir"`, `"declarationDir"`, and `"composite"`
ast/resolved: fail at the JSON property colon before package.json, relative export, or import resolution
visible symbols: []
typescript oracle: TS1005/TS2695 for JSON bodies plus TS2307 for `./src/thing.js` and `@this/package`
typescript ast top-level: Block, Block, ExportDeclaration, ImportDeclaration, ExpressionStatement, FunctionDeclaration
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `4acb25d8`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDirDeclDirCompositeNestedDirs.ts --detail --no-dashboard-data
result: UnsupportedSyntax: module-resolution; first blocker is virtual tsconfig.json parsing, owned by issue 5292
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDirDeclDirCompositeNestedDirs.ts
result: fail at JSON property colon in virtual tsconfig.json before package self-name/nested-dir/composite behavior
date: 2026-05-08
```

Remaining risks:

- After issue 5292 skips config metadata sections, this reference may next hit
  issue 5402 for `package.json` section parsing.
- After both metadata skips, it may expose a narrower relative module
  resolution, package self-name, conditional `exports` `default`/`types`,
  `outDir`, `declarationDir`, or `composite` behavior issue.
