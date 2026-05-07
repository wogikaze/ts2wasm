---
id: 1454
title: "Implement Constenumonlymodulemerging"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Closed as stale. Fresh triage and focused reference coverage for
`reference/typescript/tests/cases/compiler/constEnumOnlyModuleMerging.ts` now
report build pass, so this generated bucket no longer has a compiler blocker to
split.

## Problem

Reference test results previously showed one import-export failure for
`constEnumOnlyModuleMerging`. Current compiler behavior builds the file
successfully, and no unsupported diagnostic is emitted for the reference path.

Problem: `constEnumOnlyModuleMerging.ts` was listed as blocked, but current
fresh evidence shows no build blocker remains.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumOnlyModuleMerging.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumOnlyModuleMerging.ts --detail --no-dashboard-data
```

Current result:

```text
BuildPass: ts2wasm build succeeded
coverage: executed=1 build_pass=1 unsupported=0
```

## Desired final state

This generated bucket is closed as stale. No child issue is created because
there is no current compiler blocker to split.

## Scope

In scope:

- [x] Run fresh smart triage for the affected reference file.
- [x] Run focused reference coverage for the affected reference file.
- [x] Confirm there is no current unsupported diagnostic to split.
- [x] Preserve exact reproduction commands and build-pass evidence.

Out of scope:

- Semantic parity work beyond build coverage.
- Direct implementation from this generated bucket.

## Affected paths

Expected:

- none for this cleanup

Do not touch:

- compiler/runtime implementation
- fixtures

## Acceptance criteria

- [x] Fresh triage reports `BuildPass`.
- [x] Focused coverage reports `executed=1`, `build_pass=1`, and
      `unsupported=0`.
- [x] This closed issue records the exact reference path and commands used.
- [x] No child issue is created without a current compiler blocker.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumOnlyModuleMerging.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumOnlyModuleMerging.ts
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

- [x] none; current build step passes

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constEnumOnlyModuleMerging.ts`

## Duplicate detection

No owner issue is needed for the current state because the focused path builds
successfully.

## Smart triage

### Smart triage: Build pass: constEnumOnlyModuleMerging

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/constEnumOnlyModuleMerging.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumOnlyModuleMerging.ts
```

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumOnlyModuleMerging.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
unsupported_diagcodes=
unsupported_features=
semantic_enabled=0
```

Representative source:

```ts
namespace Outer {
    export var x = 1;
}

namespace Outer {
    export const enum A { X }
}

namespace B {
    import O = Outer;
    var x = O.A.X;
    var y = O.x;
}
```

Compiler evidence:

- Tokenization succeeds through the merged namespaces, nested
  `export const enum A { X }`, import alias, and qualified accesses.
- AST and resolved dumps are empty because namespace-only declarations are
  erased by the current frontend pipeline.
- Focused coverage reports build pass and no unsupported diagnostics.

TypeScript oracle evidence:

- TypeScript reports no diagnostics.
- TypeScript hints infer `Outer.x` and `B.y` as `number`, and `B.x` as enum
  type `A`.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumOnlyModuleMerging.ts
result: pass; BuildPass, no current compiler blocker
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumOnlyModuleMerging.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=1 unsupported=0
date: 2026-05-07
```

Remaining risks:

- Semantic parity was not enabled in this focused coverage run. If this case is
  later added to a semantic oracle, it may expose namespace/const-enum runtime
  parity work, but there is no current build blocker in this generated bucket.
