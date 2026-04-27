# Implement generic JavaScript semantic IR

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-28
**Completed**: 2026-04-28
**ID**: 020
**Type**: feature
**Area**: ir/semantics
**Priority**: P1
**Depends on**: 019
**Orchestration class**: implementation-ready

Problem: Generic JavaScript semantic IR is not implemented. Current IR is minimal and tied to specific lowering patterns. docs/04 specifies IR should have JS semantics instructions.

Scope:

This is a parent issue coordinating IR work. Sub-issues:
- 020a: Design IR with JS semantics instructions (done)
- 020b: Implement IR lowering from TypeScript AST (done)
- 020c: Add validation passes and document contracts (done)

Out of scope:

- Design, implementation, and validation are tracked in sub-issues.

Acceptance Criteria:

- [x] 020a (design) is complete.
- [x] 020b (implementation) is complete.
- [x] 020c (validation) is complete.
- [x] Node differential test passes for IR-level fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/ir-test.ts -o /tmp/ir-test.wasm
```

Completion evidence:

```text
command: cargo nextest run -p ts2wasm-ir
result: PASS (13 passed)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm
result: PASS
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/ir-test.ts -o /tmp/ir-test.wasm
result: PASS
date: 2026-04-28

command: cargo nextest run
result: PASS (211 passed, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- Semantic HIR is validated and available as an initial compiler-side IR, but backend still consumes `LoweredProgram`; broader backend consumption should remain incremental.
