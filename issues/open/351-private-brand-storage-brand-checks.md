---
id: 351
title: "Implement full private brand storage and brand-checking semantics"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [255]
blocks: []
created: 2026-04-30
updated: 2026-05-01
---

## Summary

Replace the current internal-slot private storage with full private brand storage per ECMAScript, and implement runtime brand-checking semantics so that accessing a private element on an object that does not have the correct brand throws a TypeError.

## Problem

The current implementation uses internal slots appended to class instances for private field storage. This lacks the ECMAScript brand concept: in Node, `class C { #x; } let o = {}; o.#x` throws TypeError because `o` lacks C's brand. The current implementation would either silently fail or access the wrong slot.

Problem: No runtime brand storage or brand-checking semantics for private elements.

## Current failure

```sh
tmp=/tmp/ts2wasm-351-brand-check.ts
printf 'class C { #x = 1; }\nlet o = {};\ntry { console.log(o.#x); } catch (e) { console.log(e.name); }\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-351-brand-check.wasm
```

Current result: external private access is rejected at compile time with an issue-255 diagnostic, so no runtime brand check exists.

## Desired final state

Every class with private elements has a unique brand. Runtime private element access checks the receiver's brand and throws TypeError on mismatch. Internal private slot layout is associated with the brand rather than appended unconditionally.

## Scope

In scope:

- [ ] Per-class private brand generation and storage
- [ ] Brand attachment to instances during construction
- [ ] Runtime brand-check helper for private field/method/accessor access
- [ ] TypeError throwing on brand mismatch for external private access attempts
- [ ] Node/iwasm differential fixtures for brand-check behavior

Out of scope:

- Derived-class private elements (issue 350)
- Static private field ordering with static blocks (issue 352)
- Syntax support changes (parser already handles `#x`)

## Affected paths

Expected:

- `crates/runtime-abi/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/private-class*`

Do not touch:

- `crates/frontend/src/`

## Acceptance criteria

- [ ] Node/iwasm differential fixture proves external private access throws TypeError
- [ ] Node/iwasm differential fixture proves same-class access succeeds
- [ ] Node/iwasm differential fixture proves subclass access fails (no inherited brand)
- [ ] Runtime helper tests cover brand check and TypeError throw paths
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass

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

- [ ] updated: `docs/14-runtime-abi.md` for brand storage ABI
- [ ] updated: `docs/language-reference/javascript-features.md` for brand semantics

Current state:

- [ ] updated: `current-state.md` if runtime brand capability changes

Follow-up issues:

- [ ] none

## Notes

Parent issue: 255

In ECMAScript, each class definition with private elements creates a PrivateEnvironment containing private names. Each private name is associated with a brand (the class constructor). Instances created by that constructor carry the brand. Private field access (`o.#x`) checks that `o` has the brand of the class where `#x` was declared.

The current implementation rejects external private access at compile time. Moving to runtime brand checks requires lowering external private access to runtime brand-check calls, which may require changes to how private access is represented in IR.

## Completion evidence

Fill only when moving to `done/`.

## Progress evidence

2026-05-01 child diagnostic slice:

