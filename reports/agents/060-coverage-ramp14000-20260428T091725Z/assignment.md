# Assignment: 060 coverage ramp to 14000

- Run ID: `060-coverage-ramp14000-20260428T091725Z`
- Branch: `agent/060-coverage-ramp14000-20260428T091725Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-060-coverage-ramp14000-20260428T091725Z`
- Issue: `issues/open/060-investigate-unknown-unsupported-cases.md`
- Slice: continue issue 060 by ramping stored test262 reference coverage from limit 13000 to limit 14000.

## Coordination

You are not alone in the codebase. Another child is auditing issue 232. Do not revert, overwrite, or depend on unmerged edits from other worktrees. Stay within this worktree and this branch.

## Scope

- Do not implement compiler/runtime features.
- Run the test262 limit-14000 coverage ramp and inspect any `unknown-unsupported` entries.
- If zero `unknown-unsupported`, record validated PROGRESS and leave issue 060 open.
- If unknowns appear, classify them with precise existing or new feature labels only when evidence supports it; otherwise create reference-backed follow-up issues.
- Use temp-file output for JSON artifact refresh; never leave `artifacts/coverage/results/test262.json` truncated during a long command.

## Allowed Files

- `artifacts/coverage/results/test262.json`
- `artifacts/coverage/reference-coverage-matrix.md`
- `current-state.md`
- `issues/open/060-investigate-unknown-unsupported-cases.md`
- `issues/open/**` and `issues/index.md` only if follow-up issues are generated
- `scripts/lib/feature-labels.sh` and reference-coverage scripts only if classifier changes are required
- `reports/runs/060-coverage-ramp14000-20260428T091725Z/**`
- `reports/agents/060-coverage-ramp14000-20260428T091725Z/assignment.md`

## Forbidden Files

- Compiler/runtime implementation files unless classifier script changes explicitly require them
- `docs/**`
- Unrelated issue files

## Required Validation

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 14000 --detail
tmp=$(mktemp artifacts/coverage/results/test262.json.tmp.XXXXXX)
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 14000 --json > "$tmp"
mv "$tmp" artifacts/coverage/results/test262.json
scripts/manager update-coverage-matrix
scripts/manager update-coverage-matrix --check
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager discord-report --run-id 060-coverage-ramp14000-20260428T091725Z
```

If Discord reporting fails because `DISCORD_WEBHOOK_URL` is absent, save the deferred payload/error under the run directory and continue.

## Completion Protocol

- Commit validated progress on this branch.
- Do not merge to parent.
- End with exactly one line:

```text
PARENT_EVENT: PROGRESS issue=060 branch=agent/060-coverage-ramp14000-20260428T091725Z commit=<hash> validation="<short evidence>" report=reports/runs/060-coverage-ramp14000-20260428T091725Z/cycle_report.md merge_request=no
```
