---
id: 062b
title: "Own dynamic Function constructor diagnostics"
type: feature
area: frontend/semantics
class: verification-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
---

Problem: Dynamic `Function(...)` and `new Function(...)` behave like runtime
code evaluation. They must not silently become wasm semantics without an
explicit dynamic evaluation policy.

## Summary

Keep dynamic Function constructor support diagnostic-only, preserve issue-linked
unsupported evidence, and own the Annex B dynamic Function cases formerly listed
by issue 063.

## Scope

In scope:

- [x] Verify `Function(...)` reports an issue-linked unsupported diagnostic.
- [x] Verify `new Function(...)` reports the same policy-backed diagnostic.
- [x] Preserve Annex B HTML-comment parameter/body cases from issue 063 as owned
  reference evidence.
- [x] Add or update regression fixtures only for diagnostic behavior.

Out of scope:

- Implementing runtime code evaluation.
- Implementing `eval`.
- Adding parser support unrelated to dynamic Function constructor diagnostics.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`
- `issues/done/062-implement-function.md`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [x] Direct and constructed dynamic Function forms produce stable unsupported diagnostics.
- [x] Diagnostic text links the behavior to dynamic runtime code evaluation policy.
- [x] Issue 063 affected tests are referenced here or in regression evidence.
- [x] No dynamic evaluation runtime semantics are implemented in this issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(function_constructor) or test(unsupported)'
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 94 --detail
```

Not run:

- `mise run reference-coverage -- test262 --limit 94 --detail` could not run
  because `reference/test262` is absent in this worktree.

## Affected test files from superseded issue 063

- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-params.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-open-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-open-comment-params.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-no-line-terminator-html-close-comment-body.js`

## Completion evidence

Dynamic `Function(...)` and `new Function(...)` remain diagnostic-only. The
regression fixtures under `fixtures/core-semantics/` assert the build rejects
both forms with `UnsupportedSyntax`, the issue-linked Function constructor
diagnostic, and the policy text `runtime code evaluation is intentionally not
implemented`. The Annex B HTML-comment Function constructor test262 paths from
superseded issue 063 are listed above as owned reference evidence. No runtime
dynamic evaluation semantics were implemented.

Commits:

- pending close commit on branch `agent/062b-function-diagnostics-20260429T000603Z`

Validation result:

```text
command: cargo fmt --all --check
result: passed
date: 2026-04-29

command: cargo nextest run -E 'test(function_constructor) or test(unsupported)'
result: passed, 26 tests run
date: 2026-04-29

command: cargo nextest run
result: passed, 414 tests run, 4 skipped
date: 2026-04-29

command: mise run update-issue-index && mise run update-issue-index -- --check
result: passed; issues/index.md regenerated and verified up to date
date: 2026-04-29

command: mise run check-agent-state
result: passed
date: 2026-04-29

command: mise run check issues; mise run check issue-index
result: failed on pre-existing missing report artifact paths in issue 052 and done issue 228; no 062b/index-specific errors
date: 2026-04-29

command: mise run reference-coverage -- test262 --limit 94 --detail
result: blocked because reference/test262 is missing from this worktree
date: 2026-04-29
```

Remaining risks:

- Reference coverage was not rerun locally because the assigned worktree lacks
  `reference/test262`, and the assignment forbids editing `reference/`.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/062b-dynamic-function-constructor-diagnostics.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
