# Phase 3: Implementation (TDD)

## TDD Mode: pragmatic
The Promise WAT runtime and IR routing were already implemented. This item only adds a test fixture and registers the acceptance test — no production code changes needed.

## Changed Files
1. `fixtures/core-semantics/promise-basic.ts` — NEW fixture
2. `crates/cli/tests/common/m2_node_diff_fixture_tests.rs` — Added `promise_basic_matches_node_output`
