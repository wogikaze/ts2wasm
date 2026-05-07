---
id: 3350
title: "Implement Modulemergeconstructor"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as superseded by completed issue 232's non-local module specifier policy
boundary. Fresh triage for `moduleMergeConstructor.ts` parses the ambient
declarations and `new foo.Foo()`, then module graph rejects bare specifier
`"foo"` before module merge constructor behavior becomes actionable.

## Problem

Reference test results show 1 case failing in directory
`moduleMergeConstructor` with diagnostics: import-export. Fresh coverage reports
`UnsupportedModule`, and the resolved dump shows the current blocker is the
existing non-local module specifier diagnostic:

```text
issue-232: unsupported non-local module specifier `foo`
```

Problem: this generated bucket cannot reach module merge constructor semantics
until bare/package module resolution policy changes.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleMergeConstructor.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleMergeConstructor.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

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
- [x] Completed issue 232 already owns the current non-local module specifier policy boundary
- [x] This closed bucket preserves the exact reference path, diagnostic, source context, visible symbols, AST, and TypeScript oracle evidence
- [x] No new child issue was needed because the current blocker is an existing policy boundary

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleMergeConstructor.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleMergeConstructor.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/done/232-resolve-local-relative-es-module-graph.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleMergeConstructor.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleMergeConstructor.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleMergeConstructor.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/moduleMergeConstructor.ts: UnsupportedModule: import-export
```

Source context:

```text
// @target: es2015
// @module: amd

// @filename: foo.d.ts
declare module "foo" {
    export class Foo {
        constructor();
        method1(): any;
    }
}
```

Compiler evidence:

```text
tokens: ok for `declare module "foo"`, `export class Foo`, `export interface Foo`, `import * as foo from "foo"`, and `new foo.Foo()`
ast: ImportNamespace foo from "foo"; ClassDecl Test constructor assigns `this.bar = new foo.Foo()`
resolved: issue-232 unsupported non-local module specifier `foo`; package resolution, import maps, and absolute specifiers are not implemented
```

TypeScript oracle:

```text
TS2664: Invalid module name in augmentation, module 'foo' cannot be found.
TS2307: Cannot find module 'foo' or its corresponding type declarations.
```

Superseded by:

- `issues/done/232-resolve-local-relative-es-module-graph.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleMergeConstructor.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedModule/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleMergeConstructor.ts
result: pass; reproduced issue-232 unsupported non-local module specifier `foo`
date: 2026-05-08
```

Remaining risks:

- If bare/package module resolution policy changes later, this reference may
  advance to ambient module merge, constructor, and namespace import semantics.
