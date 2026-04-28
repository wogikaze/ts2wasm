# Assignment: issue 052 JSON array object slice

- Agent ID: 052-json-array-object-20260428T033000Z
- Worktree: /home/wogikaze/wgkz/ts2wasm-052-json-array-object-20260428T033000Z
- Branch: agent/052-json-array-object-20260428T033000Z
- Issue: 052 (`issues/open/052-implement-json.md`)

## Goal

Implement or prove with regression coverage one validated JSON continuation slice.
Preferred slice: explicit `JSON.parse` object-elements-inside-arrays coverage, e.g.
`JSON.parse('[{"a":1},{"b":[2]}]')`, with Node/iwasm differential evidence. If the
runtime already supports it, add regression coverage and issue evidence as PROGRESS.
If a narrow missing runtime change is needed, implement only that slice.

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

Save a cycle report under `reports/runs/<timestamp>-052-json-array-object/`.
If Discord webhook is unavailable, save a deferred payload and continue.
End with one `PARENT_EVENT` line.
