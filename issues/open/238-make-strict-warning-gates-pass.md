---
id: 238
title: "Make strict warning gates pass"
type: infra
area: tests
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Strict warning handling now reports clippy and check warnings as errors. The repo must remove the current warning debt so non-test262 gates can pass under the stricter contract.

## Problem

`mise run clippy` now runs with `-D warnings` and fails on existing clippy diagnostics. `mise run check-architecture-rules` now reports oversized source files as `ERROR` and fails on `crates/frontend/src/parser.rs` and `crates/ir/src/lowered.rs`.

## Desired final state

All non-test262 local gates pass with warning-as-error behavior. Agents should see `ERROR` for blocking diagnostics and no warning-only success state.

## Scope

In scope:

- [ ] Remove or explicitly justify current clippy diagnostics without weakening lint signal.
- [ ] Split or otherwise resolve oversized `crates/frontend/src/parser.rs`.
- [ ] Split or otherwise resolve oversized `crates/ir/src/lowered.rs`.
- [ ] Keep `mise run clippy` strict with `-D warnings`.
- [ ] Keep check/gate scripts reporting blocking diagnostics as `ERROR`.

Out of scope:

- test262 semantic coverage improvements.
- Broad compiler feature implementation unrelated to strict gate cleanup.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- `crates/ir/src/lowered.rs`
- `crates/backend-wasm/src/emitter.rs`
- `crates/backend-wasm/src/expr_emit.rs`
- `crates/backend-wasm/src/runtime_core.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/src/stmt_emit.rs`
- `crates/cli/tests/ir_lowering.rs`
- `crates/cli/tests/parser_keywords.rs`
- `crates/cli/tests/parser_ast_structures.rs`
- `crates/runtime-abi/src/layout.rs`
- `mise.toml`
- `scripts/check/architecture-rules.py`

Do not touch:

- `reference/`
- `reports/`

## Acceptance criteria

- [ ] `mise run clippy` passes.
- [ ] `mise run check-architecture-rules` passes.
- [ ] `mise run gate-fast` passes.
- [ ] `mise run gate-all` passes or records only environment-specific missing tools as separate blockers.
- [ ] No check/gate script emits `WARN` or `warning:` for blocking diagnostics.

## Validation

Required commands:

```sh
mise run fmt
mise run clippy
mise run check-architecture-rules
mise run gate-fast
mise run gate-all
```

Impacted commands:

```sh
mise run check
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/17-windows-development.md`
- [ ] updated: `.agents/skills/scripts-workflow/SKILL.md`
- [ ] updated: `.agents/skills/scripts-workflow/SKILL-ja.md`

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Prefer small mechanical cleanup commits. Avoid broad parser or lowering rewrites unless needed to split ownership cleanly.

This issue is mandatory gate debt. If it is too large for one child cycle, do not mark it blocked solely for size. Break it down and continue:

1. overview: list current clippy and architecture-rule failures.
2. file structure design: split ownership for parser, lowering, backend, runtime ABI, and tests.
3. code design: choose mechanical moves or narrow lint fixes that preserve behavior.
4. implementation: land one internally consistent cleanup slice with validation.
5. repeat until `mise run gate-fast` and `mise run gate-all` pass or only environment-specific blockers remain.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
