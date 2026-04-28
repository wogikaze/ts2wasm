# Child Assignment: 237-ishtmldda-host-hook-20260428T120149Z

- Parent cycle: autonomous multi-worktree compiler development
- Worktree: `/home/wogikaze/wgkz/ts2wasm-237-ishtmldda-host-hook-20260428T120149Z`
- Branch: `agent/237-ishtmldda-host-hook-20260428T120149Z`
- Assigned issues: `237`
- Issue order: `237`

## Scope

Complete or make validated progress on issue 237 by extending the precise unsupported-policy coverage for Annex B `[[IsHTMLDDA]]` host/test262 forms.

Primary target:

- Cover the newly classified equality/logical/typeof/if `emulates-undefined` paths from the test262 limit-16000 ramp.
- Ensure unshadowed `$262.IsHTMLDDA` reports an issue-237 diagnostic rather than incidental generic name resolution failure.
- Add focused regression fixture or CLI/reference coverage evidence for the selected unsupported policy.
- Do not implement a broad browser/document.all model unless a tiny, evidence-backed compiler slice naturally falls out.
- Close issue 237 only if all listed acceptance criteria and full validation pass; otherwise record PROGRESS.

## Allowed Files

- `issues/open/237-implement-annexb-ishtmldda-compatibility.md`
- `issues/done/237-implement-annexb-ishtmldda-compatibility.md` only if DONE criteria are fully met
- `issues/index.md`
- `current-state.md` only if observed facts change
- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `reports/agents/237-ishtmldda-host-hook-20260428T120149Z/`
- `reports/runs/237-ishtmldda-host-hook-20260428T120149Z/`

## Forbidden Files

- `docs/`
- Coverage classifier files unless direct reference evidence proves issue 237 classification needs another label
- Any files owned by other active branches unless required for merge conflict resolution inside this worktree

## Expected Validation

Run focused reproduction first, including reference path filters for at least representative equality/logical/typeof/if cases. Then run:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli annexb_ishtmldda
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --path-filter annexB/language/expressions/equals/emulates-undefined.js --path-filter annexB/language/expressions/typeof/emulates-undefined.js --path-filter annexB/language/statements/if/emulated-undefined.js --detail
scripts/manager check-issue-health
scripts/manager check-agent-state
```

If closing issue 237, also run:

```sh
cargo nextest run
scripts/manager check-repo-smoke
```

## Reporting

- Write `reports/runs/237-ishtmldda-host-hook-20260428T120149Z/cycle_report.md`.
- Write a schema-valid `reports/runs/237-ishtmldda-host-hook-20260428T120149Z/test_report.json`.
- Attempt `scripts/manager discord-report --run-id 237-ishtmldda-host-hook-20260428T120149Z`; if webhook configuration is absent or fails, commit deferred payload/error evidence and continue.
- Commit validated work on the assigned branch.

End with exactly one parent event line:

```text
PARENT_EVENT: DONE issue=237 branch=agent/237-ishtmldda-host-hook-20260428T120149Z commit=<hash> merge_request=yes
PARENT_EVENT: PROGRESS issue=237 branch=agent/237-ishtmldda-host-hook-20260428T120149Z commit=<hash> merge_request=yes
PARENT_EVENT: BLOCKED issue=237 branch=agent/237-ishtmldda-host-hook-20260428T120149Z commit=<hash-or-none> reason=<short-reason>
```
