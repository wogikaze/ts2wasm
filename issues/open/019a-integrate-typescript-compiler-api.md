# Integrate TypeScript compiler API for type checking (audit reopened #019a)

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-28
**Completed**: 2026-04-28
**ID**: 019a
**Type**: feature
**Area**: frontend
**Priority**: P1
**Depends on**: 010
**Orchestration class**: implementation-ready

Problem: TypeScript parser/checker integration is not implemented. Current parser is minimal. docs/04 specifies using tsc as oracle but not making production compiler fully dependent on it.

Scope:

- Integrate TypeScript compiler API for type checking.
- Use tsc as oracle for diagnostics and type information.
- Propagate diagnostics from tsc correctly.
- Maintain separation from full tsc dependency (production compiler does not require tsc at runtime).

Out of scope:

- Type information extraction for optimization hints (see 019b)

Acceptance Criteria:

- [ ] TypeScript compiler API is integrated for type checking.
- [ ] Diagnostics from tsc are propagated correctly.
- [ ] Production compiler does not require tsc at runtime.
- [ ] Basic type checking works for TypeScript fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/types.ts -o /tmp/types.wasm
```

Completion evidence:

```text
command: node scripts/check/typescript-oracle.js fixtures/basics-types/types.ts
result: PASS (ok=true, TypeScript 6.0.3)
date: 2026-04-28

command: node scripts/check/typescript-oracle.js fixtures/basics-types/type-error.ts
result: PASS (TS2322 propagated)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-frontend
result: PASS (5 passed)
date: 2026-04-28

command: cargo nextest run --no-fail-fast
result: PASS (196 passed, 4 skipped)
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- check fixtures/basics-types/types.ts
result: PASS
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/types.ts -o /tmp/types.wasm
result: PASS
date: 2026-04-28

command: temporarily move node_modules aside, then cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/types.ts -o /tmp/types-no-node-modules.wasm
result: PASS
date: 2026-04-28
```

Remaining risks:

- Type information extraction for optimization is intentionally deferred to issue 019b.

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/019a-integrate-typescript-compiler-api.md` before this move
- `issues/open/019a-integrate-typescript-compiler-api.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
