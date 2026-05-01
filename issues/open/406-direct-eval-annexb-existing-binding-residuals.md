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

Current result:

```text
[UnsupportedSyntax] expected identifier or string literal as object key, got Some(Function) at 1566..1655
```

## Desired final state

The selected direct-eval Annex B existing-binding representative advances past
the current parser/frontend blocker and either builds successfully or reports
the next precise issue-linked eval semantic blocker.

## Scope

In scope:

- [ ] Reproduce and classify the selected existing-binding direct-eval case.
- [ ] Fix the smallest parser/preprocessor/lowering blocker needed to advance
      that case.
- [ ] Preserve the existing direct-eval shim behavior from issues 347-349.
- [ ] Update issue 225 with refreshed limit-300 eval evidence.

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
- `issues/open/225-implement-eval-annexb-function-declarations.md`

Do not touch:

- unrelated builtin families

## Acceptance criteria

- [ ] The selected representative no longer reports `expected identifier or
      string literal as object key, got Some(Function)`.
- [ ] Focused regression coverage is added for the parser/frontend or eval
      lowering shape that changed.
- [ ] `mise run reference-triage -- test262 reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-block-fn-no-init.js`
      reaches `BuildPass` or a more precise issue-linked semantic blocker.
- [ ] Issue 225 records refreshed limit-300 eval evidence.

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

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
