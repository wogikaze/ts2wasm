# Assignment: issue 052 JSON validation continuation

Parent branch: `master`
Base: `8988f17`
Worktree: `/home/wogikaze/wgkz/ts2wasm-052-json-validation-20260428T022516Z`
Branch: `agent/052-json-validation-20260428T022516Z`
Issue: `issues/open/052-implement-json.md`

## Scope

Implement one narrow JSON.parse validation or parsing slice.

Preferred targets:

- Reject or precisely diagnose trailing tokens after a valid top-level JSON value.
- Or add support for object elements inside parsed arrays if validation is already correct.

Keep behavior Node-compatible for the selected fixture and do not broaden invalid JSON acceptance.

Expected paths:

- `crates/backend-wasm/src/`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/builtins-and-io/`
- `issues/open/052-implement-json.md`
- `reports/agents/agent-052-json-validation-20260428T022516Z/`
- `reports/runs/<timestamp>-052-json-validation/`

## Required validation

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(json)'`
- `cargo nextest run -p ts2wasm-cli json`
- direct `node <fixture>` and `cargo run -p ts2wasm-cli -- build <fixture> -o /tmp/<name>.wasm && iwasm /tmp/<name>.wasm`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

## Completion contract

Commit validated work or a precise blocker/progress report. Do not leave the branch dirty.

Final response must include exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=052 branch=agent/052-json-validation-20260428T022516Z commit=<sha> merge_request=<yes|no>`
