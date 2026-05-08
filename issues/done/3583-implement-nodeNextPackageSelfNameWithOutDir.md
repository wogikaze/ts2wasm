---
id: 3583
title: "Implement Nodenextpackageselfnamewithoutdir"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5402]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nodeNextPackageSelfNameWithOutDir across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory
`nodeNextPackageSelfNameWithOutDir`.

Fresh triage shows this bucket is currently blocked before package self-name
resolution: the harness parses the virtual `package.json` section as
TypeScript source and fails at the first JSON property colon. The package
self-name `@this/package`, package `exports`, and `outDir` behavior are not
actionable until issue 5402 skips package metadata sections.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDir.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDir.ts --detail
```

## Desired final state

This generated bucket is superseded by issue 5402. After 5402 lands, rerun
the focused triage command and open a narrower package self-name issue only
if the next failure is no longer the virtual `package.json` body.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5402

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
- [x] Superseding issue 5402 contains exact reproduction commands and acceptance criteria
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDir.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDir.ts
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

- `reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDir.ts`

## Duplicate detection

- Superseded by `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`

## Smart triage

Generated on 2026-05-08.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDir.ts --detail --no-dashboard-data
result: UnsupportedSyntax: module-resolution
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDir.ts
headline: UnsupportedSyntax: expected Semicolon, got Some(Colon) at 101..102
source: package.json section starts with `{`; failure is on `"name": "@this/package"`
tokens: package.json object tokens, including `{`, `"name"`, `:`, `"@this/package"`, `"type"`, and `"exports"`
ast/resolved: fail at the JSON property colon before import resolution
visible symbols: []
typescript oracle: TS1005/TS2695 for JSON body plus TS2307 for import `@this/package`
typescript ast top-level: Block, ImportDeclaration, ExpressionStatement, FunctionDeclaration
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `992cc26b`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDir.ts --detail --no-dashboard-data
result: UnsupportedSyntax: module-resolution; first blocker is virtual package.json parsing, owned by issue 5402
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextPackageSelfNameWithOutDir.ts
result: fail at JSON property colon in virtual package.json before package self-name/outDir behavior
date: 2026-05-08
```

Remaining risks:

- After issue 5402 skips package metadata sections, this reference may expose a
  narrower package self-name `@this/package`, package `exports`, or `outDir`
  behavior issue.
