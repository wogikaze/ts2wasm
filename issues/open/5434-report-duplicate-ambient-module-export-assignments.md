---
id: 5434
title: "Report duplicate ambient module export assignments"
type: bug
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report duplicate `export =` diagnostics inside `declare module` blocks instead of erasing the ambient module as a clean build-pass.

## Problem

`multipleExportAssignmentsInAmbientDeclaration.ts` contains two `export =` assignments in the same ambient module. ts2wasm currently erases the whole ambient module and build-passes, while TypeScript reports TS2300 duplicate identifier `export=`.

Problem: duplicate `export =` declarations inside an ambient module are hidden by ambient erasure and produce a false build-pass.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleExportAssignmentsInAmbientDeclaration.ts
```

Observed:

```text
BuildPass: ts2wasm build succeeded
TypeScript oracle: TS2300 duplicate identifier 'export='
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleExportAssignmentsInAmbientDeclaration.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=1
unsupported=0
blocked=0
```

## Source Context

```ts
declare module "m1" {
    var a: number
    var b: number;
    export = a;
    export = b;
}
```

The lexer sees both `Export Equal Ident` sequences, but the ambient module body is erased before a duplicate export-assignment diagnostic can be emitted.

## Desired final state

The frontend reports a source-spanned duplicate `export=` diagnostic for ambient module declarations with more than one export assignment.

## Scope

In scope:

- [ ] Detect duplicate `export =` entries within a single `declare module "name" { ... }` block.
- [ ] Report diagnostics at both exported identifiers or at least at the duplicate `export =` span.
- [ ] Add focused coverage for `declare module "m" { export = a; export = b; }`.

Out of scope:

- Runtime/CommonJS export assignment lowering.
- Non-ambient top-level duplicate export assignments; those require #5346 first.
- `export =` mixed with other exported declarations; tracked by #5306.
- Ambient module import-alias resolution; tracked by #5399.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/`
- focused frontend or CLI diagnostic tests

Do not touch:

- backend emit
- package/module resolution
- runtime ABI

## Acceptance criteria

- [ ] `multipleExportAssignmentsInAmbientDeclaration.ts` no longer build-passes silently; it reports duplicate `export=`.
- [ ] A focused test covers duplicate `export =` inside a `declare module` block.
- [ ] Ambient modules with a single `export =` remain erased or rejected according to the existing ambient module boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(ambient) or test(export)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleExportAssignmentsInAmbientDeclaration.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleExportAssignmentsInAmbientDeclaration.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Notes

Split from #3412 on 2026-05-08. This issue is intentionally narrower than CommonJS export assignment parsing (#5346) because this representative already build-passes by ambient erasure.
