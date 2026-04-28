# Child Assignment: issue 050 Date valueOf continuation

- Child id: `050-date-valueof-20260428T045453Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-050-date-valueof-20260428T045453Z`
- Branch: `agent/050-date-valueof-20260428T045453Z`
- Assigned issues: `050`
- Issue order: `050`

## Required first checks

Run `pwd`, `git status --short --branch`, and confirm you are in this worktree and branch before editing. You are not alone in the codebase; do not revert or overwrite edits from other worktrees.

## Scope

Continue issue 050 with a deterministic Date-only slice. Preferred target:

- implement `Date.prototype.valueOf()` for deterministic `new Date(<epoch-ms integer>)` receivers by reusing the existing `getTime()` representation;
- add Node/iwasm differential fixture coverage for `valueOf()` on `0`, positive, and negative integer epochs.

Do not add live host time imports. Keep `Date.now()` and no-arg `new Date()` behind the existing auditable time capability diagnostics.

## Allowed files

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/builtins-and-io/`
- `issues/open/050-implement-date.md`
- `reports/runs/050-date-valueof-20260428T045453Z/`

## Expected validation

- `cargo fmt --all --check`
- targeted Date nextest or `cargo nextest run -p ts2wasm-cli date`
- direct Node/iwasm evidence for any added Date fixture
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

End with exactly one `PARENT_EVENT:` line.
