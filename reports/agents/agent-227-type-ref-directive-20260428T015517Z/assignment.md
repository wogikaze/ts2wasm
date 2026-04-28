# Assignment: issue 227 type-reference directive resolution

Parent branch: `master`
Worktree: `/home/wogikaze/wgkz/ts2wasm-227-type-ref-directive-20260428T015517Z`
Branch: `agent/227-type-ref-directive-20260428T015517Z`
Issue: `issues/open/227-implement-type-reference-directive-resolution.md`
Base: `466a4bd`

## Scope

Implement or precisely diagnose a minimal TypeScript triple-slash `reference types` directive slice.

Primary goal:

- Add regression fixture evidence for a `/// <reference types="..."/>` case from the issue notes.
- Either resolve the supported subset or produce an issue-linked diagnostic that replaces generic `type-directive-resolution` / `unknown-unsupported` output.
- Preserve `skipLibCheck` / `@ts-ignore` behavior where this slice reaches it; otherwise document remaining work.

Expected paths:

- `crates/frontend/src/`
- `crates/compiler/src/`
- `crates/cli/tests/`
- `fixtures/`
- `issues/open/227-implement-type-reference-directive-resolution.md`
- `reports/agents/agent-227-type-ref-directive-20260428T015517Z/`
- `reports/runs/<timestamp>-227-type-ref-directive/`

Avoid touching parser/backend paths outside this issue. If implementation is broader than expected, commit evidence/progress and request parent guidance in the PARENT_EVENT.

## Required validation

Run focused validation relevant to the implementation:

- `cargo fmt --all --check`
- a focused `cargo nextest run` filter for the added directive fixture/test
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

If feasible, also run:

- `TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsgo --limit 120`

If the reference command is too expensive or blocked, record exact evidence.

## Completion contract

Commit all validated work on this branch. If blocked, commit a report/evidence artifact instead of leaving the branch dirty.

Final response must include exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=227 branch=agent/227-type-ref-directive-20260428T015517Z commit=<sha> merge_request=<yes|no>`

Use `merge_request=yes` only when parent can safely merge the branch.
