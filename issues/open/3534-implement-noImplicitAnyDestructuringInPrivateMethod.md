---
id: 3534
title: "Implement Noimplicitanydestructuringinprivatemethod"
type: spike
area: ir/compiler
class: superseded
priority: P1
depends_on: [5232]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence shows the current first module
blocker is the existing entry-module `export class` boundary owned by issue
5232.

## Problem

Fresh triage for `noImplicitAnyDestructuringInPrivateMethod.ts` tokenizes and
parses the type alias, `export class Bar`, and private method object-binding
parameter `{ a, }: Arg`. AST/resolved output reaches the exported class, then
module build reports issue-5005 for entry-module `export class Bar`.

Focused coverage still classifies the representative as `UnsupportedSyntax:
destructuring`, but smart triage exposes the concrete first module blocker as
the entry-module export-class boundary. The matching owner is
`issues/open/5232-support-entry-export-class-declarations.md`.

Problem: generated import/export bucket is superseded by issue 5232's
entry-module `export class` implementation slice.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringInPrivateMethod.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringInPrivateMethod.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1, build_pass=0, unsupported=1
coverage unsupported_diagcodes=UnsupportedSyntax:1
coverage unsupported_features=destructuring:1
triage: UnsupportedModule / issue-5005 / import-export
```

Triage diagnostic:

```text
UnsupportedModule: issue-5005: entry module `export Bar` uses a declaration form outside the current static export slice; only export const and export default are supported at 109..189
```

Source context:

```ts
type Arg = {
    a: number;
};
export class Bar {
    private bar({ a, }: Arg): number {
        return a;
    }
}
export declare class Bar2 {
    private bar({ a, });
}
```

Compiler evidence:

```text
tokens: ok through type Arg, export class Bar, private method object-binding parameter, and export declare class Bar2
ast: ok; ExportDecl(ClassDecl Bar) with method parameter represented as "{a}"
resolved: ok; ClassDecl Bar has method bar with parameter "{a}" and return Ident("a")
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5232-support-entry-export-class-declarations.md`. Do not implement
directly from this bucket.

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
- [x] Owner issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Owner issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Owner issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringInPrivateMethod.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringInPrivateMethod.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only issue lifecycle change.
- `cargo nextest run`; metadata-only issue lifecycle change.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing: `issues/open/5232-support-entry-export-class-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringInPrivateMethod.ts`

## Duplicate detection

- `issues/open/5232-support-entry-export-class-declarations.md` is the exact
  owner for the current entry-module `export class Bar` issue-5005 boundary.
- Destructuring/private-method semantics remain later surfaces after issue 5232
  advances this representative.

## Smart triage

### Smart triage: Triage import export: noImplicitAnyDestructuringInPrivateMethod

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/noImplicitAnyDestructuringInPrivateMethod.ts`

Current compiler message:

```text
issue-5005: entry module `export Bar` uses a declaration form outside the current static export slice; only export const and export default are supported
```

Folded into issue 5232 because the current actionable blocker is entry-module
`export class`, not a direct destructuring implementation slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
