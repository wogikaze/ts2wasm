# Assignment: 231 close-readiness audit

- Run ID: `231-close-audit-20260428T083200Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-231-close-audit-20260428T083200Z`
- Branch: `agent/231-close-audit-20260428T083200Z`
- Issue: `issues/open/231-parse-static-es-module-declarations.md`
- Slice: close-readiness audit after parser-only import/export progress.
- Outcome rule: verify every acceptance criterion against current code/tests. If all criteria are met, move issue 231 to `issues/done/`, update completion evidence, regenerate `issues/index.md`, validate, report, and commit. If any criterion is not met, leave it open, record concise progress/blocker evidence or create only a narrow follow-up if needed, validate, report, and commit.

## Allowed files

- `issues/open/231-parse-static-es-module-declarations.md`
- `issues/done/231-parse-static-es-module-declarations.md`
- `issues/index.md`
- `crates/frontend/src/parser.rs` only for minimal parser fixture proof already supported by current AST
- `crates/cli/tests/m9_modules.rs` only for minimal CLI fixture proof already supported by current AST
- `fixtures/module-system/` only for minimal fixture proof already supported by current AST
- `reports/runs/231-close-audit-20260428T083200Z/**`
- `reports/agents/231-close-audit-20260428T083200Z/assignment.md`

## Forbidden files

- backend/runtime implementation
- `docs/**`
- unrelated issue files unless creating a necessary follow-up

## Expected validation

- `cargo fmt --all --check`
- `cargo nextest run -p ts2wasm-frontend`
- relevant `cargo nextest run -p ts2wasm-cli static_*_reports_issue_055` module guard tests
- `scripts/manager update-issue-index`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`
- full `cargo nextest run` if closing issue 231

## Reporting

- Attempt `scripts/manager discord-report --run-id 231-close-audit-20260428T083200Z`.
- If webhook env is absent or reporting fails, save deferred payload/report under the run directory and continue.
- Commit validated progress on the assigned branch only. Do not merge to parent.
