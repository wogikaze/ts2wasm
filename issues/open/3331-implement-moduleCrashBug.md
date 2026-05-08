---
id: 3331
title: "Implement Modulecrashbug"
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

Closed this generated import/export bucket as superseded by the namespace
variable annotation diagnostic issue:
`issues/open/5411-report-ts2709-for-namespace-variable-annotation.md`.

## Problem

Fresh triage shows the original import/export blocker is stale. The compiler now
build-passes the reference case, but TypeScript reports TS2709 for a namespace
root used as a variable type annotation:

```text
TS2709: Cannot use namespace '_modes' as a type.
```

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleCrashBug --detail --no-dashboard-data
```

Observed result:

```text
executed=1
build_pass=1
unsupported=0
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleCrashBug1.ts
```

Source shape:

```ts
namespace _modes {
 export interface IMode {
 }

 class Mode {
 }
}

namespace editor {
 import modes = _modes;
}

var m : _modes;
```

Compiler evidence:

```text
tokens: ok through namespace _modes, namespace editor import alias, and var m: _modes
ast/resolved: retains only Let m; namespace declarations and annotation are erased
```

TypeScript oracle evidence:

```text
TS2709: Cannot use namespace '_modes' as a type. at line 19, character 9
```

## Desired final state

Implement the first blocker in
`issues/open/5411-report-ts2709-for-namespace-variable-annotation.md`.

## Scope

In scope:

- [x] Confirm the generated bucket's current evidence.
- [x] Match the current first blocker to an existing implementation-ready issue.
- [x] Preserve exact reproduction commands and TypeScript diagnostics.

Out of scope:

- Direct implementation from this generated bucket.
- Full namespace runtime lowering.
- Namespace import-alias semantics after this diagnostic advances.

## Affected paths

Expected implementation owner:

- `crates/frontend/src/`
- focused frontend/resolver tests or fixtures

Do not touch from this bucket:

- backend namespace emit
- module/package resolution

## Acceptance criteria

- [x] Superseding issue identified: `issues/open/5411-report-ts2709-for-namespace-variable-annotation.md`.
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleCrashBug --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleCrashBug1.ts
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

- [x] existing issue 5411 owns the first blocker

## Notes

Superseded by
`issues/open/5411-report-ts2709-for-namespace-variable-annotation.md`.

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

- After issue 5411 lands, this case may expose namespace import-alias behavior
  or further namespace/type diagnostics.
