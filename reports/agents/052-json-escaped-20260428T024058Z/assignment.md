# Assignment: issue 052 JSON continuation

Agent id: `052-json-escaped-20260428T024058Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-052-json-escaped-20260428T024058Z`
Branch: `agent/052-json-escaped-20260428T024058Z`
Issue: `052` (`issues/open/052-implement-json.md`)

## Scope

Implement one validated JSON continuation slice for issue 052. Preferred target is the smallest safe `JSON.parse` progress slice covering escaped string support and/or number decimal/exponent support. Do not attempt broad JSON spec completion.

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

1. Reproduce the selected JSON gap with direct Node vs `cargo run -p ts2wasm-cli -- build ... && iwasm ...` evidence.
2. Add or update the narrow fixture and any CLI JSON differential coverage needed for the selected slice.
3. Run focused validation:
   - `cargo fmt --all --check`
   - `cargo nextest run -E 'test(json)'`
   - `cargo nextest run -p ts2wasm-cli json`
   - direct Node vs build/iwasm evidence for each new fixture
   - `scripts/manager check-issue-health`
   - `scripts/manager check-agent-state`
4. Run full `cargo nextest run` only if claiming `DONE`; otherwise report `PROGRESS` with focused validation.

## Reporting Plan

Write the cycle report under `reports/runs/<timestamp>-052-json-escaped/` with commands, outcomes, direct differential evidence, and remaining issue 052 gaps. If Discord webhook reporting is unavailable, save a deferred payload in the same run directory and continue.

## Merge Protocol

Commit only validated progress on branch `agent/052-json-escaped-20260428T024058Z`. Do not merge locally. End the cycle with exactly one `PARENT_EVENT` line including status, issue id, branch, commit hash, and `merge_request=no` unless the parent requests otherwise.
