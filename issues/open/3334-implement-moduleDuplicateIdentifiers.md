---
id: 3334
title: "Implement Moduleduplicateidentifiers"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import/export bucket as superseded by the initialized
`export var` implementation issue:
`issues/open/5285-support-export-var-initializer-declarations.md`.

## Problem

Fresh triage shows the current first blocker is the first initialized exported
variable declaration:

```text
issue-055: unsupported variable export; module resolution and loading are not implemented at 42..48
```

The intended duplicate-export diagnostics are not reachable until the frontend
accepts `export var Foo = 2;` and `export var Foo = 42;`.

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleDuplicateIdentifiers --detail --no-dashboard-data
```

Observed result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleDuplicateIdentifiers.ts
```

Source shape:

```ts
export var Foo = 2;
export var Foo = 42; // Should error

export interface Bar {
    _brand1: any;
}

export interface Bar { // Shouldn't error
    _brand2: any;
}

export namespace FooBar {
    export var member1 = 2;
}

export namespace FooBar { // Shouldn't error
    export var member2 = 42;
}

export class Kettle {
    member1 = 2;
}

export class Kettle { // Should error
    member2 = 42;
}
```

Compiler evidence:

```text
tokens: ok through export var, export interface, export namespace, export class, export enum forms
ast/resolved: issue-055 unsupported variable export at the leading export keyword
```

TypeScript oracle evidence:

```text
TS2323: Cannot redeclare exported variable 'Foo'.
TS2323: Cannot redeclare exported variable 'Foo'.
TS2300: Duplicate identifier 'Kettle'.
TS2300: Duplicate identifier 'Kettle'.
```

## Desired final state

Implement the first blocker in
`issues/open/5285-support-export-var-initializer-declarations.md`. After that
lands, rerun this case and split any duplicate exported variable or duplicate
class diagnostics that become reachable.

## Scope

In scope:

- [x] Confirm the generated bucket's current evidence.
- [x] Match the current first blocker to an existing implementation-ready issue.
- [x] Preserve exact reproduction commands and later duplicate diagnostics.

Out of scope:

- Direct implementation from this generated bucket.
- Duplicate exported variable diagnostics.
- Duplicate exported class diagnostics.
- Namespace/interface merge semantics after export-var parsing advances.

## Affected paths

Expected implementation owner:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- focused module/export tests

Do not touch from this bucket:

- backend/runtime lowering
- package resolution

## Acceptance criteria

- [x] Superseding issue identified: `issues/open/5285-support-export-var-initializer-declarations.md`.
- [x] Exact reproduction commands and observed diagnostics are recorded.
- [x] The generated bucket no longer remains as a stale blocked import/export issue.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
git diff --cached --check
```

Reference commands already run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleDuplicateIdentifiers --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleDuplicateIdentifiers.ts
```

Not run:

- `cargo fmt --all --check` (issue lifecycle only; no Rust changes)
- `cargo nextest run` (issue lifecycle only; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing issue 5285 owns the first blocker

## Notes

Superseded by
`issues/open/5285-support-export-var-initializer-declarations.md`.

## Completion evidence

Commits:

- filled by commit

Validation result:

```text
command: python scripts/manager.py update-issue-index
result: pass
date: 2026-05-08

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-08

command: git diff --check
result: pass
date: 2026-05-08

command: git diff --cached --check
result: pass
date: 2026-05-08
```

Remaining risks:

- After issue 5285 lands, this case should expose duplicate exported variable
  and duplicate exported class diagnostics.
