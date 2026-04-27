# Implement generic JavaScript semantic IR

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-28
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
- 020b: Implement IR lowering from TypeScript AST (implementation-ready, depends on 020a)
- 020c: Add validation passes and document contracts (implementation-ready, depends on 020b)

Out of scope:

- Design, implementation, and validation are tracked in sub-issues.

Acceptance Criteria:

- [x] 020a (design) is complete.
- [ ] 020b (implementation) is complete.
- [ ] 020c (validation) is complete.
- [ ] Node differential test passes for IR-level fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/ir-test.ts -o /tmp/ir-test.wasm
```
