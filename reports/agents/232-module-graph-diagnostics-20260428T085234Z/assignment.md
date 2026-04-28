# Assignment: 232 module graph diagnostics slice

- Run ID: `232-module-graph-diagnostics-20260428T085234Z`
- Branch: `agent/232-module-graph-diagnostics-20260428T085234Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-232-module-graph-diagnostics-20260428T085234Z`
- Issue: `issues/open/232-resolve-local-relative-es-module-graph.md`
- Slice: start issue 232 with a narrow compiler/frontend module graph diagnostic slice.

## Coordination

You are not alone in the codebase. Other child agents may work in separate worktrees, including issue 060 coverage. Do not revert, overwrite, or depend on their unmerged edits. Stay within this worktree and this branch.

## Scope

- Read issue 232 and the module parser output from issue 231.
- Implement the smallest safe compiler-side step toward a module graph:
  - detect static module declarations in the entry file,
  - reject bare/non-local specifiers with an issue-linked diagnostic,
  - reject missing local relative module files with a source diagnostic that points to the importing declaration/specifier span.
- If deterministic graph construction is already locally straightforward, add the minimal internal representation and tests for entry + one reachable local relative module, but do not implement export binding lowering or backend execution.
- Keep parsed module declarations stopped before lowering/emission until issue 233.
- Record PROGRESS; do not close issue 232 unless every acceptance criterion is satisfied and full close validation passes.

## Allowed Files

- `crates/compiler/src/**`
- `crates/frontend/src/**` only for span/accessor helpers needed by compiler diagnostics
- `crates/ir/src/**` only if a lightweight module graph representation is unavoidable
- `crates/cli/tests/**`
- `fixtures/module-system/**`
- `current-state.md` only if current facts change
- `issues/open/232-resolve-local-relative-es-module-graph.md`
- `reports/runs/232-module-graph-diagnostics-20260428T085234Z/**`
- `reports/agents/232-module-graph-diagnostics-20260428T085234Z/assignment.md`

## Forbidden Files

- `crates/backend-wasm/**`
- `crates/runtime-abi/**`
- `docs/**`
- Unrelated issue files
- Broad module execution semantics; those are issue 233/234

## Required Validation

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler
cargo nextest run -p ts2wasm-cli module
cargo run -q -p ts2wasm-cli -- dump fixtures/module-system/static-entry.ts --ast --resolved
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager discord-report --run-id 232-module-graph-diagnostics-20260428T085234Z
```

If a required fixture such as `static-entry.ts` does not exist, create the minimal fixture set under `fixtures/module-system/` or document why a different existing fixture was used. If Discord reporting fails because `DISCORD_WEBHOOK_URL` is absent, save the deferred payload/error under the run directory and continue.

## Completion Protocol

- Commit validated progress on this branch.
- Do not merge to parent.
- End with exactly one line:

```text
PARENT_EVENT: PROGRESS issue=232 branch=agent/232-module-graph-diagnostics-20260428T085234Z commit=<hash> validation="<short evidence>" report=reports/runs/232-module-graph-diagnostics-20260428T085234Z/cycle_report.md merge_request=yes
```
