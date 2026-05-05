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
- `issues/done/006-remove-stale-milestone-and-transitional-docs.md` (moved from open/ per close evidence)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Completion evidence

The following changes were made in the previous implementation to address the acceptance criteria:

### Changes applied (committed in prior work on this issue)

1. **`current-state.md`**: Top priority list updated to reference `issues/index.md` ready queue instead of a hardcoded list of issues. See "Next Priority Steps" section.

2. **`docs/06-testing-and-coverage.md`**: Path reference typo fixed from `current-state` to `current-state.md`.

3. **`docs/09-security-and-capability-model.md`**: Transitional manifest schema (legacy `target/imports/capabilities/runtime` shape) replaced with canonical capability manifest example from `docs/11-shared-definitions.md`. Added `clock.realtime` field to the manifest example for consistency with the canonical schema.

4. **Future work**: No scattered TODO sections remain in the modified docs. Future work references point to `issues/index.md` and individual issue links.

### Acceptance verification

- [x] Docs do not simultaneously claim a feature is both missing and implemented.
  - No contradictory claims found. docs/09 references canonical schema from docs/11.
- [x] Canonical schema is not duplicated inconsistently.
  - docs/09 manifest example matches docs/11 schema (both include `clock`).
- [x] Future work is represented in issues.
  - No scattered future-TODO sections; all future work references point to `issues/index.md`.
- [x] Remaining TODO/deferred mentions are intentional and explainable.
  - Remaining matches are inside test fixture strings, guidelines, or documented scope decisions in done issues.

### Validation command output

```sh
rg -n "transitional|未完|TODO|deferred|wrong current-state path" docs/ README.md issues/
```

Result: all matches are either:
- The issue file itself (references to "transitional" in its title and problem statement)
- Inside test fixture string data (issues/open/593, issues/done/128, issues/done/507)
- Guideline/template text (issues/README.md, issues/templates/issue.md)
- Legitimate scope-deferred decisions in done issues
- Feature names containing "deferred" (e.g., deferredLookupTypeResolution, issues/open/1943)
- The word "deferred" as a document term (docs/12-coding-standard.md gate rule, docs/19-parallel-development.md Discord send rule)

No stale `transitional` schema references remain in docs/. The `docs/12-coding-standard.md` reference to "stale milestone and transitional docs cleanup (done)" correctly reflects that the cleanup task was performed.
