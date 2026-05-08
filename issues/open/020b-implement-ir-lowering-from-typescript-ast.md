# Implement IR lowering from TypeScript AST (audit reopened #020b)

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-28
**Completed**: 2026-04-28
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

- [x] TypeScript AST lowers to semantic IR correctly.
- [x] Fixtures demonstrate IR-level semantics.
- [x] Lowering matches IR design from 020a.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/ir-test.ts -o /tmp/ir-test.wasm
```

## Completion evidence

```text
command: cargo nextest run -p ts2wasm-ir
result: PASS (9 passed)
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/ir-test.ts -o /tmp/ir-test.wasm
result: PASS
date: 2026-04-28

command: cargo nextest run --no-fail-fast
result: PASS (207 passed, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- HIR validation passes and backend consumption are tracked by issue 020c and follow-up implementation work.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

