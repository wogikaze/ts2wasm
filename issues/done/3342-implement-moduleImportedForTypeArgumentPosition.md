---
id: 3342
title: "Implement Moduleimportedfortypeargumentposition"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import/export bucket as superseded by the completed
module-graph policy boundary in `issues/done/232-resolve-local-relative-es-module-graph.md`.

## Problem

Fresh triage shows the compiler parses the file and reaches module-graph
resolution. The current blocker is the existing issue-232 unsupported
non-local module specifier diagnostic for:

```ts
import M2 = require("moduleImportedForTypeArgumentPosition_0");
```

TypeScript's oracle reports TS2307 for the same specifier.

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleImportedForTypeArgumentPosition --detail --no-dashboard-data
```

Observed result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleImportedForTypeArgumentPosition.ts
```

Source shape:

```ts
// @Filename: moduleImportedForTypeArgumentPosition_0.ts
export interface M2C { }

// @Filename: moduleImportedForTypeArgumentPosition_1.ts
import M2 = require("moduleImportedForTypeArgumentPosition_0");
class C1<T>{ }
class Test1 extends C1<M2.M2C> {
}
```

Compiler evidence:

```text
tokens: ok through export interface, import-equals require, generic class, and type argument
ast: ok; import-equals is represented as ImportDefault with source moduleImportedForTypeArgumentPosition_0
resolved/module_graph: UnsupportedModule issue-232 unsupported non-local module specifier
wat dump: same issue-232 boundary
```

TypeScript oracle evidence:

```text
TS2307: Cannot find module 'moduleImportedForTypeArgumentPosition_0' or its corresponding type declarations.
```

## Desired final state

This generated bucket is closed. The current diagnostic is an already-known
module graph policy boundary, not an executable child issue from this bucket.

## Scope

In scope:

- [x] Confirm the current first compiler blocker.
- [x] Preserve exact reproduction commands and diagnostics.
- [x] Supersede this bucket with completed issue-232 policy evidence.

Out of scope:

- Direct implementation from this generated bucket.
- Bare/package module resolution.
- Import-equals `require(...)` module loading.
- Type argument semantic checks after module resolution advances.

## Affected paths

No implementation paths are assigned from this generated bucket.

## Acceptance criteria

- [x] Exact reproduction commands and diagnostics are recorded.
- [x] Superseding completed issue identified: `issues/done/232-resolve-local-relative-es-module-graph.md`.
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleImportedForTypeArgumentPosition --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleImportedForTypeArgumentPosition.ts
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

- [x] none

## Notes

Superseded by the completed issue-232 module graph policy boundary.

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

- If bare virtual `@Filename` imports become supported later, this path may
  expose import-equals type-position or type-argument diagnostics.
