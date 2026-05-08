---
id: 1403
title: "Implement Compositewithnodemodulessourcefile"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [432]
blocks: [5295]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1403.

## Summary

Triage compositeWithNodeModulesSourceFile across 1 failing reference test case
and split this generated bucket into implementation-ready child issue 5295.

## Problem

Reference test results originally showed 1 case failing in directory
`compositeWithNodeModulesSourceFile` with diagnostics: import-export. Fresh
smart triage confirms a concrete module-resolution/import-export boundary:
the main virtual section imports a bare package with
`import myModule = require("myModule")`, and the dependency lives at
`/foo/node_modules/myModule/index.ts` with `export class c { }`.

Problem: `compositeWithNodeModulesSourceFile.ts` reports `UnsupportedModule`
for the bare `myModule`/dependency static class export path; issue 5295 now
owns the actionable fix.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/compositeWithNodeModulesSourceFile.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/compositeWithNodeModulesSourceFile.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed because the actionable work has been split into
issue 5295. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

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
- [x] At least one child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/compositeWithNodeModulesSourceFile.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/compositeWithNodeModulesSourceFile.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5295-resolve-import-equals-require-to-virtual-node-modules-class-export.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/compositeWithNodeModulesSourceFile.ts`

## Duplicate detection

Fresh smart triage listed broad same-feature import/export candidates. Existing
issue 436 is a broad triage-needed module-resolution bucket, not an executable
work order for this specific virtual `node_modules`/`import = require` shape.
Issue 5292 is related to `tsconfig.json` section handling but does not own the
current `UnsupportedModule` import/export boundary.

Resolution:

```text
Split to issue 5295: resolve import-equals require to virtual node_modules class export.
```

## Smart triage

### Smart triage: Triage import export: compositeWithNodeModulesSourceFile

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/compositeWithNodeModulesSourceFile.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/compositeWithNodeModulesSourceFile.ts
```

Failure location:

```text
message: issue-5005: dependency module declaration export uses a form outside the current static export slice at 7..18
```

Source context:

```ts
// @filename: /foo/tsconfig.json
{
    "compilerOptions": { "composite": true, "target": "es2015", "module": "commonjs" }
}

// @filename: /foo/node_modules/myModule/index.ts
export class c { }

// @filename: /foo/test.ts
import myModule = require("myModule");
new myModule.c();
```

Compiler evidence:

```text
tokens: ok; includes JSON config tokens, export class c, import myModule = require("myModule"), new myModule.c()
ast: ok; ExportDecl class c, ImportDefault-like import for myModule source, New myModule.c()
resolved/module_graph: UnsupportedModule issue-232 unsupported non-local module specifier `myModule` at 299..309
```

TypeScript oracle evidence:

```text
TS1005 diagnostics in the config section and TS2307 Cannot find module 'myModule'.
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
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/compositeWithNodeModulesSourceFile.ts
result: pass; reproduces UnsupportedModule for virtual node_modules import-equals require path
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/compositeWithNodeModulesSourceFile.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, blocked=0
date: 2026-05-07
```

Remaining risks:

- The reference harness still exposes the `tsconfig.json` section in token/AST
  dumps. If config-section handling becomes the first failure after issue 5295,
  it should be routed to issue 5292 or a narrower follow-up.
