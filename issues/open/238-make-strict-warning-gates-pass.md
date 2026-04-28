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

`scripts/manager clippy` now runs with `-D warnings` and fails on existing clippy diagnostics. `scripts/manager check-architecture-rules` now reports oversized source files as `ERROR` and fails on `crates/frontend/src/parser.rs` and `crates/ir/src/lowered.rs`.

## Desired final state

All non-test262 local gates pass with warning-as-error behavior. Agents should see `ERROR` for blocking diagnostics and no warning-only success state.

## Scope

In scope:

- [ ] Remove or explicitly justify current clippy diagnostics without weakening lint signal.
- [ ] Split or otherwise resolve oversized `crates/frontend/src/parser.rs`.
- [ ] Split or otherwise resolve oversized `crates/ir/src/lowered.rs`.
- [ ] Keep `scripts/manager clippy` strict with `-D warnings`.
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
- `scripts/manager.py`
- `scripts/check/architecture-rules.py`

Do not touch:

- `reference/`
- `reports/`

## Acceptance criteria

- [ ] `scripts/manager clippy` passes.
- [ ] `scripts/manager check-architecture-rules` passes.
- [ ] `scripts/manager check-fast-gate --skip-nextest` passes.
- [ ] `scripts/manager check-harness-installation` passes or records only environment-specific missing tools as separate blockers.
- [ ] No check/gate script emits `WARN` or `warning:` for blocking diagnostics.

## Validation

Required commands:

```sh
scripts/manager fmt
scripts/manager clippy
scripts/manager check-architecture-rules
scripts/manager check-fast-gate --skip-nextest
scripts/manager check-harness-installation
```

Impacted commands:

```sh
scripts/manager check-repo-smoke
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