- Added a cataloged `private_brand_type_error` runtime diagnostic helper using the existing runtime exception diagnostic substrate.
- Changed backend `PrivateFieldGet` / `PrivateFieldSet` brand guards so non-object receivers, wrong brands, or out-of-range private slots emit `TypeError: Cannot access private member from an object whose class did not declare it` and abort instead of returning `undefined` or silently skipping writes.
- Updated backend regression coverage for lowered private field runtime calls on an ordinary object to require the TypeError diagnostic.
- Updated `docs/14-runtime-abi.md`, `docs/language-reference/javascript-features.md`, and `current-state.md` to record the diagnostic/abort surface and the remaining catchable-TypeError/external-lowering blockers.

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-backend-wasm private_field_runtime_calls -- --nocapture: pass (3 passed)
cargo test -p ts2wasm-cli private: 2 private semantic/lowering tests passed, then 2 existing unsupported-diagnostic code expectation failures in private_class_field_unsupported_forms_report_issue_255 and private_class_delete_backing_key_reports_issue_255 (`UnsupportedRuntimeSubset` vs helper expecting `UnsupportedSyntax`)
cargo nextest run -E 'test(private) or test(class) or test(node_diff)': failed after 63 passed / 3 failed / 160 not run; failures were existing/out-of-slice node_diff cases bigint_builtin_string_conversion_fixture_matches_node_output_under_iwasm, bigint_dynamic_builtin_fixtures_match_node_output_under_iwasm, and abc451_depth8_live_set_fixture_matches_node_output_under_iwasm timeout
mise run update-issue-index -- --check: pass
mise run check issues: pass
```

Remaining blockers:

- The TypeError path is a runtime diagnostic/abort, not catchable ECMAScript exception-object propagation through `try`/`catch`.
- External top-level private access still reports issue-255 diagnostics because parser/frontend/lowering changes are out of this child slice and `crates/frontend/src/` is forbidden.
- Extracted private methods/accessors and broader receiver forms still need issue-255/351 lowering work.

2026-05-01 child resume slice:

- Relaxed instance private field lowering so same-class `other.#field` read/write inside the declaring class lowers to `PrivateFieldGet` / `PrivateFieldSet` with the declaring class brand token and slot index, rather than requiring the receiver expression to be exactly `this`.
- Added Node/iwasm differential coverage for same-class branded receiver reads/writes in `fixtures/core-semantics/private-class-field-same-class-receiver.ts`.
- Added IR lowering coverage proving non-`this` same-class private field receiver access carries brand `1` and slot `0`.
- Updated `docs/14-runtime-abi.md`, `docs/language-reference/javascript-features.md`, and `current-state.md` to record branded same-class receiver lowering and the remaining TypeError/accessor/method blockers.

Validation result:

```text
cargo fmt --all --check: pass
cargo nextest run -E 'test(private) or test(class) or test(node_diff)': 206 passed, 1 failed; only failure abc451_depth8_live_set_fixture_matches_node_output_under_iwasm timed out after 30.514s, matching the assignment's known unrelated residual risk
mise run update-issue-index -- --check: pass
mise run check issues: pass
```

Remaining blockers:

- Brand mismatch still returns `undefined` in the backend guard path instead of throwing a catchable `TypeError`, because compatible JS exception propagation is not implemented for this path.
- Top-level `c.#x` remains a source diagnostic because the private name is outside its declaring class lexical context; private method/accessor external/extracted forms remain issue-255/351 work.

2026-05-01 child slice:

- Added per-class private brand operands to lowered/backend `PrivateFieldGet` and `PrivateFieldSet` calls.
- Packed class-instance private metadata as `brand << 16 | private_slot_count` in the GC reserved word and masked the slot count during GC scanning.
- Added backend regression coverage proving a mismatched brand cannot overwrite the same slot index on a branded class instance.
- Updated `docs/14-runtime-abi.md`, `docs/language-reference/javascript-features.md`, and `current-state.md` to record the current packed brand/slot-count ABI and remaining non-DONE blockers.

Validation result:

```text
cargo test -p ts2wasm-backend-wasm private_field_runtime_calls -- --nocapture: pass (2 passed)
cargo test -p ts2wasm-cli private_field -- --nocapture: pass (2 matched private-field lowering tests passed)
cargo fmt --all --check: pass after cargo fmt --all
cargo nextest run -E 'test(private) or test(class) or test(node_diff)': 205 passed, 1 failed; only failure abc451_depth8_live_set_fixture_matches_node_output_under_iwasm timed out after 30.519s, matching the assignment's known unrelated residual risk
mise run update-issue-index -- --check: pass after mise trust
mise run check issues: pass after mise trust
```

Remaining blockers:

- External private access still reports an issue-255 source diagnostic instead of lowering to runtime brand-check calls.
- Runtime TypeError throwing on brand mismatch remains blocked on compatible JS exception object/throw support for this path.
- Extracted private methods/accessors and broader receiver forms still need issue-255/351 lowering work.

