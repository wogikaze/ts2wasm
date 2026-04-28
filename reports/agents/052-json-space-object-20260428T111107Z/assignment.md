# Assignment: issue 052 JSON stringify object/function space parity

Child run id: `052-json-space-object-20260428T111107Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-052-json-space-object-20260428T111107Z`
Branch: `agent/052-json-space-object-20260428T111107Z`
Assigned issues: `052`
Issue order: `052`

## Objective

Continue issue 052 with a narrow `JSON.stringify(value, null, space)` ignored-value parity slice. Boolean `space` is already supported; this slice should handle one or more remaining non-number/non-string `space` forms that Node ignores, such as object literal or function values, without broad JSON compliance claims.

## Allowed files

- `crates/frontend/src/**` only if function/object fixture parsing requires a narrow existing-path fix
- `crates/ir/src/**`
- `crates/backend-wasm/src/**` only if runtime behavior must change for the narrow slice
- `crates/cli/tests/**`
- `fixtures/builtins-and-io/json-*.ts`
- `issues/open/052-implement-json.md`
- `reports/runs/052-json-space-object-20260428T111107Z/`
- `reports/agents/052-json-space-object-20260428T111107Z/assignment.md`

## Forbidden files

- `artifacts/coverage/**`
- module-system fixtures
- logical-assignment fixtures
- unrelated issues
- docs

## Required workflow

1. Read `.agents/prompts/autonomous-child-worker.md`, `AGENTS.md`, issue 052, and this assignment.
2. Reproduce Node behavior for the chosen `space` cases and the current ts2wasm behavior.
3. Implement the smallest safe path for the chosen ignored `space` forms. Do not weaken diagnostics for unsupported replacer forms.
4. Add Node/iwasm differential fixture coverage under `fixtures/builtins-and-io/`.
5. Run validation:
   - `cargo fmt --all --check`
   - `cargo nextest run -E 'test(json)'`
   - `cargo nextest run -p ts2wasm-cli json`
   - direct `node` fixture command
   - direct `cargo run -q -p ts2wasm-cli -- build ... && iwasm ...`
   - `scripts/manager check-issue-health`
   - `scripts/manager check-agent-state`
6. Run full `cargo nextest run` only if closing issue 052 or changing broad runtime JSON behavior.
7. Write `reports/runs/052-json-space-object-20260428T111107Z/cycle_report.md` and schema-valid `test_report.json`.
8. Attempt `scripts/manager discord-report --run-id 052-json-space-object-20260428T111107Z`; if webhook is unavailable, save payload/error artifacts and continue.
9. Commit all validated changes. Request merge from parent.

## Expected outcome

Use `PROGRESS` unless every issue 052 acceptance criterion is met and the full close workflow is complete.

## Parent event

End with exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=052 branch=agent/052-json-space-object-20260428T111107Z commit=<hash> merge_request=yes`

Use `BLOCKED` with evidence if the selected ignored `space` form depends on unsupported language features.
