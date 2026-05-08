---
id: 3560
title: "Implement Noimplicitusestrict"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5283]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence shows all five module variants
reach the existing entry-module `export var` issue 5283.

## Problem

Fresh triage shows all affected files consist of module-target comments plus:

```ts
export var x = 0;
```

Each file tokenizes the `Export Var Ident("x")` sequence, then stops at the
same issue-055 variable-export boundary:

```text
UnsupportedModule: issue-055: unsupported variable export; module resolution and loading are not implemented
```

Problem: this generated bucket is superseded by issue 5283, which owns simple
entry-module `export var` declarations.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitUseStrict_commonjs.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitUseStrict --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=5 build_pass=0 unsupported=5
commonjs/es6/system/umd/amd triage: UnsupportedModule issue-055 unsupported variable export
```

## Desired final state

This generated bucket is closed as superseded by
`issues/done/5283-support-entry-export-var-declarations.md`. Do not implement
directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5283 covers this bucket
- [x] Fold into existing issue 5283 for the same observable module boundary
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
- [x] Existing issue 5283 contains the implementation owner; this done issue contains the exact focused triage command
- [x] Evidence includes affected paths, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5283 acceptance covers the `export var` issue-055 boundary

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitUseStrict --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitUseStrict_commonjs.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only.
- `cargo nextest run`; issue metadata only.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into: `issues/done/5283-support-entry-export-var-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitUseStrict_commonjs.ts`
- `reference/typescript/tests/cases/compiler/noImplicitUseStrict_amd.ts`
- `reference/typescript/tests/cases/compiler/noImplicitUseStrict_es6.ts`
- `reference/typescript/tests/cases/compiler/noImplicitUseStrict_umd.ts`
- `reference/typescript/tests/cases/compiler/noImplicitUseStrict_system.ts`

## Duplicate detection

- `issues/done/5283-support-entry-export-var-declarations.md` is the exact
  owner for the current `export var x = 0;` issue-055 boundary.
- `issues/open/517-implement-alwaysStrictNoImplicitUseStrict.md` and
  `issues/open/603-implement-alwaysStrictNoImplicitUseStrict.md` are older
  same-theme generated buckets, not the current implementation owner.
- `issues/open/432-implement-import-export.md` is broader than the current
  executable slice.
- Folded into issue 5283.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage import export: noImplicitUseStrict commonjs

- Issue class: triage-needed
- Feature label: import-export
- Diagnostic: UnsupportedModule / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/noImplicitUseStrict_commonjs.ts
```

Current compiler message:

```text
issue-055: unsupported variable export; module resolution and loading are not implemented
```

Compiler evidence:

```text
all five files: tokens ok; Export, Var, Ident("x"), Equal, Number(0), Semicolon
ast/resolved: fail at issue-055 variable export boundary
TypeScript oracle: diagnostics=[]; binding x has type number
```

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitUseStrict --detail --no-dashboard-data
result: pass; executed=5 unsupported=5, all at current module/export boundary
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitUseStrict_commonjs.ts
result: pass; generated smart triage evidence and duplicate review material
date: 2026-05-08
```

Remaining risks:

- After issue 5283 advances, these fixtures may expose module-target strict
  prologue or emit behavior, but the current first blocker is `export var`.
