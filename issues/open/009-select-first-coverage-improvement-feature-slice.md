# Select first coverage-improvement feature slice

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 009
**Type**: spike
**Area**: frontend/ir/runtime
**Priority**: P1
**Depends on**: 005
**Orchestration class**: design-ready

Problem: After coverage breakdown exists, the next implementation should be chosen by data. The goal is to increase semantic pass count, not just compile pass count.

Scope:
- Review top unsupported feature labels.
- Separate parser-only gaps from runtime semantic gaps.
- Choose one small feature that increases semantic pass count.
- Create fixtures and reference samples for the chosen feature.
- Produce one follow-up implementation issue.

Acceptance Criteria:
- [ ] One implementation issue is created from coverage data.
- [ ] The issue targets semantic pass improvement.
- [ ] The issue identifies affected workstream and exact fixtures.

Validation:
```sh
scripts/reference_coverage.sh tsc --limit 300
scripts/reference_coverage.sh tsgo --limit 165
```

