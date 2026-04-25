# Remove stale milestone and transitional docs

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 006
**Type**: cleanup
**Area**: docs
**Priority**: P0
**Depends on**: 002,003
**Orchestration class**: docs-ready

Problem: Several docs appear to mix stale milestone notes, transitional manifest schema, and current implementation claims. This creates handoff risk.

Scope:
- Rewrite or delete stale M/P0 notes that no longer represent current state.
- Make docs/11 the canonical manifest schema source.
- Replace transitional manifest schema in docs/09.
- Update docs/13 priority list to match current issues.
- Keep future tasks in issues, not scattered TODO sections.

Acceptance Criteria:
- [ ] Docs do not simultaneously claim a feature is both missing and implemented.
- [ ] Canonical schema is not duplicated inconsistently.
- [ ] Future work is represented in issues.
- [ ] Remaining TODO/deferred mentions are intentional and explainable.

Validation:
```sh
grep -RIn "transitional\|未完\|TODO\|deferred\|docs/current-state.md" docs README.md issues
```

