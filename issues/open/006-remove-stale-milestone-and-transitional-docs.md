# Remove stale milestone and transitional docs (audit reopened #006)

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Completed**: 2026-04-26
**ID**: 006
**Type**: cleanup
**Area**: docs
**Priority**: P0
**Depends on**: 002,003
**Orchestration class**: docs-ready

Problem: Several docs appeared to mix stale milestone notes, transitional manifest schema, and current implementation claims. This created handoff risk.

Scope:

- Rewrite or delete stale M/P0 notes that no longer represent current state.
- Make `docs/11-shared-definitions.md` canonical for manifest schema.
- Replace transitional manifest schema in `docs/09-security-and-capability-model.md`.
- Keep future tasks in issues, not scattered TODO sections.

Acceptance Criteria:

- [ ] Docs do not simultaneously claim a feature is both missing and implemented.
- [ ] Canonical schema is not duplicated inconsistently.
- [ ] Future work is represented in issues.
- [ ] Remaining TODO/deferred mentions are intentional and explainable.

Validation:

```sh
rg -n "transitional|未完|TODO|deferred|wrong current-state path" docs README.md issues
```

Executed command outputs:

- `cargo fmt --all --check`: pass
- `mise run check-issue-health`: pass
- `mise run update-issue-index -- --check`: pass
- `mise run check-fast-gate -- --skip-nextest`: pass
- `mise run nextest`: failed (7/186 failed, 4 skipped, 98 cancelled) due expected environment constraint: `iwasm` / `node` binary not found in test environment for differential/iwasm fixtures.

Notes:
- Updated `current-state.md` top priority list to match `issues/index.md` ready queue.
- Updated `docs/06-testing-and-coverage.md` path reference typo to `current-state.md`.
- Replaced `docs/09-security-and-capability-model.md` transitional manifest section with canonical schema example from `docs/11-shared-definitions.md`.

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/006-remove-stale-milestone-and-transitional-docs.md` before this move
- `issues/open/006-remove-stale-milestone-and-transitional-docs.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
