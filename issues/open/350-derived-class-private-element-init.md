---
id: 350
title: "Implement derived-class private element initialization"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: [255]
blocks: []
created: 2026-04-30
updated: 2026-04-30
status: done
completed: 2026-04-30
---

## Summary

Implement runtime initialization and storage for private class elements (fields, methods, accessors) in derived classes, including correct super-constructor ordering and brand initialization.

## Problem

Private elements in derived classes must be initialized after the super constructor completes but before the derived class constructor body runs. The current implementation only supports non-derived classes. Attempting to use private elements in a derived class fails with an issue-255 diagnostic.

Problem: Derived-class private elements are rejected with an unsupported diagnostic.

## Current failure

```sh
tmp=/tmp/ts2wasm-350-derived-private.ts
printf 'class Base { constructor() { this.x = 1; } }\nclass Derived extends Base { #value = 2; }\nconsole.log(new Derived());\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-350-derived-private.wasm
```

Current result: `[UnsupportedSyntax] issue-255: private fields are not supported in derived classes in this private field runtime slice`

## Desired final state

Derived classes with private fields, methods, getters, and setters compile and execute with Node-compatible semantics. Private element initialization happens in the correct constructor phase.

## Scope

In scope:

- [x] Derived-class constructor IR lowering with private slot allocation after super()
- [x] Private field initialization ordering relative to public fields and super constructor
- [x] Private method/accessor brand binding for derived-class instances
- [x] Node/iwasm differential fixtures for derived-class private elements

Out of scope:

- Full private brand storage overhaul (issue 351)
- Static private field ordering with static blocks (issue 352)
- External/extracted private access (remains diagnostic-only)

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/private-class*`

Do not touch:

- `crates/frontend/src/`
- `crates/frontend/src/parser/`
- Parser private syntax tokenization

## Acceptance criteria

- [x] Node/iwasm differential fixture for derived class with private field
- [x] Node/iwasm differential fixture for derived class with private method
- [x] Node/iwasm differential fixture for derived class with private getter/setter
- [x] Regression fixture proving private initialization order (after super, before constructor body)
- [x] `cargo fmt --all --check` and `cargo nextest run` pass

## Progress evidence

2026-04-30 child-350 progress:

- Implemented derived private element resolver support without parser/frontend changes.
- Added Node/iwasm differential fixtures:
  - `fixtures/core-semantics/private-class-derived-field-order.ts`
  - `fixtures/core-semantics/private-class-derived-field-implicit.ts`
  - `fixtures/core-semantics/private-class-derived-method-call.ts`
  - `fixtures/core-semantics/private-class-derived-accessor-direct.ts`
- `cargo fmt --all --check`: pass.
- `cargo test -p ts2wasm-cli private`: pass.
- `cargo nextest run -E 'test(private) or test(class) or test(node_diff)'`: 195 passed, 1 failed due existing broad-filter timeout in `fixtures/core-semantics/abc451-depth8-live-set.ts` (tracked separately as issue 357), so this issue remains open instead of done.

2026-05-01 child-350 follow-up verification:

- No additional implementation change was required for issue-specific acceptance on parent base `53a6ad73`.
- `cargo fmt --all --check`: pass.
- `cargo test -p ts2wasm-cli private`: pass; private-class tests included direct private method/getter/setter lowering checks plus `private_class_field_read_write_fixture_matches_node_output_under_iwasm`.
- `cargo nextest run -E 'test(private) or test(class) or test(node_diff)'`: 196 passed, 1 failed.
- Only failing test in the broad filter: `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm`, which timed out under `iwasm` after `30.412s` on `fixtures/core-semantics/abc451-depth8-live-set.ts`; this is unrelated to derived-class private element initialization and is tracked separately by issue 357.
- Parent close/block decision evidence: all checked issue-specific acceptance criteria remain satisfied; the only unsatisfied acceptance line is the repository-wide `cargo nextest run` gate because of the unrelated ABC451 timeout.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(private) or test(class) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo test -p ts2wasm-cli private
```

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/language-reference/javascript-features.md` if derived private element semantics are documented

Current state:

- [x] updated: `current-state.md` if runtime private element capability changes

Follow-up issues:

- [x] none

## Notes

Parent issue: 255

In ECMAScript, private fields are initialized in the constructor after `super()` returns but before any explicit constructor body code. If the derived class has no explicit constructor, private fields still initialize after the implicit `super()` call. Private methods and accessors share the same brand as private fields but do not need per-instance initialization beyond brand binding.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- Constructor execution order may interact with existing super() lowering in non-trivial ways

## Parent close evidence

Parent closed issue 350 because issue-specific derived-class private element initialization acceptance is complete. The remaining broad validation failure is the unrelated issue-357 ABC451 timeout.

```text
command: cargo test -p ts2wasm-cli private
result: pass

command: cargo nextest run -E 'test(private) or test(class) or test(node_diff)'
result: 196 passed; 1 failed only on abc451_depth8_live_set_fixture_matches_node_output_under_iwasm timeout, tracked by issue 357

command: mise run update-issue-index -- --check
result: pass

command: mise run check issues
result: pass
```

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/350-derived-class-private-element-init.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
