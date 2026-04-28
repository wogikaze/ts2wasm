# Assignment: issue 052 JSON incomplete token validation

- Agent ID: 052-json-incomplete-20260428T035800Z
- Worktree: /home/wogikaze/wgkz/ts2wasm-052-json-incomplete-20260428T035800Z
- Branch: agent/052-json-incomplete-20260428T035800Z
- Issue: 052 (`issues/open/052-implement-json.md`)

## Goal

Implement one validated JSON continuation slice. Preferred slice: stricter
`JSON.parse` incomplete-token validation, such as rejecting unterminated strings,
arrays, objects, or incomplete literals/numbers with Node-aligned failure behavior
for narrow fixtures. Keep scope small and issue 052 open unless all criteria are met.

## Boundaries

Allowed files:

- `crates/backend-wasm/src/**`
- `crates/cli/tests/**`
- `fixtures/builtins-and-io/**`
- `issues/open/052-implement-json.md`
- `reports/agents/**`
- `reports/runs/**`

Forbidden files:

- `docs/**`
- unrelated issue files
- central scripts/artifacts unless strictly required by issue 052

## Required Validation

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(json)'`
- `cargo nextest run -p ts2wasm-cli json`
- Direct Node vs build/iwasm evidence for any new fixture
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

End with one `PARENT_EVENT` line.
