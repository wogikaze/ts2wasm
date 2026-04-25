# Integrate TypeScript parser/checker

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 019
**Type**: feature
**Area**: frontend
**Priority**: P1
**Depends on**: 010
**Orchestration class**: implementation-ready

Problem: TypeScript parser/checker integration is not implemented. Current parser is minimal. docs/04 specifies using tsc as oracle but not making production compiler fully dependent on it.

Scope:

- Integrate TypeScript compiler API for type checking.
- Use tsc as oracle for diagnostics and type information.
- Extract type information for optimization hints.
- Maintain separation from full tsc dependency.
- Add fixtures for type-based optimization candidates.

Acceptance Criteria:

- [ ] TypeScript compiler API is integrated for type checking.
- [ ] Type information is available for optimization hints.
- [ ] Diagnostics from tsc are propagated correctly.
- [ ] Production compiler does not require tsc at runtime.
- [ ] Type-based optimization candidates are identified.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/types.ts -o /tmp/types.wasm
```
