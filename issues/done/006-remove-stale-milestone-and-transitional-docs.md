# Remove stale milestone and transitional docs

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Completed**: 2026-04-26
**ID**: 006
**Type**: cleanup
**Area**: docs
**Priority**: P0
**Depends on**: 002,003
**Orchestration class**: docs-ready

Problem: Several docs appear to mix stale milestone notes, transitional manifest schema, and current implementation claims. This creates handoff risk.

Scope:

- [x] Rewrite or delete stale M/P0 notes that no longer represent current state.
- [x] Make docs/11 the canonical manifest schema source.
- [x] Replace transitional manifest schema in docs/09.
- [x] Update docs/13 priority list to match current issues.
- [x] Keep future tasks in issues, not scattered TODO sections.

Acceptance Criteria:

- [x] Docs do not simultaneously claim a feature is both missing and implemented.
- [x] Canonical schema is not duplicated inconsistently.
- [x] Future work is represented in issues.
- [x] Remaining TODO/deferred mentions are intentional and explainable.

Validation:

```sh
grep -RIn "transitional\|未完\|TODO\|deferred\|wrong current-state path" docs README.md issues
```

## Completion evidence

**Validation results:**

```text
command: grep -RIn "transitional\|未完\|TODO\|deferred\|wrong current-state path" docs README.md issues
result: Only intentional mentions remain (in coding-standard.md gate guidance, issue templates, done issue notes, and README.md guidelines)
date: 2026-04-26
```

**Implementation:**
- Removed transitional manifest schema section from `docs/09-security-and-capability-model.md`
- Updated manifest CLI output section to reference canonical schema in `docs/11`
- Updated priority list in `docs/12-coding-standard.md` to reflect current issue state
- Marked P0 items as done (capability manifest output, manifest import verification, docs cleanup)
- Added P1 items (reference coverage hardening, host-deny E2E manifest)
- Added P2 items (frontend module extraction, warning-clean tree)

**Remaining TODO/deferred mentions are intentional:**
- `docs/12-coding-standard.md:1149`: Gate guidance to ignore deferred items
- `issues/done/012.md`: Historical note about transitional schema dependency
- `issues/done/002.md`: Historical problem statement about transitional schema
- `issues/README.md`: Guidelines against TODO lists in docs
- `issues/templates/issue.md`: Template instruction
- `issues/open/006.md`: This issue itself (being closed)