2026-05-01 earlier child slice:

- Added backend private slot-count guards for lowered `PrivateFieldGet` and `PrivateFieldSet`.
- Added backend regression coverage proving lowered private field runtime calls on an ordinary object do not create or read private storage; `PrivateFieldSet(o, 0, 7)` followed by `PrivateFieldGet(o, 0)` now prints `undefined`.
- Updated `docs/14-runtime-abi.md`, `docs/language-reference/javascript-features.md`, and `current-state.md` to record that the reserved heap header word is the ordinary-object private slot count and that full per-class brand IDs / TypeError throwing remain open.

Validation result:

```text
cargo test -p ts2wasm-backend-wasm private_field_runtime_calls_do_not_create_slots_on_plain_objects -- --nocapture: pass (1 passed)
cargo fmt --all --check: pass
cargo nextest run -E 'test(private) or test(class) or test(node_diff)': 197 passed, 1 failed; only failure abc451_depth8_live_set_fixture_matches_node_output_under_iwasm timed out after 30.443s
mise run update-issue-index && mise run update-issue-index -- --check: pass after mise trust
mise run check issues: pass after mise trust
```

Remaining blockers:

- This is not full ECMAScript brand semantics: two classes with the same private slot index still cannot be distinguished without a per-class brand token in lowered private access IR.
- Runtime TypeError throwing on brand mismatch remains blocked on compatible JS exception object/throw support for this path.
- External private access still reports an issue-255 source diagnostic instead of lowering to runtime brand-check calls.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- Runtime TypeError throwing requires JS exception object support, which may not be fully implemented

2026-05-01 catchable brand TypeError slice:

- Reused the issue-396 runtime exception substrate for `private_brand_type_error`.
- Private field get/set brand mismatches now keep the uncaught diagnostic/abort
  behavior without a handler, and raise a catchable TypeError-like object when a
  supported `try/catch` is active.
- Added backend regression coverage proving lowered private field calls can
  branch to the catch body and continue after the try/catch.
- Added Node/iwasm differential coverage for same-class external receiver
  brand mismatch inside the declaring class:
  `fixtures/core-semantics/private-class-field-external-receiver-catch.ts`.

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-backend-wasm private_field_runtime_calls -- --nocapture: pass (4 passed)
cargo test -p ts2wasm-cli private -- --nocapture: pass (4 private node_diff tests plus private parser/lowering filtered tests)
cargo nextest run -E 'test(private) or test(class) or test(node_diff)': fail after 64 passed / 3 failed / 167 not run; failures are existing/out-of-slice bigint_builtin_string_conversion_fixture_matches_node_output_under_iwasm, bigint_dynamic_builtin_fixtures_match_node_output_under_iwasm, and abc451_depth8_live_set_fixture_matches_node_output_under_iwasm timeout
mise run update-issue-index -- --check && mise run check issues: pass
```

Remaining blockers:

- Issue 351 remains open: subclass/no-inherited-brand fixture coverage and
  broader extracted/external private method/accessor brand-check forms are not
  complete in this slice.

2026-05-01 subclass brand coverage slice:

- Added Node/iwasm differential coverage for the no-inherited-brand case:
  `fixtures/core-semantics/private-class-derived-no-inherited-brand.ts`.
- The fixture proves a `Derived` same-class private field read takes the success
  path on a `Derived` instance but throws through the catchable private-brand
  TypeError path when the receiver is only a `Base` instance, so subclassing
  does not grant the derived class private brand to base-class instances.

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-cli private -- --nocapture: pass
cargo test -p ts2wasm-backend-wasm private_field_runtime_calls -- --nocapture: pass
```

Remaining blockers:

- Broader extracted/external private method/accessor brand-check forms still
  need a separate lowering/runtime slice. In particular,
  method-only/accessor-only classes need an explicit brand without private field
  slots, and assignment/call evaluation order needs to be preserved before
  replacing the current issue-255 diagnostics.
