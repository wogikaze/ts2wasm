# Assignment: issue 052 JSON nested parse slice

Parent branch: `master`
Worktree: `/home/wogikaze/wgkz/ts2wasm-052-json-nested-20260428T015517Z`
Branch: `agent/052-json-nested-20260428T015517Z`
Issue: `issues/open/052-implement-json.md`
Base: `466a4bd`

## Scope

Implement one narrow, validated JSON continuation slice. Prefer nested `JSON.parse` support for arrays/objects because issue 052 already supports top-level primitive arrays and flat objects.

Primary goal:

- Support at least one Node-differential fixture that currently fails, such as nested arrays, object values inside arrays, or array values inside objects.
- Keep unsupported JSON forms precise; do not silently parse invalid JSON incorrectly.

Expected paths:

- `crates/backend-wasm/src/`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/builtins-and-io/`
- `issues/open/052-implement-json.md`
- `reports/agents/agent-052-json-nested-20260428T015517Z/`
- `reports/runs/<timestamp>-052-json-nested/`

Avoid docs unless a factual current-state change is required.

## Required validation

Run the narrow gates first:

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(json)'`
- `cargo nextest run -p ts2wasm-cli json`
- direct `cargo run -p ts2wasm-cli -- build <fixture> -o /tmp/<name>.wasm && iwasm /tmp/<name>.wasm`
- matching `node <fixture>` output

Before requesting merge, also run:

- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

Run full `cargo nextest run` if the change touches shared runtime parsing enough to justify it. If full nextest is skipped, state why in the report.

## Completion contract

Commit all validated work on this branch. If blocked, commit a report/evidence artifact instead of leaving the branch dirty.

Final response must include exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=052 branch=agent/052-json-nested-20260428T015517Z commit=<sha> merge_request=<yes|no>`

Use `merge_request=yes` only when parent can safely merge the branch.
