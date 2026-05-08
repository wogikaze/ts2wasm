---
id: 5407
title: "Report ambient getter implementation bodies"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report TypeScript's ambient-context implementation diagnostic when a getter
inside a `declare class` has a body. Keep bodyless ambient getter signatures
erased.

## Problem

`ambientGetters.ts` currently records a ts2wasm `BuildPass`, but TypeScript
reports TS1183 for `declare class B { get length() { return 0; } }`.

Problem: ambient class getter implementation bodies are currently erased
instead of being diagnosed at the source span where TypeScript reports TS1183.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientGetters.ts
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientGetters.ts --detail --no-dashboard-data
```

Current compiler evidence:

```text
ts2wasm: BuildPass
ast: ok; []
resolved: ok; []
```

TypeScript oracle evidence:

```text
TS1183: An implementation cannot be declared in ambient contexts.
source: declare class B { get length() { return 0; } }
span: getter body `{` at line 8, character 18
```

## Desired final state

The ambient class parser detects getter bodies while scanning a `declare
class` body and reports a source-spanned diagnostic equivalent to TS1183 before
erasing the ambient declaration.

## Scope

In scope:

- [ ] Detect `get name(...) { ... }` inside `declare class` bodies and emit a source-spanned diagnostic equivalent to TS1183.
- [ ] Keep bodyless ambient getter signatures erased without runtime bindings.

Out of scope:

- Runtime accessor lowering for non-ambient classes.
- Setter bodies in ambient classes.
- Constructor bodies in ambient classes, tracked by issue 5358.
- Ambient namespace/module ownership.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`
- unrelated class runtime lowering

## Acceptance criteria

- [ ] `ambientGetters.ts` no longer records `BuildPass` when TS1183 applies.
- [ ] `declare class C { get x() { return 1; } }` reports the ambient implementation diagnostic with a source span at the getter body.
- [ ] `declare class C { get x(): number; }` remains accepted and erased.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(ambient) or test(accessor)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientGetters.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientGetters.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

## Docs / current-state / issue sync

Final-state docs: not affected.
Current state: not affected.
Follow-up issues: none.

## Notes

Split from `issues/done/156-implement-ambientGetters.md` on 2026-05-08 after
the generated build blocker was resolved by ambient declaration erasure.

Related issue:

- `issues/done/5358-report-constructor-bodies-in-ambient-class-declarations.md` covers the same TS1183 boundary for constructor bodies.

## Completion evidence

Fill only when implemented.
