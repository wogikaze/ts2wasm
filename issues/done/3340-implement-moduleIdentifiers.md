---
id: 3340
title: "Implement Moduleidentifiers"
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

Closed this generated import/export bucket as superseded by the existing
implementation-ready namespace value binding issue:
`issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`.

## Problem

Fresh triage shows the file no longer has an import/export syntax blocker. The
current first blocker is a same-file non-ambient namespace root that is not
visible for qualified value access.

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleIdentifiers --detail --no-dashboard-data
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleIdentifiers.ts
```

Source shape:

```ts
namespace M {
    interface P { x: number; y: number; }
    export var a = 1
}

var x1 = M.a;
```

Compiler evidence:

```text
tokens: ok through namespace M, interface P, export var a, and M.a
ast: only the outside `var x1 = M.a` remains after namespace erasure
resolved: UnresolvedName unresolved name: `M` at 149..150
```

TypeScript oracle evidence:

```text
ok: true
diagnostics: []
```

## Desired final state

Implement issue 5287. Do not implement directly from this generated bucket.

## Scope

In scope:

- [x] Confirm current first blocker.
- [x] Confirm the generated import/export classification is stale.
- [x] Supersede this bucket with the existing same-file namespace value binding issue.

Out of scope:

- Direct implementation from this generated bucket.
- Ambient namespace value access.
- Non-exported namespace member diagnostics.

## Affected paths

Expected implementation owner:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused namespace/name-resolution tests

## Acceptance criteria

- [x] Exact reproduction commands and diagnostics are recorded.
- [x] Superseding issue identified: `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`.
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleIdentifiers --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleIdentifiers.ts
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

- [x] existing issue 5287 owns the blocker

## Notes

Superseded by
`issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`.

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

- After issue 5287 lands, this path may expose value lowering/runtime behavior
  for exported namespace vars.
