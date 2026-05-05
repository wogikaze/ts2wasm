---
id: 5007
title: "Meta: TypeScript Compiler Module Resolution Coverage (audit reopened #5007)"
type: meta
area: frontend/resolver
class: design-ready
priority: P2
depends_on: [5005]
blocks: []
created: 2026-05-02
updated: 2026-05-05status: open
---

## Summary

Covers TypeScript compiler test cases for module resolution (~18 issues; 11 overload/type resolution issues moved to 5005, 3 bucket issues moved to done/). Module and import resolution is a subset of name resolution.

## Problem

~18 tsc test cases (~30 originally, 11 reclassified as overload/type resolution → 5005, 3 bucket issues → done/) fail due to module resolution gaps including base URL, paths, and module-name resolution.

## Scope

In scope:

- Module resolution algorithms
- Import/export path resolution
- Base URL and path mapping

Out of scope:

- General name resolution (meta-issue 5005)
- Type checking

## Affected paths

Expected:

- `crates/frontend/src/`

## Acceptance criteria

- [ ] All ~18 child issues dependency-linked to this meta

## Validation

```sh
mise run reference-coverage -- tsc --limit 20 --detail
```

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5007-meta-tsc-module-resolution.md` before this move
- `issues/open/5007-meta-tsc-module-resolution.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
