---
id: 406
title: "Direct eval Annex B existing binding residuals"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P3
depends_on: [347, 348, 349]
blocks: [225]
created: 2026-05-01
updated: 2026-05-01
status: done
completed: 2026-05-01
---

## Summary

Unblock the remaining direct-eval Annex B cases in the limit-300 Test262
window after issues 347, 348, and 349. The next concrete representative is an
eval-code existing binding case that now fails before eval semantics can be
validated.

## Problem

Issue 225 cannot close: the limit-300 Test262 window still reports eval-labeled
unsupported cases after the first direct-eval implementation slices.

Problem: direct eval Annex B existing-binding representatives still report
`UnsupportedSyntax: eval` or adjacent parser/name-resolution blockers.

## Current failure

```sh
mise run reference-coverage -- test262 --limit 300 --no-web-ui
```

Current result:

```text
unsupported_diagcodes=UnsupportedSyntax:152,UnresolvedName:64,UnsupportedRegExp:33,UnsupportedBuiltin:17,UnresolvedFunction:13,UnsupportedDate:6,UnsupportedEval:6,DuplicateLocal:3
unsupported_features=string-builtin:75,name-resolution:64,regexp-literal:52,eval:29,date:21,builtin-api:18,legacy-global-builtin:18,function-resolution:13,duplicate-local:3,array-builtin:1
```

Representative:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-block-fn-no-init.js
```

Previous result:

```text
[UnsupportedSyntax] expected identifier or string literal as object key, got Some(Function) at 1566..1655
```

Current result after the parser-blocker slice:

```text
[UnsupportedEval] issue-406: direct eval Annex B existing-binding sequences with statements before and after block function declarations are not implemented yet
```

## Desired final state

The selected direct-eval Annex B existing-binding representative advances past
the current parser/frontend blocker and either builds successfully or reports
the next precise issue-linked eval semantic blocker.

## Scope

In scope:

- [x] Reproduce and classify the selected existing-binding direct-eval case.
- [x] Fix the smallest parser/preprocessor/lowering blocker needed to advance
      that case.
- [x] Preserve the existing direct-eval shim behavior from issues 347-349.
- [x] Update issue 225 with refreshed limit-300 eval evidence.

Out of scope:

- Broad `Function` constructor HTML-comment Annex B cases.
- Full indirect eval or host global-environment semantics.
- Unrelated regexp/date/string builtin blockers in the same limit window.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `issues/done/225-implement-eval-annexb-function-declarations.md`

Do not touch:

- unrelated builtin families

## Acceptance criteria

- [x] The selected representative no longer reports `expected identifier or
      string literal as object key, got Some(Function)`.
- [x] Focused regression coverage is added for the parser/frontend or eval
      lowering shape that changed.
- [x] `mise run reference-triage -- test262 reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-block-fn-no-init.js`
      reaches `BuildPass` or a more precise issue-linked semantic blocker.
- [x] Issue 225 records refreshed limit-300 eval evidence.

## Validation

Required commands:

```sh
cargo fmt --all --check
mise run reference-triage -- test262 reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-block-fn-no-init.js
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 300 --no-web-ui
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected unless eval/shim semantics change

Current state:

- [x] not affected unless eval/shim semantics change

Follow-up issues:

- [x] none

## Notes

Start with the prepared Test262 source around the failing span. The failure
appears while parsing the harness/source combination before direct eval
semantics can be checked.

2026-05-01 child-worker slice:

- The representative now reports the issue-linked eval semantic blocker above
  instead of falling through to object-literal parsing for the eval suffix
  `{ function f() { } }`.
- Issue remains open because the underlying Annex B existing-binding semantics
  are still unsupported.

## Completion evidence

Commits:

- `d005943f` issue-406: close eval existing binding parser residual

Validation result:

```text
command: cargo test -p ts2wasm-frontend expands_direct_eval_existing_block_function_residuals -- --nocapture
result: pass (1 parser regression)
date: 2026-05-01

command: cargo fmt --all --check
result: pass
date: 2026-05-01

command: mise run reference-triage -- test262 reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-block-fn-no-init.js
result: BuildPass / build-pass
date: 2026-05-01

command: mise run reference-coverage -- test262 --limit 300 --no-web-ui
result: pass; build_pass=7; semantic_pass=2; unsupported=293; unsupported_features includes eval:28
date: 2026-05-01
```

Remaining risks:

- Issue 225 remains open because the limit-300 window still reports `eval:28`
  and `UnsupportedEval:6`; this issue only closed the selected existing-binding
  representative and parser/frontend blocker.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/406-direct-eval-annexb-existing-binding-residuals.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
