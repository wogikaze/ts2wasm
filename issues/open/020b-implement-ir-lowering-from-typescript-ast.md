# Implement IR lowering from TypeScript AST

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 020b
**Type**: feature
**Area**: ir/semantics
**Priority**: P1
**Depends on**: 020a
**Orchestration class**: implementation-ready

Problem: IR design is complete in 020a but IR lowering from TypeScript AST is not implemented.

Scope:

- Implement IR lowering from TypeScript AST.
- Lower TypeScript AST to semantic IR correctly.
- Add fixtures for IR-level semantics.

Out of scope:

- IR design (see 020a)
- Validation passes (see 020c)

Acceptance Criteria:

- [ ] TypeScript AST lowers to semantic IR correctly.
- [ ] Fixtures demonstrate IR-level semantics.
- [ ] Lowering matches IR design from 020a.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/ir-test.ts -o /tmp/ir-test.wasm
```
