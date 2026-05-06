---
id: 5128
title: "Add semantic parity for static re-export module forms"
type: test
area: ir/compiler
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
status: done
---

## Summary

Static re-export forms currently have build-smoke coverage, but not Node/iwasm differential coverage that proves exported values are observable through an importing entry.

## Problem

Problem: `export * from`, `export { x } from`, and `export * as ns from` fixtures build, but the semantic test suite does not yet prove an entry module can import through those re-export modules and match Node stdout under iwasm.

## Current failure

Existing build-smoke tests:

```sh
cargo nextest run -p ts2wasm-cli static_star_re_export_entry_build_smoke static_named_re_export_from_entry_build_smoke static_namespace_re_export_from_entry_build_smoke
```

Current gap: no focused `m2_node_diff` test imports from a re-exporting module and compares Node/iwasm stdout.

## Desired final state

Re-export forms have semantic parity tests that exercise observable imports through a re-exporting dependency module.

## Scope

In scope:

- [x] Add fixtures or generated Node variants for `export * from "./source"` read through a downstream import.
- [x] Add fixtures or generated Node variants for `export { x } from "./source"` read through a downstream import.
- [x] Add fixtures or generated Node variants for `export * as ns from "./source"` read through a downstream import.
- [x] Preserve existing build-smoke coverage for entry-module re-export forms.

Out of scope:

- Circular module re-export semantics.
- Live binding updates after initial export evaluation.
- Non-literal dependency exports.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`
- `fixtures/module-system/`

Do not touch:

- unrelated runtime/backend code unless the differential tests show an emitter issue

## Acceptance criteria

- [x] Star re-export import-through fixture matches Node stdout under iwasm.
- [x] Named re-export import-through fixture matches Node stdout under iwasm.
- [x] Namespace re-export import-through fixture matches Node stdout under iwasm.
- [x] `cargo nextest run -p ts2wasm-cli module` passes.
- [x] Issue evidence records the exact focused differential command.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli module
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli static_star_re_export_entry_build_smoke static_named_re_export_from_entry_build_smoke static_namespace_re_export_from_entry_build_smoke
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from audit-reopened issue 5010 so local named export evidence can close separately from broader re-export semantic parity.

## Completion evidence

Completed on 2026-05-06.

Commits:

- `762fe17b` compiler: support static re-export parity

Validation result:

```text
command: cargo nextest run -p ts2wasm-cli static_star_re_export_module_import_fixture_matches_node_output_under_iwasm static_named_re_export_module_import_fixture_matches_node_output_under_iwasm static_namespace_re_export_module_import_fixture_matches_node_output_under_iwasm
result: pass (3 tests run: 3 passed, 654 skipped)
date: 2026-05-06

command: cargo nextest run -p ts2wasm-cli module
result: pass (27 tests run: 27 passed, 630 skipped)
date: 2026-05-06

command: cargo nextest run -p ts2wasm-compiler
result: pass (58 tests run: 58 passed, 0 skipped)
date: 2026-05-06

command: cargo fmt --all --check
result: pass
date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

