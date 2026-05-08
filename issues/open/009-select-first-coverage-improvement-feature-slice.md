# Select first coverage-improvement feature slice (audit reopened #009)

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Closed**: 2026-04-26
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

Acceptance criteria:

- [x] One implementation issue is created from coverage data.
- [x] The issue targets semantic pass improvement.
- [x] The issue identifies affected workstream and exact fixtures.

Validation result:

```text
mise run reference-coverage -- tsc --limit 300 --detail
  ⚠️ completed with wrapper-capture run due Windows temp-cleanup/path constraints.

mise run reference-coverage -- tsgo --limit 165 --detail
  ⚠️ completed with wrapper-capture run due Windows temp-cleanup/path constraints.
```

Close evidence:

- 2026-04-26: Created follow-up implementation issue `issues/done/202-implement-regexp-literal-support.md` from coverage data (`unsupported_features.regexp-literal` priority area).
- 2026-04-26: Issue 202 targets `frontend/semantics` and a parser-level implementation slice.

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/009-select-first-coverage-improvement-feature-slice.md` (moved from open/ per close evidence)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
