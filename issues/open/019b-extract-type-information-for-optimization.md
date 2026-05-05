# Extract type information for optimization hints (audit reopened #019b)

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-28
**Completed**: 2026-04-28
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

Completion evidence:

```text
command: node scripts/check/typescript-oracle.js fixtures/basics-types/optimization-hints.ts
result: PASS (parameter/binding hints and number-add-fast-path candidate emitted)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-frontend
result: PASS (9 passed)
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/optimization-hints.ts -o /tmp/optimization-hints.wasm
result: PASS
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/types.ts -o /tmp/types.wasm
result: PASS
date: 2026-04-28
```

Remaining risks:

- The hints are available to compiler consumers but are not yet consumed by backend optimization passes.

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/019b-extract-type-information-for-optimization.md` before this move
- `issues/open/019b-extract-type-information-for-optimization.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
