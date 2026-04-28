# Child Assignment: issue 228 logical assignment index/computed continuation

- Child id: `228-logical-assignment-index-20260428T044424Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-228-logical-assignment-index-20260428T044424Z`
- Branch: `agent/228-logical-assignment-index-20260428T044424Z`
- Assigned issues: `228`
- Issue order: `228`

## Required first checks

Run `pwd`, `git status --short --branch`, and confirm you are in the assigned worktree and branch before editing. You are not alone in the codebase; do not revert or overwrite edits from other worktrees.

## Scope

Continue issue 228 after identifier and static-member logical assignment progress. Target one narrow continuation:

- computed/index logical assignment for a simple identifier object and literal/index expression if single-evaluation semantics can be preserved; or
- precise `issue-228` diagnostics plus regression coverage for the unsupported computed/index forms if implementation is unsafe in this cycle.

Preserve short-circuiting and do not evaluate RHS on skipped branches. Do not claim support for non-identifier receiver temporaries such as `getObj()[key] ||= rhs()` unless you implement explicit single-evaluation temporaries.

## Allowed files

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `issues/open/228-implement-logical-assignment-operators.md`
- `current-state.md` only if facts change
- `reports/agents/228-logical-assignment-index-20260428T044424Z/`
- `reports/runs/228-logical-assignment-index-20260428T044424Z/`

## Forbidden files

- `docs/`
- unrelated issue files
- parent worktree files outside this worktree

## Expected validation

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(logical_assignment)'`
- direct Node/iwasm differential command for any added logical-assignment fixture
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

Run broader `cargo nextest run` only if you move issue 228 to done or touch shared assignment behavior.

## Reporting

Commit validated work. Send Discord report if configured; otherwise save deferred payload/error artifacts under `reports/runs/228-logical-assignment-index-20260428T044424Z/`.

End with exactly one parent event line:

```text
PARENT_EVENT: DONE issue=228 branch=agent/228-logical-assignment-index-20260428T044424Z commit=<hash> merge_request=yes
PARENT_EVENT: PROGRESS issue=228 branch=agent/228-logical-assignment-index-20260428T044424Z commit=<hash> merge_request=no
PARENT_EVENT: BLOCKED issue=228 branch=agent/228-logical-assignment-index-20260428T044424Z commit=<hash-or-none> reason=<short-reason>
```
