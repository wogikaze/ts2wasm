# Extract frontend module from crates/cli (audit reopened #010)

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-28
**ID**: 010
**Type**: refactor
**Area**: frontend
**Priority**: P2
**Depends on**: 003,004
**Orchestration class**: design-ready

Problem: `crates/cli/src/lib.rs` previously mixed lexer, parser, AST, span, validation, and build pipeline code. `crates/frontend/` now owns AST/span/diagnostic/token plus lexer/parser implementation; remaining closure is validation evidence for the broader suite and final issue-state cleanup.

Scope:

- Define frontend crate public boundary.
- Move lexer/parser/AST/span first.
- Keep IR/lowering/backend in place.
- Preserve diagnostics and span contracts.
- Avoid behavior changes.

Acceptance Criteria:

- [x] `crates/frontend` is a real workspace member.
- [x] Parser-related code moves out of `crates/cli`.
- [x] Existing tests pass.
- [x] Public API remains minimal.

Current slice evidence:

- `crates/frontend/src/lexer.rs` owns `Lexer`.
- `crates/frontend/src/parser.rs` owns `Parser`.
- `crates/compiler/src/lib.rs` calls frontend APIs and is below the 2000-line architecture warning threshold.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
```

## Completion evidence

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-28

command: cargo nextest run --no-fail-fast
result: PASS (194 passed, 4 skipped)
date: 2026-04-28
```

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

