# Cycle Report: 2026-04-26 12:58

## Issue Completed

- **ID**: 004
- **Title**: Reclassify compile-only compatibility tests
- **Type**: test
- **Area**: tests/coverage
- **Priority**: P0

## Summary

Reclassified compile-only tests to explicitly distinguish build success from semantic compatibility. Renamed test functions and added documentation to clarify that build smoke tests only verify compilation (syntax parsing, name resolution, lowering to WASM), not runtime semantics.

## Implementation

### Changes Made

1. **m7_control_flow.rs**: Renamed all `*_compiles()` functions to `*_build_smoke()` with documentation
2. **m8_oop_classes.rs**: Renamed all `*_compiles()` functions to `*_build_smoke()` with documentation
3. **m9_modules.rs**: Renamed `compile_fixture()` to `compile_fixture_build_smoke()`, renamed test functions, renamed semantic test to `*_semantic_diff()`
4. **m10_node_apis.rs**: Renamed all `*_compiles()` functions to `*_build_smoke()` with documentation
5. **official_corpora.rs**: Added test classification documentation, updated `classify_build_case()` to return build_smoke records with explicit reason
6. **test_infrastructure.rs**: Renamed `test_pass_fixture_compiles()` to `test_pass_fixture_exists()` and `test_fail_fixture_compiles()` to `test_fail_fixture_exists()`
7. **current-state.md**: Added "Test classification" section documenting build_smoke vs semantic_diff vs parser_smoke

### Test Classification

- **build_smoke**: Tests that compilation succeeds (syntax parsing, name resolution, lowering to WASM). These do NOT verify runtime semantics.
- **semantic_diff**: Tests that Node.js and iwasm execution produce identical output (differential testing).
- **parser_smoke**: Tests that syntax can be parsed (not yet implemented).

Build pass does NOT imply semantic compatibility.

## Verification

### Commands Run

```bash
cargo fmt --all --check  # PASS
cargo nextest run        # PASS (185 passed, 4 skipped)
grep -R "compiles" crates/cli/tests  # Only returns infrastructure fixture existence tests
```

### Acceptance Criteria

- [x] Compile-only tests no longer imply semantic support.
- [x] Coverage reporting distinguishes build pass from semantic pass.
- [x] Current state clearly identifies class/module/Node API semantic gaps.

## Evidence

- All renamed test functions follow `*_build_smoke()` convention
- Documentation added to all affected test files clarifying build smoke vs semantic diff
- official_corpora.rs now uses target "wasm32-wasi-build" for build smoke tests vs "wasm32-wasi" for semantic tests
- current-state.md documents test classification policy
- grep for "compiles" in tests directory no longer returns compilation test functions

## Commit

- **Hash**: 016fb2a
- **Message**: feat(tests): reclassify compile-only tests as build_smoke

## Next Steps

Ready queue still has P0 issues:

- 003: Verify manifest against emitted WAT imports (depends on 002)
- 013: Implement heap OOM check

Consider selecting issue 013 next (no dependencies).
