---
id: 3333
title: "Implement Moduledetectionisolatedmodulescjsfilescope"
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

Closed this generated duplicate-local bucket by splitting the current blocker
into a focused implementation issue:
`issues/open/5412-report-ts2451-duplicate-const-filename-sections.md`.

## Problem

Fresh triage shows the compiler already detects the duplicate local binding,
but it remains an unsupported `DuplicateLocal` blocker. TypeScript reports
TS2451 duplicate block-scoped variable diagnostics for the same source shape.

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleDetectionIsolatedModulesCjsFileScope --detail --no-dashboard-data
```

Observed result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=DuplicateLocal:1
unsupported_features=duplicate-local:1
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleDetectionIsolatedModulesCjsFileScope.ts
```

Source shape:

```ts
// @filename: filename.cts
const a = 2;
// @filename: filename.mts
const a = 2;
```

Compiler evidence:

```text
tokens: ok for both @filename sections and both const a declarations
ast: two Let { name: "a", is_var: false } declarations
resolved: DuplicateLocal duplicate local binding: `a` at 191..203
```

TypeScript oracle evidence:

```text
TS2451: Cannot redeclare block-scoped variable 'a'. at line 7, character 7
TS2451: Cannot redeclare block-scoped variable 'a'. at line 9, character 7
```

## Desired final state

Implement the focused diagnostic work in
`issues/open/5412-report-ts2451-duplicate-const-filename-sections.md`.

## Scope

In scope:

- [x] Confirm the generated bucket's current evidence.
- [x] Split the duplicate-local blocker into an implementation-ready issue.
- [x] Preserve exact reproduction commands and TypeScript diagnostics.

Out of scope:

- Direct implementation from this generated bucket.
- Broad test262 DuplicateLocal parent work.
- Exported external-module binding isolation.

## Affected paths

Expected implementation owner:

- `crates/ir/src/name_resolver.rs`
- focused frontend/IR tests or fixtures

Do not touch from this bucket:

- backend/runtime lowering
- package or on-disk module resolution

## Acceptance criteria

- [x] Created `issues/open/5412-report-ts2451-duplicate-const-filename-sections.md`.
- [x] Exact reproduction commands and observed diagnostics are recorded.
- [x] The generated bucket no longer remains as a broad triage-needed issue.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleDetectionIsolatedModulesCjsFileScope --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleDetectionIsolatedModulesCjsFileScope.ts
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

- [x] created: `issues/open/5412-report-ts2451-duplicate-const-filename-sections.md`

## Notes

Split to `issues/open/5412-report-ts2451-duplicate-const-filename-sections.md`.

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

- After issue 5412 maps this true duplicate to TS2451, the reference case may
  expose declaration emit or module detection diagnostics.
