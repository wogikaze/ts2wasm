# Extract type information for optimization hints

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 019b
**Type**: feature
**Area**: frontend
**Priority**: P1
**Depends on**: 019a
**Orchestration class**: implementation-ready

Problem: TypeScript compiler API is integrated in 019a but type information is not yet extracted for optimization hints.

Scope:

- Extract type information from tsc for optimization hints.
- Add fixtures for type-based optimization candidates.
- Identify optimization opportunities from type information.

Out of scope:

- Basic TypeScript compiler API integration (see 019a)

Acceptance Criteria:

- [ ] Type information is available for optimization hints.
- [ ] Type-based optimization candidates are identified.
- [ ] Fixtures demonstrate type-based optimization opportunities.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/types.ts -o /tmp/types.wasm
```
