# Integrate TypeScript parser/checker

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-28
**Completed**: 2026-04-28
**ID**: 019
**Type**: feature
**Area**: frontend
**Priority**: P1
**Depends on**: 010
**Orchestration class**: implementation-ready

Problem: TypeScript parser/checker integration is not implemented. Current parser is minimal. docs/04 specifies using tsc as oracle but not making production compiler fully dependent on it.

Scope:

This is a parent issue coordinating TypeScript integration work. Sub-issues:
- 019a: Integrate TypeScript compiler API for type checking (done)
- 019b: Extract type information for optimization hints (done)

Out of scope:

- Integration details are tracked in sub-issues.

Acceptance Criteria:

- [x] 019a (basic integration) is complete.
- [x] 019b (optimization hints) is complete.
- [x] TypeScript compiler API is integrated.
- [x] Production compiler does not require tsc at runtime.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/types.ts -o /tmp/types.wasm
```

Completion evidence:

```text
command: cargo nextest run -p ts2wasm-frontend
result: PASS (9 passed)
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- check fixtures/basics-types/types.ts
result: PASS
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/types.ts -o /tmp/types.wasm
result: PASS
date: 2026-04-28
```

Remaining risks:

- Type hints are extracted and exposed, but optimization passes do not yet consume them.
