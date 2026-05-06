---
id: 1212
title: "Implement Classfunctionmerging Import Export"
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

Closed as stale: fresh smart triage for `classFunctionMerging.ts` now reaches a
ts2wasm build pass, so there is no current compiler blocker to split from this
generated bucket.

## Problem

Reference test results previously showed 1 case failing in directory
`classFunctionMerging-import-export` with diagnostics: import-export. Fresh
triage shows the representative case builds successfully.

Problem: this generated bucket was stale and had no smart-triage evidence. The
current runner view reports `BuildPass` for the representative reference path,
so the bucket should be closed instead of converted into an implementation
issue.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFunctionMerging.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFunctionMerging.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm that no current compiler failure remains to split
- [x] Close this stale bucket without creating a duplicate child issue
- [x] Preserve exact reproduction commands and representative build-pass evidence

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded by current build-pass evidence
- [x] No child issue is needed because the representative path no longer reports a compiler blocker
- [x] This issue includes the exact path, diagnostic code, source context, visible symbols, and parser/TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and build-pass result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFunctionMerging.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFunctionMerging.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only, no Rust code changed
- `cargo nextest run`; issue metadata only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classFunctionMerging.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/done/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/543-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/549-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/662-implement-arrayAssignmentTest-import-export.md` - Implement Arrayassignmenttest Import Export (same feature label, title overlap)
- `issues/open/732-implement-assignmentCompatability-import-export.md` - Implement Assignmentcompatability Import Export (same feature label, title overlap)
- `issues/open/766-implement-augmentedTypesEnum-import-export.md` - Implement Augmentedtypesenum Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)

## Smart triage

Fresh triage shows this generated import/export bucket is no longer a current
compiler blocker.

### Smart triage: Build pass: classFunctionMerging

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/classFunctionMerging.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFunctionMerging.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFunctionMerging.ts --detail --no-dashboard-data
```

Coverage result:

```text
build_pass=1
unsupported=0
unsupported_diagcodes=
unsupported_features=
semantic_enabled=0
```

Source context:

```ts
declare function Foo (x: number): Foo.Inst;
declare class Foo {
    constructor(x: string);
}
declare namespace Foo {
    export type Inst = number;
}

const a = new Foo("");
const b = Foo(12);
```

Compiler evidence:

```text
tokens: ok
ast: ok; ambient declarations are erased, with remaining let bindings for `a` and `b`
resolved: ok; `new Foo("")` and `Foo(12)` lower to resolved expressions
build: succeeded
```

TypeScript oracle evidence:

```text
typescript ok: true
diagnostics: []
binding a type: Foo
binding b type: number
```

## Completion evidence

Commits:

- Closed as stale by current smart-triage/build-pass evidence; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFunctionMerging.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; current result is BuildPass
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFunctionMerging.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; 1 executed, build_pass=1, unsupported=0
date: 2026-05-07
```

Remaining risks:

- Semantic parity is not enabled for this coverage window, so later semantic
  assertions for ambient class/function/namespace merge behavior may still need
  separate verification. There is no current build blocker in this bucket.
