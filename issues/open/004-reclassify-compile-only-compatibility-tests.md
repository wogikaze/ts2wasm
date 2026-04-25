# Reclassify compile-only compatibility tests

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 004
**Type**: test
**Area**: tests/coverage
**Priority**: P0
**Depends on**: 001
**Orchestration class**: implementation-ready

Problem: Tests such as class/module/Node API compile-only checks can make compatibility look more advanced than it is. Build success must not be counted as semantic compatibility.

Scope:
- Classify tests as `parser_smoke`, `build_smoke`, or `semantic_diff`.
- Rename compile-only tests so they do not imply runtime semantics are implemented.
- Move actual semantic claims to Node differential tests.
- Mark unsupported runtime semantics explicitly.
- Document that compile pass is not compatibility pass.

Acceptance Criteria:
- [ ] Compile-only tests no longer imply semantic support.
- [ ] Coverage reporting distinguishes build pass from semantic pass.
- [ ] Current state clearly identifies class/module/Node API semantic gaps.

Validation:
```sh
cargo fmt --all --check
cargo nextest run
grep -R "compiles" crates/cli/tests
```

