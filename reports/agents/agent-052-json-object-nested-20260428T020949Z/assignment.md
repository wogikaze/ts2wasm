# Assignment: issue 052 JSON nested object/value slice

Parent branch: `master`
Base: `334fb90`
Worktree: `/home/wogikaze/wgkz/ts2wasm-052-json-object-nested-20260428T020949Z`
Branch: `agent/052-json-object-nested-20260428T020949Z`
Issue: `issues/open/052-implement-json.md`

## Scope

Implement one narrow JSON continuation after nested arrays.

Preferred target:

- `JSON.parse` object values containing arrays or nested objects, with a Node/iwasm differential fixture.
- Keep invalid JSON handling precise; do not silently accept malformed input.

Expected paths:

- `crates/backend-wasm/src/`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/builtins-and-io/`
- `issues/open/052-implement-json.md`
- `reports/agents/agent-052-json-object-nested-20260428T020949Z/`
- `reports/runs/<timestamp>-052-json-object-nested/`

## Required validation

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(json)'`
- `cargo nextest run -p ts2wasm-cli json`
- direct `node <fixture>` and `cargo run -p ts2wasm-cli -- build <fixture> -o /tmp/<name>.wasm && iwasm /tmp/<name>.wasm`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

Run full `cargo nextest run` if the runtime parser changes are broad. If skipped, state why.

## Completion contract

Commit all validated work. If blocked, commit evidence/report instead of leaving the branch dirty.

Final response must include exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=052 branch=agent/052-json-object-nested-20260428T020949Z commit=<sha> merge_request=<yes|no>`
