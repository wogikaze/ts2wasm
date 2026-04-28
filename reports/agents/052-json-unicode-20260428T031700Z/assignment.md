# Assignment: issue 052 JSON unicode escape slice

- Agent ID: 052-json-unicode-20260428T031700Z
- Worktree: /home/wogikaze/wgkz/ts2wasm-052-json-unicode-20260428T031700Z
- Branch: agent/052-json-unicode-20260428T031700Z
- Issue: 052 (`issues/open/052-implement-json.md`)

## Goal

Implement one validated JSON continuation slice. Preferred slice: `JSON.parse`
support for `\uXXXX` escapes when they map to the current runtime's supported
single-byte/ASCII string representation. If full unicode handling is too broad,
implement the smallest safe escape-validation or ASCII unicode escape slice that
produces Node/iwasm differential progress.

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

Full `cargo nextest run` is required only for DONE. Focused validated PROGRESS is acceptable.

## Reporting

Save a cycle report under `reports/runs/<timestamp>-052-json-unicode/`.
If Discord webhook is unavailable, save a deferred payload and continue.
End with one `PARENT_EVENT` line.
