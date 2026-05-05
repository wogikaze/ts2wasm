---
id: 5005
title: "Meta: TypeScript Compiler Name Resolution Coverage"
type: meta
area: frontend/resolver
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-02
completed: 2026-05-06
updated: 2026-05-06
---

## Summary

Covers TypeScript compiler test cases requiring name resolution and scope analysis (~462 issues). These fail with resolver or name-resolution diagnostics.

## Problem

~462 tsc test cases fail due to missing or incomplete name resolution, scope management, and symbol table logic.

Problem: name-resolution failures currently need child issue classification by resolver behavior so implementers can select one symbol lookup, binding, or module-name slice.

## Current failure

Current failure: `mise run reference-coverage -- tsc --limit 50 --detail` reports name-resolution families that are too broad to implement directly from this meta issue.

## Scope

In scope:

- [x] Review child issues currently dependency-linked to `5005`.
- [x] Keep symbol table, scope chain, identifier binding, shadowing, and name lookup children under `5005`.
- [x] Move pure scope-analysis or module-resolution children to `5006` or `5007` when narrower.

Out of scope:

- Type checking (meta-issue 5002)
- Semantic analysis (meta-issue 5001)

## Affected paths

Expected:

- `crates/frontend/src/`
- `issues/open/`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [x] Name-resolution child issues are dependency-linked to `5005` only when resolver behavior is the primary blocker.
- [x] Scope-analysis and module-resolution children are linked to `5006` or `5007` where appropriate.
- [x] `issues/index.md` is regenerated after dependency or class edits.

## Validation

Required commands:

```sh
mise run update-issue-index
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 50 --detail
```

Not run:

- none

## Progress evidence

2026-05-06 dependency cleanup slice:

- Reviewed `5005` direct children for generated buckets whose filename and
  problem statement explicitly identify `parser-syntax`.
- Relinked 16 parser-syntax buckets from `depends_on: [5005]` to
  `depends_on: [5000]` and normalized their area to `frontend/syntax`.
- Follow-up scan found 70 more direct children whose recorded current
  diagnostic is `parser-syntax`; relinked those to `5000` and normalized their
  area to `frontend/syntax`.
- Left `5002`, `5006`, and `5007` linked to `5005` because they are explicit
  type-system/scope-analysis/module-resolution meta dependencies, not
  accidental generated child buckets.

Validation result:

```text
python scripts/manager.py update-issue-index: pass
python scripts/manager.py update-issue-index --check: pass
python scripts/manager.py check issues: pass
```

## Completion evidence

Date: 2026-05-06

This meta issue is closed as the name-resolution classification/design gate,
not as a claim that all resolver failures are implemented.

After the parser-diagnostic relinks, direct open children that still depend on
`5005` have these recorded diagnostics:

```text
7 function-resolution
1 method-call
10 module-resolution
381 name-resolution
```

The remaining `module-resolution` direct children are overload/type-argument
resolution buckets such as `overloadResolutionOnDefaultConstructor`,
`functionDeclarationWithResolutionOfTypeNamedArguments`, and
`typeArgumentConstraintResolution`. Those match the existing `5007` note that
overload/type-resolution buckets belong under `5005`, while import/export path
module-resolution cleanup remains under `5007`.

Validation:

```text
rg -l 'depends_on: \[[^\]]*5005' issues/open -g '*.md' | xargs rg -n 'Reference test results show .* diagnostics:' | sed -E 's/.*diagnostics: ([^.]+).*/\1/' | sort | uniq -c
result: no parser-syntax or scope-analysis diagnostics remain directly linked to 5005

python scripts/manager.py update-issue-index
result: pass

python scripts/manager.py update-issue-index --check
result: pass

python scripts/manager.py check issue-index
result: pass

python scripts/manager.py check issues
result: pass
```
