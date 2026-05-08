---
id: 5007
title: "Meta: TypeScript Compiler Module Resolution Coverage (audit reopened #5007)"
type: meta
area: frontend/resolver
class: done
priority: P2
depends_on: [5005]
blocks: []
created: 2026-05-02
completed: 2026-05-06
updated: 2026-05-06
status: done
---

## Summary

Covers TypeScript compiler test cases for module resolution (~18 issues; 11 overload/type resolution issues moved to 5005, 3 bucket issues moved to done/). Module and import resolution is a subset of name resolution.

## Problem

~18 tsc test cases (~30 originally, 11 reclassified as overload/type resolution → 5005, 3 bucket issues → done/) fail due to module resolution gaps including base URL, paths, and module-name resolution.

Problem: module-resolution failures currently need child issue classification by import/export path behavior before resolver implementation work can be selected.

## Current failure

Current failure: `mise run reference-coverage -- tsc --limit 20 --detail` reports module-resolution gaps, but this reopened meta issue lacks a concrete dependency cleanup contract.

## Scope

In scope:

- [x] Review child issues currently labeled or dependency-linked as module-resolution.
- [x] Keep import/export path resolution, base URL, path mapping, and module-name lookup children under `5007`.
- [x] Move overload, type-resolution, and general name-resolution children to narrower meta issues.

Out of scope:

- General name resolution (meta-issue 5005)
- Type checking

## Affected paths

Expected:

- `crates/frontend/src/`
- `issues/open/`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [x] Module-resolution child issues are dependency-linked to `5007` only when module path or module-name lookup behavior is the primary blocker.
- [x] Overload/type-resolution/name-resolution children are linked to the correct meta issue.
- [x] `issues/index.md` is regenerated after dependency or class edits.

## Validation

Required commands:

```sh
mise run update-issue-index
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 20 --detail
```

Not run:

- none

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/5007-meta-tsc-module-resolution.md` before this move
- `issues/done/5007-meta-tsc-module-resolution.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Completion evidence

Date: 2026-05-06

This meta issue is closed as the module-resolution classification/design gate,
not as a claim that module resolution has been implemented.

Live issue-file evidence:

```text
rg -l 'depends_on: \[[^\]]*5007' issues/open -g '*.md'
result: no direct open children currently depend on 5007

rg -l 'diagnostics: module-resolution' issues/open -g '*.md'
result: 10 open module-resolution diagnostic buckets remain
```

The remaining `module-resolution` diagnostic buckets are overload/type-name
resolution cases such as `overloadResolutionOnDefaultConstructor`,
`functionDeclarationWithResolutionOfTypeNamedArguments`, and
`typeArgumentConstraintResolution`. They are intentionally linked to `5005`, as
recorded in this issue's existing summary: overload/type-resolution issues were
moved to `5005`, while import/export path module-resolution work should be
tracked separately when concrete children are created.

Validation:

```text
python scripts/manager.py update-issue-index
result: pass

python scripts/manager.py update-issue-index --check
result: pass

python scripts/manager.py check issue-index
result: pass

python scripts/manager.py check issues
result: pass
```
