# Assignment: issue 054 Error stack slice

- Agent ID: 054-error-stack-20260428T034000Z
- Worktree: /home/wogikaze/wgkz/ts2wasm-054-error-stack-20260428T034000Z
- Branch: agent/054-error-stack-20260428T034000Z
- Issue: 054 (`issues/open/054-implement-error-types.md`)

## Goal

Implement one validated Error continuation slice. Preferred slice: minimal observable
`.stack` behavior for supported Error constructors if it can be made Node-compatible
enough for a narrow fixture. If actual stack strings are too broad, implement a precise
issue-linked diagnostic or a smaller property/prototype regression with Node/iwasm evidence.

## Boundaries

Allowed files:

- `crates/backend-wasm/src/**`
- `crates/ir/src/**` only if needed
- `crates/cli/tests/**`
- `fixtures/builtins-and-io/**`
- `issues/open/054-implement-error-types.md`
- `reports/agents/**`
- `reports/runs/**`

Forbidden files:

- `docs/**`
- unrelated issue files
- coverage artifacts/scripts unless strictly required by issue 054

## Required Validation

- Reproduce a narrow pre-change gap if feasible.
- `cargo fmt --all --check`
- `cargo nextest run -E 'test(error)'`
- `cargo nextest run -p ts2wasm-cli error`
- Direct Node vs build/iwasm evidence for any new fixture
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

Full `cargo nextest run` is required only for DONE. Focused validated PROGRESS is acceptable.

End with one `PARENT_EVENT` line.
