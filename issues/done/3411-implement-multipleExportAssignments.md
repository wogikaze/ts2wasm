---
id: 3411
title: "Close multipleExportAssignments to CommonJS export assignment parser owner"
type: maintenance
area: frontend/parser
class: superseded
priority: P1
depends_on: [5346]
blocks: []
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Closed the generated `multipleExportAssignments` bucket as covered by implementation-ready issue #5346.

## Problem

`reference/typescript/tests/cases/compiler/multipleExportAssignments.ts` currently stops at the first `export = server;` with the generic CommonJS export assignment parser boundary.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleExportAssignments.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=import-export:1
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleExportAssignments.ts
result: UnsupportedModule issue-055 unsupported static export at export = server;
date: 2026-05-08
```

## Evidence

Source:

```ts
declare const server: {
    (): connectExport;
    test1: connectModule;
    test2(): connectModule;
};
export = server;
export = connectExport;
```

Compiler evidence:

```text
tokens: ok through both export assignment statements
ast/resolved: fails at first export assignment with issue-055 unsupported static export
typescript oracle: parses both ExportAssignment nodes and reports TS2300 duplicate identifier 'export='
```

The current first blocker is parsing/representing `export = expr;`. The later duplicate export-assignment diagnostic is not actionable until #5346 advances past this parser boundary.

## Owner

- #5346: parse CommonJS export assignment statements.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleExportAssignments.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleExportAssignments.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- #5346 remains open for implementation.
