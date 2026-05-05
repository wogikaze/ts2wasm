---
id: 5001
title: "Meta: TypeScript Compiler Semantic Analysis Coverage"
type: meta
area: frontend/semantics
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-02
updated: 2026-05-02
---

## Summary

Covers TypeScript compiler test cases requiring semantic analysis beyond parser-level support (~2,278 issues). These are tsc test failures in the semantic/diagnostics layer.

## Problem

~2,278 reference test cases fail in semantic analysis. Each requires type checking, diagnostic emission, or semantic validation logic in the compiler frontend.

Problem: tsc semantic-analysis failures currently need classified child work orders so implementers can select one diagnostic or semantic family without redoing coverage triage.

## Current failure

Current failure: `mise run reference-coverage -- tsc --limit 100 --detail` reports semantic-analysis families that are too broad to implement directly from this meta issue.

## Scope

In scope:

- [ ] Review semantic-analysis child issues that depend on this meta issue.
- [ ] Ensure each child issue has one concrete diagnostic, source pattern, or semantic family.
- [ ] Update dependency links when a child belongs under a narrower meta issue.

Out of scope:

- Parser syntax support (covered by meta-issue 5000)
- Declaration emit (covered by meta-issue 5003)
- Name resolution (covered by meta-issue 5005)

## Affected paths

Expected:

- `crates/frontend/src/`
- `issues/open/`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] Semantic-analysis child issues are dependency-linked to `5001` only when they need frontend semantic or diagnostic work.
- [ ] Child issues that are parser, declaration-emit, name-resolution, or runtime gaps are moved to the narrower meta dependency.
- [ ] `issues/index.md` is regenerated after dependency or class edits.

## Validation

Required commands:

```sh
mise run update-issue-index
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 100 --detail
```

Not run:

- none

## Progress evidence

2026-05-06 dependency cleanup slice:

- Reviewed `5001` direct children for generated buckets with recorded
  `import-export` diagnostics.
- Relinked 1,384 import/export buckets from `depends_on: [5001]` to the
  existing import/export triage parent `432`.
- Normalized those relinked child areas to `frontend/syntax`, matching the
  import/export syntax/module owner.
- Follow-up scan found 348 direct children with recorded `parser-syntax`
  diagnostics; relinked those to parser meta `5000` and normalized their area
  to `frontend/syntax`.
- Relinked 23 direct children with recorded `type-alias` diagnostics to the
  open TypeScript type-alias parent `345`.
- Relinked 78 direct children with syntax-family diagnostics
  (`ambient-declaration`, `decorator`, `jsx`, `parameter-property`,
  `rest-parameter`, `template-literal`, `type-annotation`, `type-assertion`) to
  parser-extension epic `059` and normalized their area to `frontend/syntax`.

Validation result:

```text
python scripts/manager.py update-issue-index: pass
python scripts/manager.py update-issue-index --check: pass
python scripts/manager.py check issues: pass
```
