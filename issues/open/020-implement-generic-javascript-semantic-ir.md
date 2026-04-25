# Implement generic JavaScript semantic IR

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 020
**Type**: feature
**Area**: ir/semantics
**Priority**: P1
**Depends on**: 019
**Orchestration class**: implementation-ready

Problem: Generic JavaScript semantic IR is not implemented. Current IR is minimal and tied to specific lowering patterns. docs/04 specifies IR should have JS semantics instructions.

Scope:

- Design IR with JS semantics instructions (truthiness, `===`, `+`, etc.).
- Implement IR lowering from TypeScript AST.
- Add validation passes for IR invariants.
- Document IR contracts in docs/13-ir-contracts.md.
- Add fixtures for IR-level semantics.

Acceptance Criteria:

- [ ] IR design includes JS semantics instructions.
- [ ] TypeScript AST lowers to semantic IR correctly.
- [ ] IR validation passes catch contract violations.
- [ ] IR contracts are documented in docs/13-ir-contracts.md.
- [ ] Node differential test passes for IR-level fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/ir-test.ts -o /tmp/ir-test.wasm
```
