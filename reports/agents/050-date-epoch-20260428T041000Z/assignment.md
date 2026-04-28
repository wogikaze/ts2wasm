# Assignment: issue 050 Date epoch slice

- Agent ID: 050-date-epoch-20260428T041000Z
- Worktree: /home/wogikaze/wgkz/ts2wasm-050-date-epoch-20260428T041000Z
- Branch: agent/050-date-epoch-20260428T041000Z
- Issue: 050 (`issues/open/050-implement-date.md`)

## Goal

Implement one deterministic Date continuation slice. Preferred slice: `new Date(0)`
and `.getTime()` support with Node/iwasm differential evidence. Do not add live host
time imports for `new Date()` or `Date.now()` unless an auditable capability policy is
already present and validated. Keep issue 050 open unless all criteria are satisfied.

## Boundaries

Allowed files:

- `crates/ir/src/**`
- `crates/backend-wasm/src/**`
- `crates/cli/tests/**`
- `fixtures/builtins-and-io/**`
- `issues/open/050-implement-date.md`
- `reports/agents/**`
- `reports/runs/**`

Forbidden files:

- `docs/**`
- unrelated issue files
- coverage artifacts/scripts unless strictly required by issue 050

## Required Validation

- Reproduce current `new Date(0)` / `.getTime()` gap if feasible.
- `cargo fmt --all --check`
- targeted Date tests you add or update
- direct Node vs build/iwasm evidence for any new fixture
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

Full `cargo nextest run` is required only for DONE. Focused validated PROGRESS is acceptable.
End with one `PARENT_EVENT` line.
