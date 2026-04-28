# Child Assignment: 060-coverage-ramp17000-20260428T120149Z

- Parent cycle: autonomous multi-worktree compiler development
- Worktree: `/home/wogikaze/wgkz/ts2wasm-060-coverage-ramp17000-20260428T120149Z`
- Branch: `agent/060-coverage-ramp17000-20260428T120149Z`
- Assigned issues: `060`
- Issue order: `060`

## Scope

Continue issue 060 reference-backed classification by expanding the stored test262 coverage window from 16000 to 17000.

Primary target:

- Run detail coverage for test262 limit 17000.
- If new `unknown-unsupported` entries appear, inspect their reference paths/source metadata, classify them with a precise feature label, and create or update follow-up issues when no existing issue covers them.
- If no new unknowns appear, refresh artifacts and record the evidence.
- Keep the stored JSON artifact at `limit=17000`, `blocked=0` if rerun can clear transient blocks, and `unknown-unsupported=0` unless genuinely unclassifiable cases remain with evidence.
- Do not implement feature support in this branch.

## Allowed Files

- `issues/open/060-investigate-unknown-unsupported-cases.md`
- `issues/open/` only for new reference-backed follow-up issues
- `issues/index.md`
- `current-state.md`
- `artifacts/coverage/results/test262.json`
- `artifacts/coverage/reference-coverage-matrix.md`
- `scripts/lib/feature-labels.sh`
- `scripts/run/reference-coverage.py`
- `reports/agents/060-coverage-ramp17000-20260428T120149Z/`
- `reports/runs/060-coverage-ramp17000-20260428T120149Z/`

## Forbidden Files

- `crates/`
- `fixtures/`
- `docs/`
- Any feature implementation files
- Any files owned by other active branches unless required for merge conflict resolution inside this worktree

## Expected Validation

Use the repository reference root unless missing data requires the established temporary TypeScript checkout fallback for tsc. For this assignment, the required test262 commands are:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 17000 --detail
tmp=$(mktemp artifacts/coverage/results/test262.json.tmp.XXXXXX); TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 17000 --json > "$tmp"; mv "$tmp" artifacts/coverage/results/test262.json
scripts/manager update-coverage-matrix
scripts/manager update-coverage-matrix --check
bash -n scripts/lib/feature-labels.sh
python -m py_compile scripts/run/reference-coverage.py
scripts/manager update-issue-index
scripts/manager update-issue-index --check
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager check-repo-smoke
```

`scripts/manager nextest` is required if this branch changes anything outside coverage classification, issue files, reports, and scripts.

## Reporting

- Write `reports/runs/060-coverage-ramp17000-20260428T120149Z/cycle_report.md`.
- Write a schema-valid `reports/runs/060-coverage-ramp17000-20260428T120149Z/test_report.json`.
- Attempt `scripts/manager discord-report --run-id 060-coverage-ramp17000-20260428T120149Z`; if webhook configuration is absent or fails, commit deferred payload/error evidence and continue.
- Commit validated work on the assigned branch.

End with exactly one parent event line:

```text
PARENT_EVENT: PROGRESS issue=060 branch=agent/060-coverage-ramp17000-20260428T120149Z commit=<hash> merge_request=yes
PARENT_EVENT: BLOCKED issue=060 branch=agent/060-coverage-ramp17000-20260428T120149Z commit=<hash-or-none> reason=<short-reason>
```
