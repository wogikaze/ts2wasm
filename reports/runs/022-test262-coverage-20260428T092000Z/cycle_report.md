# Issue 022 test262 coverage cycle

Date: 2026-04-28
Branch: `agent/022-test262-coverage-20260428T092000Z`
Issue: `issues/open/022-expand-test262-differential-coverage.md`
Outcome: PROGRESS

## Scope

Coverage execution, artifact refresh, matrix sync, and issue evidence only. No compiler implementation, fixtures, or issue 210 files were touched.

## Evidence

- `reference/test262` was missing in this worktree and was restored with a shallow official checkout: `git clone --depth 1 https://github.com/tc39/test262.git reference/test262`.
- `python scripts/manager.py reference-coverage test262 --limit 100 --detail` passed and saved detail output to `reports/agents/agent-022-test262-coverage-20260428T092000Z/reference-coverage-test262-limit100-detail.txt`.
- Limit-100 summary: `denominator=53445`, `executed=100`, `build_pass=0`, `semantic_pass=0`, `fail=0`, `unsupported=100`, `blocked=0`, `skip_with_reason=0`, `semantic_enabled=1`.
- Unsupported breakdown: `UnsupportedSyntax:63`, `UnresolvedName:37`; feature labels: `regexp-literal:47`, `name-resolution:37`, `date:13`, `function:2`, `unknown-unsupported:1`.
- `artifacts/coverage/results/test262.json` was refreshed with the limit-100 JSON output.
- `scripts/manager update-coverage-matrix` regenerated `artifacts/coverage/reference-coverage-matrix.md`.

## Acceptance State

- Gate D executed count is satisfied for test262: `executed=100`.
- Gate E is not satisfied: build-pass remains `0`, semantic-pass remains `0`.
- Build-pass and semantic-pass are tracked separately in both the JSON artifact and generated matrix.

## Next Step

Keep issue 022 open. The sorted test262 ramp is currently blocked by unsupported feature clusters before it reaches runnable semantic-core cases. The next safe work is either to add a harness selection mode for known runnable semantic-core test262 seeds or to implement the dominant unsupported feature slices before continuing the sorted `--limit` ramp for Gate E.
