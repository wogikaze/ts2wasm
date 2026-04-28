# Assignment: 233 named import alias and diagnostics slice

- Run ID: `233-named-import-alias-diagnostics-20260428T093927Z`
- Branch: `agent/233-named-import-alias-diagnostics-20260428T093927Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-233-named-import-alias-diagnostics-20260428T093927Z`
- Issue: `issues/open/233-emit-static-es-module-bindings.md`
- Slice: continue issue 233 by hardening the current graph-backed static named import build rewrite for alias imports and missing export diagnostics.

## Coordination

You are not alone in the codebase. Another child is running issue 060 coverage. Do not revert, overwrite, or depend on unmerged edits from other worktrees. Stay within this worktree and this branch.

## Scope

- Build on the current issue-233 literal export build rewrite.
- Add coverage for `import { value as renamed } from "./static-entry-source";` backed by literal `export const value = 1;`.
- Add coverage for a missing named export from an existing local module, verifying the diagnostic is issue-233 and points at the imported name span where feasible.
- Do not implement full module binding IR, dependency-order initialization, live bindings, namespace/default semantics, or issue 234 execution parity.
- Update issue 233 progress evidence.

## Allowed Files

- `crates/compiler/src/lib.rs`
- `crates/cli/tests/**`
- `fixtures/module-system/**`
- `current-state.md` only if current facts change
- `issues/open/233-emit-static-es-module-bindings.md`
- `reports/runs/233-named-import-alias-diagnostics-20260428T093927Z/**`
- `reports/agents/233-named-import-alias-diagnostics-20260428T093927Z/assignment.md`

## Forbidden Files

- `crates/backend-wasm/**` unless a focused link-plan test is truly needed
- `crates/runtime-abi/**`
- `docs/**`
- Unrelated issue files

## Required Validation

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler
cargo nextest run -p ts2wasm-cli module
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/<new-alias-fixture>.ts -o /tmp/ts2wasm-esm-alias.wasm
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager discord-report --run-id 233-named-import-alias-diagnostics-20260428T093927Z
```

If Discord reporting fails because `DISCORD_WEBHOOK_URL` is absent, save the deferred payload/error under the run directory and continue.

## Completion Protocol

- Commit validated progress on this branch.
- Do not merge to parent.
- End with exactly one line:

```text
PARENT_EVENT: PROGRESS issue=233 branch=agent/233-named-import-alias-diagnostics-20260428T093927Z commit=<hash> validation="<short evidence>" report=reports/runs/233-named-import-alias-diagnostics-20260428T093927Z/cycle_report.md merge_request=yes
```
