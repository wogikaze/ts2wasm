# Design JavaScript semantic IR

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
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

- [ ] IR design includes JS semantics instructions.
- [ ] IR instruction set is documented.
- [ ] Design decisions are justified.

Validation:

```sh
cargo fmt --all --check
grep -A 30 "IR design" docs/13-ir-contracts.md
```
