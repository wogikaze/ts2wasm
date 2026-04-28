# Assignment: issue 229 legacy octal escape handling

Parent branch: `master`
Base: `334fb90`
Worktree: `/home/wogikaze/wgkz/ts2wasm-229-legacy-octal-20260428T020949Z`
Branch: `agent/229-legacy-octal-20260428T020949Z`
Issue: `issues/open/229-implement-legacy-octal-escape-handling.md`

## Scope

Implement or precisely diagnose one legacy octal escape sequence slice for template literals.

Primary goal:

- Use the reference-backed strict/non-strict legacy octal escape cases as evidence.
- Either support the selected subset or replace generic unsupported output with precise issue-linked diagnostics.
- Preserve existing template literal interpolation behavior.

Expected paths:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `issues/open/229-implement-legacy-octal-escape-handling.md`
- `reports/agents/agent-229-legacy-octal-20260428T020949Z/`
- `reports/runs/<timestamp>-229-legacy-octal/`

Avoid broad template literal refactors unless required for this slice.

## Required validation

- `cargo fmt --all --check`
- focused nextest filters for template / octal / parser
- direct `node <fixture>` and build/iwasm if a runtime fixture is added
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

If feasible, run:

- `TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --path-filter legacy-octal-escape-sequence --limit 750 --detail`

## Completion contract

Commit all validated work or a precise blocker/progress report. Do not leave the branch dirty.

Final response must include exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=229 branch=agent/229-legacy-octal-20260428T020949Z commit=<sha> merge_request=<yes|no>`
