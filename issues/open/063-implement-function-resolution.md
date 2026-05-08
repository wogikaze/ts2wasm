---
id: 063
title: "Implement function resolution (dup)"
type: feature
area: frontend/resolver
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-05-05
completed: 2026-04-29
status: done
---

## Summary

Implement function-resolution feature to handle 5 failing test cases in reference tests.

Problem: This issue overlaps with issue 062 and currently lists only Annex B dynamic Function constructor cases, so it is not an independent implementation slice.

Queue design note:

- Do not select this issue directly.
- Superseded by issue 062b. The affected Annex B dynamic Function constructor
  cases are owned by the Function constructor diagnostic/policy child.

## Problem

Reference test results show 5 cases fail with function-resolution diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

function-resolution feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

Original in-scope implementation work, now superseded by issue 062b:

- Add required syntax to lexer/parser
- Implement semantics for function-resolution feature
- Add fixtures for function-resolution feature behavior
- Update diagnostics appropriately

Out of scope:

- Related features (separate issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Original acceptance criteria

- function-resolution feature passes for basic cases
- Related diagnostics reduced in reference tests
- Regression test added for function-resolution feature
- Docs updated if semantics change

These implementation criteria were not completed in issue 063 because the issue
was closed as a duplicate of the dynamic Function constructor diagnostic/policy
slice. The affected reference cases are now tracked by issue 062b.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 10
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- not affected

Current state:

- not updated by this superseded close

Follow-up issues:

- issue 062b owns the affected Annex B dynamic Function constructor cases

## Notes

## Affected test files

- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-params.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-open-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-open-comment-params.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-no-line-terminator-html-close-comment-body.js`

## Completion evidence

Closed as superseded by issue 062b. No function-resolution implementation was
performed in this cleanup.

Commits:

- closing commit on branch `agent/062a-function-split-20260428T235026Z` (hash recorded in cycle report)

Validation result:

```text
command: mise run update-issue-index; mise run update-issue-index -- --check; mise run check issues; mise run check issue-index; mise run check-agent-state
result: index update/check and agent-state passed; issue health commands failed only on pre-existing missing report paths in issue 052 and done issue 228, with no remaining 062a/063 errors
date: 2026-04-29
```

Remaining risks:

- Function constructor diagnostic/policy behavior remains tracked by issue 062b.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: superseded`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/063-implement-function-resolution.md` before this move
- `issues/open/063-implement-function-resolution.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## False-done audit correction

Date: 2026-05-05

Classification: truly-done duplicate/superseded tracking issue.

Audit result: returned to `issues/done/` after review feedback. The title contains `(dup)`, which marks this issue as a duplicate/superseded tracker; duplicate closure issues must not remain in `issues/open/` unless the duplicate mapping itself is invalid.

Evidence files:
- `issues/open/063-implement-function-resolution.md` after this correction
- `issues/index.md` after regeneration
