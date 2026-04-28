# Assignment: 231 export class guard

- Run ID: `231-export-class-guard-20260428T083349Z`
- Branch: `agent/231-export-class-guard-20260428T083349Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-231-export-class-guard-20260428T083349Z`
- Issue: `issues/open/231-parse-static-es-module-declarations.md`
- Slice: close the blocker found by the 231 close-readiness audit: `export class C {}` currently builds successfully instead of producing an issue-055 unsupported module diagnostic.

## Coordination

You are not alone in the codebase. Other child agents are working in separate worktrees on issue 060 coverage and issue 052 JSON runtime behavior. Do not revert, overwrite, or depend on their unmerged edits. Stay within this worktree and this branch.

## Scope

- Reproduce the `export class C {}` behavior narrowly.
- Implement the smallest parser/frontend and downstream guard change needed so this form is not silently accepted.
- Prefer keeping `export class` issue-linked unsupported for this slice unless full AST support is clearly trivial and all existing issue 231 acceptance remains true.
- Add or update parser and CLI regression coverage proving the behavior.
- Update issue 231 progress evidence with the blocker resolution and remaining close status.

## Allowed Files

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser.rs`
- `crates/frontend/src/diagnostic.rs`
- `crates/cli/tests/m9_modules.rs`
- `fixtures/module-system/**`
- `issues/open/231-parse-static-es-module-declarations.md`
- `reports/runs/231-export-class-guard-20260428T083349Z/**`
- `reports/agents/231-export-class-guard-20260428T083349Z/assignment.md`

## Forbidden Files

- `crates/backend-wasm/**`
- `crates/runtime-abi/**`
- `docs/**`
- Unrelated issue files

## Required Validation

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli static_class_export_reports_issue_055
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager discord-report --run-id 231-export-class-guard-20260428T083349Z
```

If the exact CLI test name differs after implementation, use the precise test added for `export class` and record that in the report. If Discord reporting fails because `DISCORD_WEBHOOK_URL` is absent, save the deferred payload/error under the run directory and continue.

## Completion Protocol

- Commit validated progress on this branch.
- Do not merge to parent.
- End with exactly one line:

```text
PARENT_EVENT: PROGRESS issue=231 branch=agent/231-export-class-guard-20260428T083349Z commit=<hash> validation="<short evidence>" report=reports/runs/231-export-class-guard-20260428T083349Z/cycle_report.md merge_request=yes
```
