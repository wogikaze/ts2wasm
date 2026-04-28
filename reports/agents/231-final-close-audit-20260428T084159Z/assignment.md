# Assignment: 231 final close audit

- Run ID: `231-final-close-audit-20260428T084159Z`
- Branch: `agent/231-final-close-audit-20260428T084159Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-231-final-close-audit-20260428T084159Z`
- Issue: `issues/open/231-parse-static-es-module-declarations.md`
- Slice: audit whether issue 231 can now be closed after the export-class guard merge; close it only if every acceptance criterion is satisfied with evidence.

## Coordination

You are not alone in the codebase. Other child agents are working in separate worktrees on issue 060 coverage and issue 052 JSON runtime behavior. Do not revert, overwrite, or depend on their unmerged edits. Stay within this worktree and this branch.

## Scope

- Read all issue 231 acceptance criteria and progress evidence.
- Verify the parser/static module declaration forms currently implemented and the unsupported forms that must remain issue-linked.
- If all acceptance criteria are satisfied, complete the close workflow:
  - move `issues/open/231-parse-static-es-module-declarations.md` to `issues/done/231-parse-static-es-module-declarations.md`
  - update frontmatter status/path if the local issue format requires it
  - add completion evidence
  - run `scripts/manager update-issue-index`
  - run full `cargo nextest run`
- If any close criterion is still missing, do not close. Record the exact blocker in the issue and cycle report, commit validated PROGRESS, and request parent merge.

## Allowed Files

- `issues/open/231-parse-static-es-module-declarations.md`
- `issues/done/231-parse-static-es-module-declarations.md`
- `issues/index.md`
- `reports/runs/231-final-close-audit-20260428T084159Z/**`
- `reports/agents/231-final-close-audit-20260428T084159Z/assignment.md`

## Forbidden Files

- Compiler/runtime implementation files
- `docs/**`
- Unrelated issue files
- Fixtures/tests, unless a missing close-blocker requires a minimal audit probe fixture; prefer recording BLOCKED instead of implementing in this close audit

## Required Validation

Always run:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli static_named_import_reports_issue_055 static_side_effect_import_reports_issue_055 static_namespace_import_reports_issue_055 static_default_import_reports_issue_055 static_combined_named_import_reports_issue_055 static_combined_namespace_import_reports_issue_055 static_named_export_reports_issue_055 static_re_export_reports_issue_055 static_named_re_export_reports_issue_055 static_namespace_re_export_reports_issue_055 static_declaration_export_reports_issue_055 static_default_export_reports_issue_055 static_class_export_reports_issue_055
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager discord-report --run-id 231-final-close-audit-20260428T084159Z
```

If closing issue 231, also run:

```sh
scripts/manager update-issue-index
scripts/manager update-issue-index --check
scripts/manager check-issue-index
cargo nextest run
```

If Discord reporting fails because `DISCORD_WEBHOOK_URL` is absent, save the deferred payload/error under the run directory and continue.

## Completion Protocol

- Commit validated DONE or PROGRESS on this branch.
- Do not merge to parent.
- End with exactly one line:

```text
PARENT_EVENT: DONE issue=231 branch=agent/231-final-close-audit-20260428T084159Z commit=<hash> validation="<short evidence>" report=reports/runs/231-final-close-audit-20260428T084159Z/cycle_report.md merge_request=yes
```

or:

```text
PARENT_EVENT: PROGRESS issue=231 branch=agent/231-final-close-audit-20260428T084159Z commit=<hash> validation="<short evidence>" report=reports/runs/231-final-close-audit-20260428T084159Z/cycle_report.md merge_request=yes
```
