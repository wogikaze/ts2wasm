---
id: 3508
title: "Implement Nobundledemitfromnodemodules"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5324]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage noBundledEmitFromNodeModules across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed as superseded by
`issues/open/5324-support-dependency-export-class-declarations.md`. Fresh
triage shows the current first blocker is dependency-module `export class`
issue-5005 for the virtual `/node_modules/projB/index.ts` section.

## Problem

Reference test results show 1 cases fail in directory `noBundledEmitFromNodeModules` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: noBundledEmitFromNodeModules currently reaches the existing
dependency-module `export class` issue-5005 boundary before node_modules
bundling or no-bundled-emit behavior is actionable.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noBundledEmitFromNodeModules.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noBundledEmitFromNodeModules.ts --detail
```

## Desired final state

This generated bucket is closed. Implement the current blocker through
`issues/open/5324-support-dependency-export-class-declarations.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5324
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
- [x] Existing issue 5324 owns the current dependency-module `export class` issue-5005 boundary
- [x] This closure includes failing path, diagnostic code, source context,
  visible symbols, parser/resolved evidence, and TypeScript AST evidence
- [x] No child issue is needed from 3508 because the current blocker is already implementation-ready in issue 5324

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noBundledEmitFromNodeModules.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noBundledEmitFromNodeModules.ts
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

- [x] none; superseded by issue 5324

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noBundledEmitFromNodeModules.ts`

## Duplicate detection

- `issues/open/5324-support-dependency-export-class-declarations.md` owns the
  current dependency virtual-file `export class` issue-5005 boundary.
- `issues/open/232-resolve-local-relative-es-module-graph.md` documents that
  package resolution, `node_modules`, and bare specifiers are out of scope for
  the completed local-relative graph slice. The current triage dump also sees
  `projB` as a non-local specifier, but the headline/stack first blocker is the
  dependency export-class boundary owned by issue 5324.

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noBundledEmitFromNodeModules.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noBundledEmitFromNodeModules.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/noBundledEmitFromNodeModules.ts: UnsupportedModule: import-export
```

Current headline/stack diagnostic:

```text
UnsupportedModule: issue-5005: dependency module declaration export uses a form
outside the current static export slice at 7..17
```

Source context:

```ts
// @fileName: /node_modules/projB/index.ts
export class C {}

// @fileName: /a.ts
import { C } from "projB";
```

Compiler evidence:

```text
tokens: ok; export class C and import { C } from "projB"
ast: ok; ExportDecl(ClassDecl C) and ImportNamed C from "projB"
resolved dump: also reports issue-232 unsupported non-local module specifier `projB`
module build/stack: issue-5005 dependency-module export class boundary
visible symbols: []
```

TypeScript oracle:

```text
TS2395: Individual declarations in merged declaration 'C' must be all exported or all local.
TS2307: Cannot find module 'projB' or its corresponding type declarations.
AST topLevel includes ClassDeclaration `export class C {}` and
ImportDeclaration `import { C } from "projB";`.
```

## Completion evidence

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noBundledEmitFromNodeModules.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedModule/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noBundledEmitFromNodeModules.ts
result: pass; current dependency export-class blocker is superseded by issue 5324
date: 2026-05-08
```

Remaining risks:

- After issue 5324 lands, this reference may expose bare package resolution for
  `projB`, node_modules traversal, no-bundled-emit parity, or TypeScript
  declaration merge diagnostics.
