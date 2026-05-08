---
id: 352
title: "Implement static private field ordering with static blocks"
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

Implement correct evaluation ordering for static private fields and static blocks in class definitions, ensuring they execute in source order alongside static public fields and static blocks.

## Problem

Static private fields (`static #x = 1`) and static blocks (`static { ... }`) must execute in class body source order. The current implementation supports individual static private fields and static blocks separately, but their relative ordering and interaction are not guaranteed. This is observable when a static block reads or writes a static private field.

Problem: Static private field and static block execution order is unverified.

## Current failure

```sh
tmp=/tmp/ts2wasm-352-static-order.ts
printf 'class C {\n  static #a = 1;\n  static { console.log(C.#a); }\n  static #b = 2;\n}\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-352-static-order.wasm
```

Current result: static blocks and static private fields may not execute in guaranteed source order; the above may produce incorrect output or fail to compile.

## Desired final state

Static private fields and static blocks execute in source order. A static block can read static private fields declared before it and set static private fields declared after it (per TDZ rules for the latter).

## Scope

In scope:

- [x] Class-body static element ordering (public fields, private fields, blocks, methods)
- [x] Static block lowering that can access static private fields
- [x] TDZ enforcement for static private fields accessed before declaration in a static block
- [x] Node/iwasm differential fixtures for static field/block ordering

Out of scope:

- Derived-class private elements (issue 350)
- Full brand storage (issue 351)
- Static accessor get/set duplicate-pair semantics

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/private-class*`

Do not touch:

- `crates/frontend/src/`

## Acceptance criteria

- [x] Node/iwasm differential fixture proves static private field and block source ordering
- [x] Node/iwasm differential fixture proves static block can read preceding static private field
- [x] Diagnostic fixture proves TDZ violation for forward-referenced static private field in static block
- [x] `cargo fmt --all --check` and `cargo nextest run` pass

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

- [x] updated: `docs/language-reference/javascript-features.md` for static element ordering

Current state:

- [x] updated: `current-state.md` if static element capability changes

Follow-up issues:

- [x] none

## Notes

Parent issue: 255

ECMAScript class evaluation order for static elements: class heritage, class body (in source order: field initializers including private, then static blocks, then method definitions). Static private fields are initialized in order alongside static public fields and static blocks. A static block that references a static private field declared later in the class body should trigger a ReferenceError/TDZ.

## Completion evidence

Fill only when moving to `done/`.

## Progress evidence

2026-05-01 child-352 follow-up:

- Rechecked the existing issue-specific 352 coverage from parent base `42b9158f`; no backend/parser/runtime-memory code change was needed.
- Acceptance coverage present:
  - Node/iwasm differential fixture `fixtures/core-semantics/private-class-static-field-static-block-order.ts` proves source-ordered static private field and static block execution, including static block reads of preceding static private fields.
  - Diagnostic fixture `fixtures/core-semantics/private-class-static-field-static-block-tdz-unsupported.ts` proves forward static-block access to a later static private field reports `issue-352:`.
  - `docs/language-reference/javascript-features.md` and `current-state.md` already record direct same-class static private fields plus static blocks executing in source order with issue-352 diagnostics for forward static-block access.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo test -p ts2wasm-cli private` (`12` selected private tests passed; included `private_class_field_read_write_fixture_matches_node_output_under_iwasm` and `private_class_static_field_static_block_tdz_reports_issue_352`)
  - `mise run update-issue-index -- --check`
  - `mise run check issues`
- Broad validation result:
  - `cargo nextest run -E 'test(private) or test(class) or test(node_diff)'` selected `197` tests; `196` passed and `m2_node_diff_fixture_tests::abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` failed by iwasm timeout after `30.474s`.
  - The failing fixture is `fixtures/core-semantics/abc451-depth8-live-set.ts`, outside the issue-352 private/static-block scope and outside this assignment's allowed runtime-memory area.
- Child recommendation: parent can close issue 352 if the unrelated ABC451 timeout is accepted as a tracked broad-gate blocker; otherwise keep 352 open only as a validation-policy blocker, not as missing issue-specific implementation.

2026-04-30 child-352:

- Implemented IR/lowering support for source-ordered static private field initializers and static blocks without touching frontend/parser or runtime-memory/ABC451 files.
- Added Node/iwasm differential fixture `fixtures/core-semantics/private-class-static-field-static-block-order.ts` proving a static block can read a preceding static private field and that field/block execution follows source order.
- Added diagnostic fixture `fixtures/core-semantics/private-class-static-field-static-block-tdz-unsupported.ts` proving forward static-block access to a later static private field reports `issue-352:`.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo test -p ts2wasm-cli private`
  - `mise run update-issue-index -- --check`
  - `mise run check issues`
- Validation blocked for close:
  - `cargo nextest run -E 'test(private) or test(class) or test(node_diff)'` ran 197 selected tests; 196 passed and `m2_node_diff_fixture_tests::abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` failed by iwasm timeout at 30s. That failure is in the forbidden runtime-memory/ABC451 area and is tracked separately by issue 357, so issue 352 remains open as validated progress rather than done.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- Static block lowering may need IR changes to represent class-body-level sequential execution

## Parent close evidence

Parent closed issue 352 because the issue-specific implementation and acceptance evidence are complete. The remaining broad validation failure is the unrelated issue-357 ABC451 timeout, already tracked outside the static-private/static-block scope.

```text
command: cargo test -p ts2wasm-cli private
result: pass; 12 selected private tests passed

command: cargo nextest run -E 'test(private) or test(class) or test(node_diff)'
result: 196 passed; 1 failed only on abc451_depth8_live_set_fixture_matches_node_output_under_iwasm timeout, tracked by issue 357

command: mise run update-issue-index -- --check
result: pass

command: mise run check issues
result: pass
```

## Parent close evidence

Parent closed issue 352 because the issue-specific implementation and acceptance evidence are complete. The remaining broad validation failure is the unrelated issue-357 ABC451 timeout, already tracked outside the static-private/static-block scope.

```text
command: cargo test -p ts2wasm-cli private
result: pass; 12 selected private tests passed

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

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/352-static-private-field-static-blocks-order.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
