---
id: 062e
title: "Implement function closures"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: ["062c"]
blocks: []
status: done
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
---

Problem: Captured lexical environments require different resolver, lowering,
runtime root, and backend behavior than direct local function calls.

## Summary

Implement closure capture for ordinary functions as a focused slice after basic
function declarations and direct calls are available.

## Scope

In scope:

- [x] Capturing immutable outer locals used by a returned or nested function.
- [x] Calling a closure after the declaring scope has returned.
- [x] GC/rooting evidence for captured values when allocation pressure is involved.
- [x] Node/iwasm differential fixtures for basic closure capture.

Out of scope:

- Dynamic Function constructor behavior.
- Full environment mutation semantics beyond the selected fixtures.
- Generator/async closure semantics.
- Function object metadata.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/` unless ABI changes are explicitly required and reviewed.

## Progress

2026-04-29 child-062e progress slice:

- Non-escaping nested ordinary functions can capture immutable outer locals and
  be called before the declaring activation returns.
- Returning a nested ordinary closure now lowers to a heap closure object and
  can be called after the declaring activation returns.
- Mutable captured outer locals remain rejected with `issue-062e` because the
  current narrow slice passes captures as hidden values, not as a shared mutable
  heap environment.
- Follow-up issue `issues/open/062g-heap-closure-object-abi-and-rooting.md`
  completed returned closure object ABI, dispatch, and GC rooting.

## Acceptance criteria

- [x] A nested function can capture an outer local and return the captured value.
- [x] A returned closure keeps captured values live across the selected fixture.
- [x] Closure allocation/rooting behavior is covered by a regression fixture.
- [x] Unsupported escaping/mutation forms produce issue-linked diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(closure) or test(function) or test(node_diff)'
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 94 --detail
```

Not run:

- none

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `a558aca269c61f0ba64f82d6799d729874930b0f`
- `b1e9a98c8fc94ccf794998ba97376045e7438cb9`
- `115d5cf74a9d19840303ff951463264529deb415`
- `29d57aced2fdcc3273ead0997bac39797780e0e5`

Validation result:

```text
command: cargo nextest run -E 'test(closure) or test(function) or test(node_diff) or test(gc)'
result: pass; 42 tests
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/ordinary-function-closure-gc-pressure.ts -o /tmp/ts2wasm-258-gc-pressure-parent.wasm && iwasm /tmp/ts2wasm-258-gc-pressure-parent.wasm
result: pass; output `closure-object-alive`
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-29

command: mise run check issues
result: pass
date: 2026-04-29
```

Remaining risks:

- Mutable captured environments, dynamic `Function`, function metadata,
  generator/async closures, and broader closure dispatch forms remain outside
  this issue and are tracked by separate issues/scopes.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/062e-function-closures.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
