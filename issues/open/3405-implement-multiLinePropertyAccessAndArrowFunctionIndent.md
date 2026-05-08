---
id: 3405
title: "Close multiLinePropertyAccessAndArrowFunctionIndent to TS1108 owner"
type: maintenance
area: compiler/diagnostics
class: superseded
priority: P2
depends_on: [711]
blocks: []
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Closed the generated `multiLinePropertyAccessAndArrowFunctionIndent` bucket as covered by implementation-ready issue #711.

## Problem

`reference/typescript/tests/cases/compiler/multiLinePropertyAccessAndArrowFunctionIndent1.ts` fails with the same top-level return diagnostic family already represented by #711.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiLinePropertyAccessAndArrowFunctionIndent1.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=InvalidTopLevelReturn:1, unsupported_features=top-level-return:1
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiLinePropertyAccessAndArrowFunctionIndent1.ts
result: InvalidTopLevelReturn at top-level return; TypeScript oracle reports TS1108
date: 2026-05-08
```

## Evidence

Source:

```ts
return this.edit(role)
    .then((role: Role) =>
        this.roleService.add(role)
            .then((data: ng.IHttpPromiseCallbackArg<Role>) => data.data));
```

The frontend AST is valid and contains one top-level `Return` whose expression is the full chained call. Validation rejects it as `InvalidTopLevelReturn`. TypeScript also rejects this case with TS1108: a `return` statement can only be used within a function body.

## Owner

- #711: report TS1108 for top-level return statements.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiLinePropertyAccessAndArrowFunctionIndent1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiLinePropertyAccessAndArrowFunctionIndent1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/asiReturn.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/asiReturn.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- #711 remains open for implementation.
