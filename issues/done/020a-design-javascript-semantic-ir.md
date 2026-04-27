# Design JavaScript semantic IR

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-28
**Completed**: 2026-04-28
**ID**: 020a
**Type**: feature
**Area**: ir/semantics
**Priority**: P1
**Depends on**: 019
**Orchestration class**: design-ready

Problem: Generic JavaScript semantic IR is not designed. Current IR is minimal and tied to specific lowering patterns. docs/04 specifies IR should have JS semantics instructions.

Scope:

- Design IR with JS semantics instructions (truthiness, `===`, `+`, etc.).
- Define IR instruction set and semantics.
- Document IR design decisions.

Out of scope:

- Implementation of IR lowering (see 020b)
- Validation passes (see 020c)

Acceptance Criteria:

- [x] IR design includes JS semantics instructions.
- [x] IR instruction set is documented.
- [x] Design decisions are justified.

Validation:

```sh
cargo fmt --all --check
grep -A 30 "IR design" docs/13-ir-contracts.md
```

Completion evidence:

```text
command: grep -A 30 "IR design" docs/13-ir-contracts.md
result: PASS (semantic instruction set and design decisions documented)
date: 2026-04-28

command: cargo nextest run --no-fail-fast
result: PASS (204 passed, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- Rust enum implementation and lowering are tracked by issues 020b and 020c.
