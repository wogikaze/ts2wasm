# Assignment

- Agent ID: agent-052-json-number-20260428T030707Z
- Worktree: /home/wogikaze/wgkz/ts2wasm-052-json-number-20260428T030707Z
- Branch: agent/052-json-number-20260428T030707Z
- Issue: 052 (`issues/open/052-implement-json.md`)
- Goal: implement one validated JSON continuation slice, preferring `JSON.parse` decimal/exponent number support if feasible.
- Started: 2026-04-28

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

## Validation Plan

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(json)'`
- `cargo nextest run -p ts2wasm-cli json`
- Direct Node vs build/iwasm evidence for any new fixture
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

Full `cargo nextest run` is required only if claiming DONE; focused validated PROGRESS is acceptable.
