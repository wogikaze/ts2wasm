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

- [ ] `crates/frontend` is a real workspace member.
- [ ] Parser-related code moves out of `crates/cli`.
- [ ] Existing tests pass.
- [ ] Public API remains minimal.

Current slice evidence:

- `crates/frontend/src/lexer.rs` owns `Lexer`.
- `crates/frontend/src/parser.rs` owns `Parser`.
- `crates/compiler/src/lib.rs` calls frontend APIs and is below the 2000-line architecture warning threshold.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
```

Completion evidence:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-28

command: cargo nextest run --no-fail-fast
result: PASS (194 passed, 4 skipped)
date: 2026-04-28
```

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/010-extract-frontend-module-from-crates-cli.md` before this move
- `issues/open/010-extract-frontend-module-from-crates-cli.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
