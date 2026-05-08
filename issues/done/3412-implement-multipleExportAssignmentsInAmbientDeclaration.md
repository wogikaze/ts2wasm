---
id: 3412
title: "Split multipleExportAssignmentsInAmbientDeclaration to ambient duplicate export issue"
type: maintenance
area: frontend/semantics
class: superseded
priority: P1
depends_on: [5434]
blocks: []
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Closed the generated `multipleExportAssignmentsInAmbientDeclaration` bucket by splitting the current false-build-pass diagnostic gap into focused child issue #5434.

## Problem

`reference/typescript/tests/cases/compiler/multipleExportAssignmentsInAmbientDeclaration.ts` now build-passes in ts2wasm, but TypeScript reports duplicate `export=` diagnostics inside the ambient module declaration.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleExportAssignmentsInAmbientDeclaration.ts --detail --no-dashboard-data
result: executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleExportAssignmentsInAmbientDeclaration.ts
result: BuildPass; TypeScript oracle reports TS2300 duplicate identifier 'export='
date: 2026-05-08
```

## Evidence

Source:

```ts
declare module "m1" {
    var a: number
    var b: number;
    export = a;
    export = b;
}
```

Compiler evidence:

```text
tokens: ok through declare module, both var declarations, and both export assignments
ast: []
resolved: []
typescript oracle: TS2300 duplicate identifier 'export=' at a and b
```

This is not a current parser blocker. The ambient module is erased before the duplicate export-assignment diagnostic can be reported.

## Child Issues

- #5434: report duplicate export assignments in ambient module declarations.

## Validation

Issue sync and health checks:

```text
python scripts/manager.py update-issue-index
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Focused reference checks:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleExportAssignmentsInAmbientDeclaration.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleExportAssignmentsInAmbientDeclaration.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- #5434 remains open for implementation.
