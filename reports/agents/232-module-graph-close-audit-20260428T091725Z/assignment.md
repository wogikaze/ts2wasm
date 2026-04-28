# Assignment: 232 module graph close audit / contract slice

- Run ID: `232-module-graph-close-audit-20260428T091725Z`
- Branch: `agent/232-module-graph-close-audit-20260428T091725Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-232-module-graph-close-audit-20260428T091725Z`
- Issue: `issues/open/232-resolve-local-relative-es-module-graph.md`
- Slice: audit whether issue 232 can close after graph diagnostics and cycle coverage; if not closeable, make the smallest contract/progress update needed.

## Coordination

You are not alone in the codebase. Another child is running issue 060 coverage. Do not revert, overwrite, or depend on unmerged edits from other worktrees. Stay within this worktree and this branch.

## Scope

- Read issue 232 acceptance criteria and all progress evidence.
- Verify whether the current compiler module graph implementation satisfies:
  - reachable local relative modules exactly once,
  - deterministic ordering,
  - missing relative module diagnostics,
  - bare specifier diagnostics,
  - cycle behavior,
  - preservation or exposure of module IDs and paths for downstream work.
- If all criteria are satisfied, close issue 232 with full close workflow and required validation.
- If one criterion is still missing, implement only the smallest compiler-facing contract exposure or record an exact blocker; do not implement issue 233 binding/lowering.

## Allowed Files

- `crates/compiler/src/module_graph.rs`
- `crates/compiler/src/lib.rs`
- `crates/compiler/src/dump.rs` only if needed for graph inspection evidence
- `current-state.md`
- `issues/open/232-resolve-local-relative-es-module-graph.md`
- `issues/done/232-resolve-local-relative-es-module-graph.md`
- `issues/index.md`
- `reports/runs/232-module-graph-close-audit-20260428T091725Z/**`
- `reports/agents/232-module-graph-close-audit-20260428T091725Z/assignment.md`

## Forbidden Files

- `crates/backend-wasm/**`
- `crates/runtime-abi/**`
- `docs/**`
- Unrelated issue files
- Export binding lowering or module execution semantics

## Required Validation

Always run:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler module_graph
cargo nextest run -p ts2wasm-compiler
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager discord-report --run-id 232-module-graph-close-audit-20260428T091725Z
```

If closing issue 232, also run:

```sh
scripts/manager update-issue-index
scripts/manager update-issue-index --check
scripts/manager check-issue-index
cargo nextest run
```

If Discord reporting fails because `DISCORD_WEBHOOK_URL` is absent, save the deferred payload/error under the run directory and continue.

## Completion Protocol

- Commit validated DONE or PROGRESS on this branch.
- Do not merge to parent.
- End with exactly one line:

```text
PARENT_EVENT: DONE issue=232 branch=agent/232-module-graph-close-audit-20260428T091725Z commit=<hash> validation="<short evidence>" report=reports/runs/232-module-graph-close-audit-20260428T091725Z/cycle_report.md merge_request=yes
```

or:

```text
PARENT_EVENT: PROGRESS issue=232 branch=agent/232-module-graph-close-audit-20260428T091725Z commit=<hash> validation="<short evidence>" report=reports/runs/232-module-graph-close-audit-20260428T091725Z/cycle_report.md merge_request=yes
```
