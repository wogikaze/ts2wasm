---
id: 3305
title: "Implement Moduleandinterfacewithsamename"
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

Closed this generated bucket as superseded by the existing same-file namespace
value binding issue.

## Problem

Fresh triage shows the original import/export bucket label is stale. The current
first blocker is name resolution for a non-ambient namespace declaration:

```text
UnresolvedName: unresolved name: `Foo2` at 309..313
```

The parser tokenizes the namespace declarations and preserves the later
qualified expression, but the namespace declaration itself is erased before it
creates a resolver-visible value binding for `Foo2`.

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAndInterfaceWithSameName --detail --no-dashboard-data
```

Observed result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAndInterfaceWithSameName.ts
```

Failure location:

```text
22 | var z2 = Foo2.Bar.y; // Error for using interface name as a value.
```

Source shape:

```ts
namespace Foo2 {
    namespace Bar {
        export var x = 42;
    }

    export interface Bar {
        y: string;
    }
}

var z2 = Foo2.Bar.y; // Error for using interface name as a value.
```

Compiler evidence:

```text
tokens: ok through namespace Foo2, nested namespace Bar, exported interface Bar, and Foo2.Bar.y
ast: ok; retained top-level statement var z2 = Foo2.Bar.y
resolved: UnresolvedName for Foo2 during resolve_names
```

TypeScript oracle evidence:

```text
TS2339: Property 'Bar' does not exist on type 'typeof Foo2'.
```

This confirms `Foo2` should be resolver-visible first; after namespace binding
is fixed, the case should advance to a narrower namespace member visibility
diagnostic.

## Desired final state

Implement the first blocker in
`issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`.
After that lands, rerun this case and record the next diagnostic if the
`Foo2.Bar` interface/value visibility behavior is not already covered.

## Scope

In scope:

- [x] Confirm fresh smart-triage evidence for this generated bucket.
- [x] Match the current first blocker to an existing implementation-ready issue.
- [x] Preserve the later TypeScript oracle diagnostic.

Out of scope:

- Direct implementation from this generated bucket.
- Full TypeScript namespace runtime lowering.
- Interface/value namespace member merge diagnostics beyond the first namespace
  binding blocker.

## Affected paths

Expected implementation owner:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused namespace/name-resolution fixtures or tests

Do not touch from this bucket:

- static ES module loading
- package resolution
- backend namespace emit unless issue 5287 exposes a reviewed runtime shape

## Acceptance criteria

- [x] Superseding issue identified: `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`.
- [x] Exact reproduction commands and observed diagnostics are recorded.
- [x] Later `Foo2.Bar` TypeScript diagnostic is recorded as follow-up risk.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAndInterfaceWithSameName --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAndInterfaceWithSameName.ts
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

- [x] existing issue 5287 owns the first blocker

## Notes

Superseded by `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`.

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

- Once issue 5287 binds `Foo2`, this reference case should advance to the
  TypeScript-style `Foo2.Bar` namespace member visibility diagnostic.
