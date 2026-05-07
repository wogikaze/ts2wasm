---
id: 3413
title: "Split multipleExports to namespace export declaration diagnostic"
type: maintenance
area: frontend/semantics
class: superseded
priority: P1
depends_on: [5435]
blocks: []
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Closed the generated `multipleExports` bucket by splitting the current false-build-pass diagnostic gap into focused child issue #5435.

## Problem

`reference/typescript/tests/cases/compiler/multipleExports.ts` now build-passes in ts2wasm, but TypeScript reports diagnostics for an export declaration inside a namespace.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleExports.ts --detail --no-dashboard-data
result: executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleExports.ts
result: BuildPass; TypeScript oracle reports TS1194 and TS2484 for export {x} inside namespace M
date: 2026-05-08
```

## Evidence

Source:

```ts
export namespace M {
    export var v = 0;
    export let x;
}

const x = 0;
export namespace M {
    v;
    export {x};
}
```

Compiler evidence:

```text
tokens: ok through exported namespaces, exported var/let, const x, and export {x}
ast/resolved: only top-level const x remains
typescript oracle: TS1194 Export declarations are not permitted in a namespace; TS2484 export declaration conflicts with exported declaration of x
```

## Child Issues

- #5435: report export declarations inside namespace bodies.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleExports.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleExports.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- #5435 remains open for implementation.
