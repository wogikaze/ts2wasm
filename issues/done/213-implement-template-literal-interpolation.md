---
id: 213
title: "Implement template literal interpolation"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Implement `${...}` expression interpolation for JavaScript template literals.

## Problem

Issue 041 added basic backtick literal syntax but recorded interpolation as deferred work. Literal-only parsing must stay separate from full template-literal semantic support.

## Desired final state

Template literals concatenate cooked string segments and expression values using JavaScript string conversion, matching Node.js for the supported subset.

## Scope

In scope:

- [x] Parse template literal parts with embedded `${...}` expressions.
- [x] Lower interpolation to string conversion and concatenation.
- [x] Add Node differential fixtures for one expression, multiple expressions, empty segments, and escaped backticks.
- [x] Update docs/current-state/issues when semantic status changes.

Out of scope:

- Tagged template literals.
- Full raw/cooked template object semantics.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- none

## Acceptance criteria

- [x] Template literal interpolation with `${...}` parses and executes.
- [x] Interpolated values use the project's JavaScript string conversion path.
- [x] Node differential fixtures cover multiple interpolation shapes.
- [x] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(template)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] update `docs/language-reference/javascript-features.md`

Current state:

- [x] update `current-state.md`

Follow-up issues:

- [x] none

## Notes

Created from issue 203 audit of `issues/done/041-implement-template-literals.md`.

## Completion evidence

Commits:

- `3af66ea` issue-213: implement template interpolation
- close commit for docs/issue sync

Validation result:

```text
command: cargo test -p ts2wasm-frontend template --lib -- --nocapture
result: pass (3 template parser tests)
date: 2026-04-28

command: cargo test -p ts2wasm-cli --test ir_lowering template -- --nocapture
result: pass (template interpolation lowers through addition)
date: 2026-04-28

command: cargo test -p ts2wasm-cli --test m2_node_diff template_literal_fixture_matches_node_output_under_iwasm -- --nocapture
result: pass (Node/iwasm stdout matched)
date: 2026-04-28

command: cargo nextest run -E 'test(template)'
result: pass (5 passed)
date: 2026-04-28

command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: mise run check-agent-state
result: pass
date: 2026-04-28

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-28

command: mise run check-issue-health
result: pass
date: 2026-04-28

command: mise run check-repo-smoke
result: pass
date: 2026-04-28

command: cargo nextest run
result: pass (199 passed, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- Tagged templates and full raw/cooked template object semantics remain out of scope.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

