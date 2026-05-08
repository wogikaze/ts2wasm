---
id: 5006
title: "Meta: TypeScript Compiler Scope Analysis Coverage"
type: meta
area: frontend/resolver
class: done
priority: P2
depends_on: [5005]
blocks: []
created: 2026-05-02
completed: 2026-05-06
updated: 2026-05-06
---

## Summary

Covers TypeScript compiler test cases for scope analysis (~32 issues). Scope analysis is a subset of name resolution focusing on block scoping, hoisting, and lexical scope chains.

## Problem

~32 tsc test cases fail due to scope analysis gaps. These are a subset of the broader name resolution work.

Problem: scope-analysis failures currently need child issue classification by block scope, hoisting, lexical lookup, or temporal-dead-zone behavior.

## Current failure

Current failure: `mise run reference-coverage -- tsc --limit 20 --detail` reports scope-analysis gaps, but the meta issue lacks an actionable dependency cleanup contract.

## Scope

In scope:

- [x] Review child issues currently labeled or dependency-linked as scope-analysis.
- [x] Keep block scoping, lexical scope chain, hoisting, and temporal-dead-zone children under `5006`.
- [x] Move general resolver or type-system children to `5005` or `5002`.

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

- [x] Scope-analysis child issues are dependency-linked to `5006` only when lexical scope or hoisting behavior is the primary blocker.
- [x] General resolver and type-system children are linked to the correct meta issue.
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

## Completion evidence

Date: 2026-05-06

This meta issue is closed as the scope-analysis classification/design gate, not
as a claim that scope-analysis behavior has been implemented.

Live issue-file evidence:

```text
rg -l 'depends_on: \[[^\]]*5006' issues/open -g '*.md' | wc -l
result: 24 direct open children

rg -l 'depends_on: \[[^\]]*5006' issues/open -g '*.md' | xargs rg -n 'Reference test results show .* diagnostics:' | sed -E 's/.*diagnostics: ([^.]+).*/\1/' | sort | uniq -c
result: 24 scope-analysis
```

The direct children cover block-scoped binding use-before-definition,
block-scoped reassignment/capture, lexical shadowing, class/static scope
checks, and with-statement nested scope cases. Their area metadata was
normalized to `frontend/resolver` to match the scope-analysis owner.

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

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

