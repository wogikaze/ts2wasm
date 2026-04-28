# Child Assignment: 060 coverage ramp to 15000

Child run id: `060-coverage-ramp15000-20260428T100717Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-060-coverage-ramp15000-20260428T100717Z`
Branch: `agent/060-coverage-ramp15000-20260428T100717Z`
Parent branch at assignment: `master` @ `74565f1`

You are not alone in this repository. Other agents are active in separate worktrees; do not revert or overwrite changes made by others, do not touch parent `master`, and stay within this assignment.

## Assigned Issue List

1. `issues/open/060-investigate-unknown-unsupported-cases.md`

## Objective

Continue the reference-backed issue 060 coverage ramp from stored test262 limit 14000 to limit 15000. Do not stop if the detail run finds a blocked case; classify `unknown-unsupported` evidence, generate follow-up issues if needed, and keep validated artifacts consistent.

## Required Work

1. Run a detail classification ramp:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 15000 --detail
```

1. If `unknown-unsupported` is nonzero, inspect the detailed output and either:
   - add a precise classifier if it maps to an existing issue/category, or
   - generate a new reference-backed issue using the repo issue workflow and update `issues/index.md`.
2. Refresh the stored JSON artifact atomically:

```sh
tmp=$(mktemp artifacts/coverage/results/test262.json.tmp.XXXXXX); TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 15000 --json > "$tmp"; mv "$tmp" artifacts/coverage/results/test262.json
```

1. Run `scripts/manager update-coverage-matrix`.
2. Update `current-state.md` and issue 060 progress evidence only with facts that actually changed.

## Allowed Files

- `artifacts/coverage/results/test262.json`
- `artifacts/coverage/reference-coverage-matrix.md`
- `issues/open/060-investigate-unknown-unsupported-cases.md`
- `issues/open/` and `issues/index.md` only if new unknown-backed issues are required
- `current-state.md`
- `scripts/run/` or classifier code only if a precise existing feature label is missing and evidence justifies it
- `reports/agents/060-coverage-ramp15000-20260428T100717Z/`
- `reports/runs/060-coverage-ramp15000-20260428T100717Z/`

## Forbidden Files

- `docs/`
- Compiler/runtime implementation files unless classifier logic truly requires a script change
- Parent branch or any other agent worktree

## Required Validation

```sh
scripts/manager update-coverage-matrix --check
scripts/manager check-issue-health
scripts/manager check-agent-state
```

Also parse `artifacts/coverage/results/test262.json` and record `executed`, `build_pass`, `semantic_pass`, `unsupported`, `blocked`, and whether `unknown-unsupported` remains.

## Reporting

- Write `reports/runs/060-coverage-ramp15000-20260428T100717Z/cycle_report.md`.
- Attempt `scripts/manager discord-report --run-id 060-coverage-ramp15000-20260428T100717Z`; if unavailable, commit deferred payload/error artifacts.
- Commit all validated useful work.
- Merge latest parent `master` before final event.
- End with exactly one parent event line:

```text
PARENT_EVENT: PROGRESS issue=060 branch=agent/060-coverage-ramp15000-20260428T100717Z commit=<hash> validation="<summary>" report=reports/runs/060-coverage-ramp15000-20260428T100717Z/cycle_report.md merge_request=yes
```
