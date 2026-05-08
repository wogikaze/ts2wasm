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
completed: 2026-04-29
---

## Summary

Strict warning handling now reports clippy and check warnings as errors. The repo must remove the current warning debt so non-test262 gates can pass under the stricter contract.

## Problem

`mise run clippy` now runs with `-D warnings` and fails on existing clippy diagnostics. `mise run check-architecture-rules` now reports oversized source files as `ERROR` and fails on `crates/frontend/src/parser.rs` and `crates/ir/src/lowered.rs`.

## Desired final state

All non-test262 local gates pass with warning-as-error behavior. Agents should see `ERROR` for blocking diagnostics and no warning-only success state.

## Scope

In scope:

- [x] Remove or explicitly justify current clippy diagnostics without weakening lint signal.
- [x] Split or otherwise resolve oversized `crates/frontend/src/parser.rs`.
- [x] Split or otherwise resolve oversized `crates/ir/src/lowered.rs`.
- [x] Keep `mise run clippy` strict with `-D warnings`.
- [x] Keep check/gate scripts reporting blocking diagnostics as `ERROR`.

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

- [x] `mise run clippy` passes.
- [x] `mise run check-architecture-rules` passes.
- [x] `mise run gate-fast` passes.
- [x] `mise run gate-all` passes or records only environment-specific missing tools as separate blockers.
- [x] No check/gate script emits `WARN` or `warning:` for blocking diagnostics.

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

- [x] verified current: `docs/17-windows-development.md` documents `clippy` as `cargo clippy --all-targets -- -D warnings` and lists `gate-fast` / `gate-all`.
- [x] verified current: `.agents/skills/scripts-workflow/SKILL.md` documents `gate-all`, `RUSTFLAGS=-D warnings`, and strict clippy.
- [x] verified current: `.agents/skills/scripts-workflow/SKILL-ja.md` documents `gate-all`, `RUSTFLAGS=-D warnings`, and strict clippy.

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Prefer small mechanical cleanup commits. Avoid broad parser or lowering rewrites unless needed to split ownership cleanly.

This issue is mandatory gate debt. If it is too large for one child cycle, do not mark it blocked solely for size. Break it down and continue:

1. overview: list current clippy and architecture-rule failures.
2. file structure design: split ownership for parser, lowering, backend, runtime ABI, and tests.
3. code design: choose mechanical moves or narrow lint fixes that preserve behavior.
4. implementation: land one internally consistent cleanup slice with validation.
5. repeat until `mise run gate-fast` and `mise run gate-all` pass or only environment-specific blockers remain.

## Completion evidence

Completed on 2026-04-29.

Commits:

- `c7f8cb3` issue-238: split oversized parser and lowered files
- parent merge commit: see repository history after integration

Validation result:

```text
command: mise run fmt
result: pass
date: 2026-04-29

command: mise run clippy
result: pass
date: 2026-04-29

command: mise run check-architecture-rules
result: pass; check_architecture_rules: OK
date: 2026-04-29

command: mise run gate-fast
result: pass; gate: OK
date: 2026-04-29

command: cargo nextest run
result: pass; 416 tests run, 416 passed, 4 skipped
date: 2026-04-29

command: mise run gate-all
result: pass; HARNESS BASELINE PASSED
date: 2026-04-29

command: mise run check issues
result: pass; issues/index.md queue OK; check_issue_health: OK
date: 2026-04-29
```

Remaining risks:

- none

## Progress evidence

2026-04-29 child-238:

- Split oversized parser and lowered IR implementation into focused subfiles while keeping the existing module/API surface:
  - `crates/frontend/src/parser.rs`: 28 lines
  - parser subfiles: largest subfile 1395 lines
  - `crates/ir/src/lowered.rs`: 4 lines
  - lowered subfiles: largest subfile 1299 lines
- `mise run fmt`: pass.
- `mise run clippy`: pass.
- `mise run check-architecture-rules`: pass; no oversized-file errors remain.
- `cargo nextest run`: pass, 414 passed, 4 skipped.
- `mise run gate-all`: code/toolchain/custom harness portions pass, including `cargo nextest (RUSTFLAGS=-D warnings)` and `mise run check architecture`; final aggregate result still fails because `mise run gate-fast`/`mise run check issues` report missing historical ignored report artifacts referenced by unrelated issue files:
  - `issues/open/052-implement-json.md`: missing historical report artifact paths.
  - `issues/open/228-implement-logical-assignment-operators.md`: missing historical report artifact path.

Parent re-validation on 2026-04-29 used the parent checkout with local report artifacts present; `mise run gate-fast`, `cargo nextest run`, `mise run gate-all`, and `mise run check issues` all passed, so the earlier child-worktree report-artifact blocker no longer applies.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/238-make-strict-warning-gates-pass.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
