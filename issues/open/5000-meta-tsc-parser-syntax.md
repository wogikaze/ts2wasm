---
id: 5000
title: "Meta: TypeScript Compiler Parser Syntax Coverage"
type: meta
area: frontend/syntax
class: design
priority: P1
depends_on: []
blocks: [5001, 5002, 5003, 5005]
created: 2026-05-02
updated: 2026-05-02
---

## Summary

Covers all TypeScript compiler test cases that fail with `parser-syntax` diagnostic (1,172 issues). These are individual tsc test cases that require parser-level syntax support, or were misclassified by the diagnostic classifier.

## Problem

1,172 reference test cases across the TypeScript compiler suite produce `parser-syntax` diagnostic errors. Each is tracked as an individual triage-needed issue. This meta-issue organizes them for coordinated implementation.

**Note:** Many issues are misclassified by the diagnostic text-matching heuristic (`"expected "`, `"unsupported character"`, etc.). Phase A (re-triage audit) will reclassify these to the appropriate meta-issue (5001 semantic, 5002 type system, 5005 resolver).

## Scope

In scope:

- Parser support for all TypeScript syntax constructs reported as `parser-syntax`
- Individual child issues (IDs 066-4814) each cover one tsc test case or test family
- **Phase A:** Re-classify misclassified issues to correct meta-issues

Out of scope:

- Semantic analysis of parsed constructs (covered by meta-issues 5001, 5003)
- Resolver/name-resolution (covered by meta-issue 5005)
- Runtime builtins (covered by meta-issue 5004)
- Module/namespace ownership design (tracked as issue-399)

## Acceptance criteria

- [ ] All 1,172 child issues are closed or moved to appropriate meta-issues
- [ ] Parser-syntax diagnostic count trends toward zero as child issues are resolved
- [ ] Remaining parser-syntax failures are confirmed to be true parser issues

## Validation

```sh
mise run reference-coverage -- tsc --limit 100 --detail
```

## Status

| Phase | Status | Description |
|-------|--------|-------------|
| Waves 1-2 | ✅ Done | Keyword property names, for-in type annotations, numeric keys, access modifiers, enum erasure, function overloads, class implements |
| Phase A | 🔄 In progress | Re-triage audit: classify remaining issues |
| Phase B | ⏳ Pending | Pure erasure remaining (~15 issues) |
| Phase C | ⏳ Pending | Parse rule changes (~10 issues) |
| Phase D | ⏳ Pending | Complex type erasure (~5 issues) |
| Phase E | ⏳ Pending | Close #5000 |

## Notes

See `docs/superpowers/plans/2026-05-02-tsc-parser-syntax.md` for the full implementation plan.
