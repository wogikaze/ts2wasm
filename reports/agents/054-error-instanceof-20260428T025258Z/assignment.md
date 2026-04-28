# Assignment: issue 054 Error instanceof slice

Agent id: 054-error-instanceof-20260428T025258Z
Worktree: /home/wogikaze/wgkz/ts2wasm-054-error-instanceof-20260428T025258Z
Branch: agent/054-error-instanceof-20260428T025258Z
Issue: 054, issues/open/054-implement-error-types.md

## Scope

Implement one validated continuation slice for Error types. Preferred slice is Error prototype identity, `instanceof Error`, and subclass `instanceof` behavior for already-supported Error constructors. If that is too broad, implement the smallest safe observable property/prototype regression. Do not attempt full `.stack` unless it is clearly small and Node-compatible.

Allowed files:

- crates/backend-wasm/src/**
- crates/ir/src/** only if needed for Error/instanceof representation
- crates/frontend/src/** only if needed for existing syntax support
- crates/cli/tests/**
- fixtures/builtins-and-io/**
- issues/open/054-implement-error-types.md
- reports/agents/**
- reports/runs/**

Forbidden files:

- docs/**
- unrelated issue files
- coverage artifacts/scripts unless strictly required by issue 054

## Validation Plan

1. Reproduce a narrow pre-change gap for Error prototype or `instanceof` behavior if feasible.
2. Add or update focused fixture coverage under `fixtures/builtins-and-io/`.
3. Collect direct Node versus build/iwasm evidence for any new fixture.
4. Run focused validation:
   - `cargo nextest run -E 'test(error)'`
   - `cargo nextest run -p ts2wasm-cli error`
5. Run required gates:
   - `cargo fmt --all --check`
   - `scripts/manager check-issue-health`
   - `scripts/manager check-agent-state`
6. Run full `cargo nextest run` only if claiming DONE; otherwise record focused validation for PROGRESS.

## Reporting Plan

Write a cycle report under `reports/runs/<timestamp>-054-error-instanceof/` with reproduction, implementation notes, validation commands, outputs or log paths, and remaining gaps. If Discord webhook is unavailable, save a deferred payload in the run report directory and continue.

## Merge Protocol

Commit only validated progress on this branch. Leave issue 054 open for PROGRESS, or move/close only if every acceptance criterion and full validation requirement is satisfied. End the cycle with exactly one `PARENT_EVENT` line including status, issue id, branch, commit hash, and merge request intent.
