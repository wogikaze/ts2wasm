# Extract frontend module from crates/cli

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 010
**Type**: refactor
**Area**: frontend
**Priority**: P2
**Depends on**: 003,004
**Orchestration class**: design-ready

Problem: `crates/cli/src/lib.rs` appears to mix lexer, parser, AST, span, validation, and build pipeline code. `crates/frontend/` exists but is empty. Extraction is needed, but only after P0 gates are stable.

Scope:
- Define frontend crate public boundary.
- Move lexer/parser/AST/span first.
- Keep IR/lowering/backend in place.
- Preserve diagnostics and span contracts.
- Avoid behavior changes.

Acceptance Criteria:
- [ ] `crates/frontend` is a real workspace member.
- [ ] Parser-related code moves out of `crates/cli`.
- [ ] Existing tests pass.
- [ ] Public API remains minimal.

Validation:
```sh
cargo fmt --all --check
cargo nextest run
```

