---
id: 3337
title: "Implement Moduleexports"
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
name-resolution diagnostic issue:
`issues/open/056-implement-name-resolution.md`.

## Problem

Fresh triage shows the file no longer stops on import/export syntax. It parses
the exported namespace/class and reaches a genuinely unresolved CommonJS
`module` identifier in `module.exports`.

TypeScript's oracle also reports `Cannot find name 'module'`, so this is not a
standalone CommonJS module implementation issue for this reference case.

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleExports1 --detail --no-dashboard-data
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleExports1.ts
```

Source shape:

```ts
export namespace TypeScript.Strasse.Street {
    export class Rue {
        public address: string;
    }
}

var rue = new TypeScript.Strasse.Street.Rue();
rue.address = "1 Main Street";

if (!module.exports) module.exports = "";
```

Compiler evidence:

```text
tokens: ok through export namespace, export class, qualified new expression, and module.exports
ast: ok; namespace/export declarations are not the current blocker
resolved: UnresolvedName unresolved name: `module`
```

TypeScript oracle evidence:

```text
TS2564: Property 'address' has no initializer and is not definitely assigned in the constructor.
TS2591: Cannot find name 'module'.
TS2591: Cannot find name 'module'.
```

## Desired final state

This generated bucket is closed. The current observable compiler diagnostic is
covered by the completed name-resolution behavior in issue 056.

## Scope

In scope:

- [x] Confirm the current first compiler blocker.
- [x] Confirm the original import/export blocker is stale.
- [x] Confirm TypeScript also treats `module` as an unresolved name in this fixture.

Out of scope:

- CommonJS `module.exports` runtime support.
- Node ambient type definitions for `module`.
- TS2564 strict-property-initialization diagnostics.

## Affected paths

No implementation paths are assigned from this generated bucket.

## Acceptance criteria

- [x] Exact reproduction commands and diagnostics are recorded.
- [x] Existing completed issue 056 is identified as the diagnostic owner.
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleExports1 --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleExports1.ts
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

Superseded by `issues/open/056-implement-name-resolution.md`.

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

- TS2564 strict-property-initialization diagnostics remain a future semantic
  parity gap, but they are not the current compiler blocker in this coverage
  path.
