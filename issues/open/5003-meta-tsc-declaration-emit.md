---
id: 5003
title: "Meta: TypeScript Compiler Declaration Emit Coverage"
type: meta
area: frontend/syntax
class: done
priority: P2
depends_on: [5000, 5001]
blocks: []
created: 2026-05-02
completed: 2026-05-06
updated: 2026-05-06
---

## Summary

Covers TypeScript compiler test cases for declaration emit (~104 issues). Primarily `.d.ts` generation and declaration output.

## Problem

~104 tsc test cases fail due to declaration emit (`.d.ts` generation) missing or incorrect.

Problem: declaration-emit failures currently need child issue classification by emitted declaration shape before implementers can safely change frontend emit contracts.

## Current failure

Current failure: `mise run reference-coverage -- tsc --limit 50 --detail` reports declaration-emit gaps, but this meta issue does not yet define the child issue review contract.

## Scope

In scope:

- [x] Review declaration-emit child issues for the emitted `.d.ts` construct or diagnostic they cover.
- [x] Keep declaration output, visibility, and `.d.ts` generation children under `5003`.
- [x] Move parser, runtime emit, name-resolution, or type-system children to the correct meta issue.

Out of scope:

- Runtime code generation
- Parser support (meta-issue 5000)

## Affected paths

Expected:

- `crates/frontend/src/`
- `issues/open/`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [x] Declaration-emit child issues are dependency-linked to `5003` only when the observable output is `.d.ts` or declaration diagnostics.
- [x] Non-declaration children are linked to the correct narrower meta issue.
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

This meta issue is closed as the declaration-emit classification/design gate,
not as a claim that declaration emit has been implemented.

Live issue-file evidence:

```text
rg -l 'depends_on: \[[^\]]*5003' issues/open -g '*.md' | wc -l
result: 92 direct open children after relinking one import/export bucket to 432

rg -l 'depends_on: \[[^\]]*5003' issues/open -g '*.md' | xargs rg -n 'Reference test results show .* diagnostics:' | sed -E 's/.*diagnostics: ([^.]+).*/\1/' | sort | uniq -c
result: 92 declaration-emit
```

The one non-declaration child found in this pass,
`issues/open/2665-implement-import.md`, had recorded `import-export`
diagnostics and was relinked to the import/export triage parent `432`.

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

