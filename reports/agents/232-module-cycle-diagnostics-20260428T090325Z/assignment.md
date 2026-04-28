# Assignment: 232 module cycle diagnostics slice

- Run ID: `232-module-cycle-diagnostics-20260428T090325Z`
- Branch: `agent/232-module-cycle-diagnostics-20260428T090325Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-232-module-cycle-diagnostics-20260428T090325Z`
- Issue: `issues/open/232-resolve-local-relative-es-module-graph.md`
- Slice: continue issue 232 by making cycle behavior explicit and covered.

## Coordination

You are not alone in the codebase. Other child agents are working in separate worktrees, including issue 060 coverage and issue 052 JSON. Do not revert, overwrite, or depend on their unmerged edits. Stay within this worktree and this branch.

## Scope

- Read the current `crates/compiler/src/module_graph.rs` implementation and issue 232.
- Add focused tests and, if needed, the smallest implementation adjustment so static local module cycles are represented safely and deterministically, or diagnosed explicitly with issue-232.
- Prefer representing cycles safely if the existing graph builder already supports it; do not implement module execution/lowering.
- Add issue 232 progress evidence with remaining close blockers.

## Allowed Files

- `crates/compiler/src/module_graph.rs`
- `crates/compiler/src/lib.rs` only if needed for test exposure
- `crates/cli/tests/m9_modules.rs` only for a small external regression if needed
- `fixtures/module-system/**` only for cycle diagnostics fixtures if needed
- `current-state.md` only if current facts change
- `issues/open/232-resolve-local-relative-es-module-graph.md`
- `reports/runs/232-module-cycle-diagnostics-20260428T090325Z/**`
- `reports/agents/232-module-cycle-diagnostics-20260428T090325Z/assignment.md`

## Forbidden Files

- `crates/backend-wasm/**`
- `crates/runtime-abi/**`
- `docs/**`
- Unrelated issue files
- Export binding lowering or module execution semantics

## Required Validation

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler module_graph
cargo nextest run -p ts2wasm-compiler
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager discord-report --run-id 232-module-cycle-diagnostics-20260428T090325Z
```

If Discord reporting fails because `DISCORD_WEBHOOK_URL` is absent, save the deferred payload/error under the run directory and continue.

## Completion Protocol

- Commit validated progress on this branch.
- Do not merge to parent.
- End with exactly one line:

```text
PARENT_EVENT: PROGRESS issue=232 branch=agent/232-module-cycle-diagnostics-20260428T090325Z commit=<hash> validation="<short evidence>" report=reports/runs/232-module-cycle-diagnostics-20260428T090325Z/cycle_report.md merge_request=yes
```
