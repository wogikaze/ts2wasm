# Add IR validation passes and document contracts (audit reopened #020c)

**Status**: open
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

- [ ] IR validation passes catch contract violations.
- [ ] IR contracts are documented in docs/13-ir-contracts.md.
- [ ] Validation passes are integrated into build pipeline.

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

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/020c-add-ir-validation-passes-and-document-contracts.md` before this move
- `issues/open/020c-add-ir-validation-passes-and-document-contracts.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
