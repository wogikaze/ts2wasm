# Add IR validation passes and document contracts

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
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

- [ ] IR validation passes catch contract violations.
- [ ] IR contracts are documented in docs/13-ir-contracts.md.
- [ ] Validation passes are integrated into build pipeline.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
grep -A 20 "IR contracts" docs/13-ir-contracts.md
```
