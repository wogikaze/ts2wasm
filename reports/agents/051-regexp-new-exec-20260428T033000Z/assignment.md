# Assignment: issue 051 RegExp new-exec slice

- Agent ID: 051-regexp-new-exec-20260428T033000Z
- Worktree: /home/wogikaze/wgkz/ts2wasm-051-regexp-new-exec-20260428T033000Z
- Branch: agent/051-regexp-new-exec-20260428T033000Z
- Issue: 051 (`issues/open/051-implement-regexp.md`)

## Goal

Implement one validated RegExp continuation slice. Preferred slice: direct
`new RegExp("plain").exec("...")` support if the current parser/member-access path
rejects it, preserving the existing plain-pattern limitations and issue-051 diagnostics
for unsupported metacharacters. Do not attempt full match-array semantics.

## Boundaries

Allowed files:

- `crates/frontend/src/**`
- `crates/ir/src/**`
- `crates/backend-wasm/src/**`
- `crates/cli/tests/**`
- `fixtures/core-semantics/**`
- `issues/open/051-implement-regexp.md`
- `reports/agents/**`
- `reports/runs/**`

Forbidden files:

- `docs/**`
- unrelated issue files
- coverage artifacts/scripts unless strictly required by issue 051

## Required Validation

- Reproduce the current direct `new RegExp("plain").exec(...)` gap if present.
- `cargo fmt --all --check`
- `cargo nextest run -E 'test(regexp)'`
- `cargo nextest run -p ts2wasm-cli regexp`
- Direct Node vs build/iwasm evidence for any new fixture
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

Full `cargo nextest run` is required only for DONE. Focused validated PROGRESS is acceptable.

## Reporting

Save a cycle report under `reports/runs/<timestamp>-051-regexp-new-exec/`.
If Discord webhook is unavailable, save a deferred payload and continue.
End with one `PARENT_EVENT` line.
