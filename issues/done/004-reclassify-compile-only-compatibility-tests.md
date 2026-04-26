# Reclassify compile-only compatibility tests

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Completed**: 2026-04-26
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

Close:

- Date: 2026-04-26
- Evidence:
  - Renamed all `*_compiles()` test functions to `*_build_smoke()` in m7_control_flow.rs, m8_oop_classes.rs, m9_modules.rs, m10_node_apis.rs
  - Added documentation clarifying build smoke tests only check compilation, not runtime semantics
  - Updated official_corpora.rs to distinguish build_smoke (target: wasm32-wasi-build) from semantic_diff (target: wasm32-wasi)
  - Updated current-state.md with test classification section documenting build_smoke vs semantic_diff
  - Renamed infrastructure test functions to avoid "compiles" naming
  - All tests pass: cargo nextest run (185 passed, 4 skipped)
  - Format check passes: cargo fmt --all --check
  - grep -R "compiles" crates/cli/tests now only returns infrastructure fixture existence tests (not compilation tests)

Validation:

```sh
cargo fmt --all --check
cargo nextest run
grep -R "compiles" crates/cli/tests
```
