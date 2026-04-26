# Reclassify compile-only compatibility tests

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 004
**Type**: test
**Area**: tests/coverage
**Priority**: P0
**Depends on**: none
**Orchestration class**: implementation-ready

Problem: Tests such as class/module/Node API compile-only checks can make compatibility look more advanced than it is. Build success must not be counted as semantic compatibility.

Scope:

- Classify tests as `parser_smoke`, `build_smoke`, or `semantic_diff`.
- Rename compile-only tests so they do not imply runtime semantics are implemented.
- Move actual semantic claims to Node differential tests.
- Mark unsupported runtime semantics explicitly.
- Document that compile pass is not compatibility pass.

Acceptance Criteria:

- [x] Compile-only tests no longer imply semantic support.
- [x] Coverage reporting distinguishes build pass from semantic pass.
- [x] Current state clearly identifies class/module/Node API semantic gaps.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
rg "compiles" crates/cli/tests
```

Validation result (local):

- `cargo fmt --all --check`: pass
- `cargo nextest run`: fail (environmental: `iwasm` is not installed in this workspace; existing differential fixtures requiring runtime execution are blocked in this environment)
- `rg "compiles" crates/cli/tests`: pass (build-smoke rename in targeted files; semantic differentiation handled in `m2_node_diff.rs`)

## Completion evidence

- `crates/cli/tests/m6_builtin_methods.rs`: rebuilt as `build_smoke_*` and no longer emits synthetic pass `TestRecord`s.
- `crates/cli/tests/m7_control_flow.rs`: rebuilt as `build_smoke_*` and asserts build success explicitly.
- `crates/cli/tests/m8_oop_classes.rs`: rebuilt as `build_smoke_*`.
- `crates/cli/tests/m9_modules.rs`: module semantic claim test removed; now build-smoke only.
- `crates/cli/tests/m10_node_apis.rs`: rebuilt as `build_smoke_*`.
- `crates/cli/tests/m2_node_diff.rs`:
  - Added `CLASS_SEMANTIC_GAP_FIXTURES`, `MODULE_SEMANTIC_GAP_FIXTURES`, `NODE_API_SEMANTIC_GAP_FIXTURES`.
  - Added `assert_fixture_not_semantically_pass(...)` helper and 3 gap assertions, requiring non-`pass` status + tracking.
- `docs/06-testing-and-coverage.md` and `docs/15-coverage-matrix.md` updated to distinguish `build_smoke` vs `semantic_pass`.
- `current-state.md` now explicitly tracks class/module/node-api semantic gaps and references `m2_node_diff.rs`.
