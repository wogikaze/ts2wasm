---
id: 5002
title: "Meta: TypeScript Compiler Type System Coverage"
type: meta
area: frontend/semantics
class: done
priority: P1
depends_on: [5000, 5005]
blocks: []
created: 2026-05-02
completed: 2026-05-06
updated: 2026-05-06
---

## Summary

Covers TypeScript compiler test cases specifically for type-system semantics (~244 issues). Requires type inference, conditional types, mapped types, and generic type operations.

## Problem

~244 tsc test cases fail with type-system related diagnostics. These require implementing type inference, type relationships, and type-level computations.

Problem: type-system reference failures currently need dependency and scope cleanup so each child issue is a concrete type inference, relationship, or type-level computation slice.

## Current failure

Current failure: `mise run reference-coverage -- tsc --limit 50 --detail` exposes type-system buckets, but child issues still need review against `5001` and `5005` before implementation order is clear.

## Scope

In scope:

- [x] Review child issues currently labeled type-system.
- [x] Keep only type inference, type relationship, conditional type, mapped type, generic constraint, and type-level computation children under `5002`.
- [x] Move parser, declaration-emit, name-resolution, or broad semantic children to their narrower meta dependencies.

Out of scope:

- Basic semantic analysis (meta-issue 5001)
- Name resolution (meta-issue 5005)

## Affected paths

Expected:

- `crates/frontend/src/`
- `issues/open/`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [x] Type-system child issues are dependency-linked to `5002` only when they require type-level implementation work.
- [x] Non-type-system children are linked to the correct narrower meta issue.
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

## Completion evidence

Date: 2026-05-06

This meta issue is closed as the type-system classification/design gate, not as
a claim that TypeScript type-system semantics are implemented.

Live issue-file evidence:

```text
rg -l 'depends_on: \[[^\]]*5002' issues/open -g '*.md' | wc -l
result: 235 direct open children

rg -l 'depends_on: \[[^\]]*5002' issues/open -g '*.md' | xargs rg -n 'Reference test results show .* diagnostics:' | sed -E 's/.*diagnostics: ([^.]+).*/\1/' | sort | uniq -c
result: 234 type-system
```

No direct child with recorded failure evidence is parser-syntax,
declaration-emit, name-resolution, scope-analysis, or module-resolution. Direct
child areas were normalized to `frontend/semantics` to match the type-system
owner.

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

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

