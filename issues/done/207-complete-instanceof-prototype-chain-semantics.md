---
id: 207
title: "Complete instanceof prototype-chain semantics"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: [048]
blocks: []
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Complete `instanceof` runtime semantics so the operator tests the right-hand constructor's `prototype` against the left-hand object's prototype chain.

## Problem

Issue 030 added parser/lowering support but recorded that emitted behavior is a placeholder. The operator must not count as semantic compatibility until prototype-chain traversal is implemented and differentially verified.

## Desired final state

`obj instanceof Constructor` matches Node.js behavior for ordinary constructors and objects with a traversable prototype chain.

## Scope

In scope:

- [x] Replace the current placeholder result with runtime prototype-chain traversal.
- [x] Validate right-hand operands and report unsupported/diagnostic behavior for unsupported constructor forms.
- [x] Add Node differential fixtures for true, false, and non-object left-hand operands.
- [x] Update docs/current-state/issues when semantic status changes.

Out of scope:

- Custom `Symbol.hasInstance` behavior.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- none

## Acceptance criteria

- [x] `instanceof` no longer returns a fixed placeholder result.
- [x] Differential fixtures match Node.js for ordinary prototype-chain cases.
- [x] Unsupported constructor or custom-hasInstance cases are explicitly diagnosed or tracked.
- [x] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(instanceof)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] update `docs/language-reference/javascript-features.md`

Current state:

- [x] update `current-state.md`

Follow-up issues:

- [x] none

## Notes

Created from issue 203 audit of `issues/done/030-implement-instanceof-operator.md`.

## Completion evidence

Commits:

- `c4d56af` `issue-207: implement instanceof prototype chain`

Validation result:

```text
command: cargo test -p ts2wasm-cli --test m2_node_diff instanceof -- --nocapture
result: pass (2 passed)
date: 2026-04-28

command: cargo nextest run -E 'test(instanceof)'
result: pass (4 passed)
date: 2026-04-28

command: cargo test -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm -- --nocapture
result: pass (1 passed)
date: 2026-04-28

command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: mise run check-agent-state
result: pass
date: 2026-04-28

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-28

command: mise run check-issue-health
result: pass
date: 2026-04-28

command: mise run check-repo-smoke
result: pass
date: 2026-04-28

command: cargo nextest run
result: pass (206 passed, 4 skipped)
date: 2026-04-28

command: cargo clippy --workspace --all-targets -- -D warnings
result: fail (pre-existing clippy::assertions-on-constants in crates/runtime-abi/src/layout.rs)
date: 2026-04-28
```

Remaining risks:

- Custom `Symbol.hasInstance` remains out of scope for this slice.
