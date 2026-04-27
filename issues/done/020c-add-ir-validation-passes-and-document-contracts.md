# Add IR validation passes and document contracts

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-28
**Completed**: 2026-04-28
**ID**: 020c
**Type**: feature
**Area**: ir/semantics
**Priority**: P1
**Depends on**: 020b
**Orchestration class**: implementation-ready

Problem: IR lowering is implemented in 020b but validation passes and contract documentation are missing.

Scope:

- Add validation passes for IR invariants.
- Document IR contracts in docs/13-ir-contracts.md.
- Ensure validation passes catch contract violations.

Out of scope:

- IR design (see 020a)
- IR lowering implementation (see 020b)

Acceptance Criteria:

- [x] IR validation passes catch contract violations.
- [x] IR contracts are documented in docs/13-ir-contracts.md.
- [x] Validation passes are integrated into build pipeline.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
grep -A 20 "IR contracts" docs/13-ir-contracts.md
```

Completion evidence:

```text
command: cargo nextest run -p ts2wasm-ir
result: PASS (13 passed)
date: 2026-04-28

command: grep -A 20 "IR contracts" docs/13-ir-contracts.md
result: PASS
date: 2026-04-28

command: cargo nextest run
result: PASS (211 passed, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- HIR validation is integrated for the supported HIR subset; unsupported syntax continues through the existing LoweredProgram pipeline until HIR coverage expands.
