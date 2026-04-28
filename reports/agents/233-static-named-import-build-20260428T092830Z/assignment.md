# Assignment: 233 static named import build slice

- Run ID: `233-static-named-import-build-20260428T092830Z`
- Branch: `agent/233-static-named-import-build-20260428T092830Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-233-static-named-import-build-20260428T092830Z`
- Issue: `issues/open/233-emit-static-es-module-bindings.md`
- Slice: start issue 233 with the smallest static named export/import build path.

## Coordination

You are not alone in the codebase. Another child is running issue 060 coverage. Do not revert, overwrite, or depend on unmerged edits from other worktrees. Stay within this worktree and this branch.

## Scope

- Read issues 233, 231, and 232 close evidence.
- Use the compiler module graph API from issue 232.
- Implement the smallest safe path where `fixtures/module-system/static-entry.ts` with `import { value } from "./static-entry-source";` and `export const value = 1;` in the source module can build to WASM without the old issue-055 static named import diagnostic.
- Prefer a build-path/progress slice over full semantic parity if runtime execution is too broad.
- Keep unsupported diagnostics for default import/export, namespace imports/re-exports, export default, and dynamic import unless explicitly implemented by this slice.
- Add regression coverage proving:
  - simple named export/import program builds,
  - existing CommonJS module-cache fixture still builds,
  - module helper/link-plan behavior is not unconditionally enabled if a focused check exists.

## Allowed Files

- `crates/compiler/src/**`
- `crates/ir/src/**`
- `crates/backend-wasm/src/**`
- `crates/cli/tests/**`
- `fixtures/module-system/**`
- `current-state.md`
- `issues/open/233-emit-static-es-module-bindings.md`
- `reports/runs/233-static-named-import-build-20260428T092830Z/**`
- `reports/agents/233-static-named-import-build-20260428T092830Z/assignment.md`

## Forbidden Files

- `crates/runtime-abi/**` unless a layout constant is strictly required and justified
- `docs/**`
- Unrelated issue files
- Broad live-binding semantics or namespace/default module semantics

## Required Validation

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-backend-wasm
cargo nextest run -p ts2wasm-cli module
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-esm.wasm
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager discord-report --run-id 233-static-named-import-build-20260428T092830Z
```

If Discord reporting fails because `DISCORD_WEBHOOK_URL` is absent, save the deferred payload/error under the run directory and continue.

## Completion Protocol

- Commit validated progress on this branch.
- Do not merge to parent.
- End with exactly one line:

```text
PARENT_EVENT: PROGRESS issue=233 branch=agent/233-static-named-import-build-20260428T092830Z commit=<hash> validation="<short evidence>" report=reports/runs/233-static-named-import-build-20260428T092830Z/cycle_report.md merge_request=yes
```
