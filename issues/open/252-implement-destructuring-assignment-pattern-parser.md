---
id: 252
title: "Implement destructuring assignment pattern parser support"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: ["247"]
blocks: []
status: done
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
---

## Summary

Implement parser support for destructuring assignment expressions separately from binding declarations and parameters.

Problem: Issue 247 intentionally covers binding patterns only; assignment targets such as `({ x } = obj)` and `[a] = arr` still are not represented as assignment patterns.

## Current failure

Representative examples:

```ts
({ x } = obj);
[a, b] = arr;
```

These are outside the issue 247 binding-pattern parser slice and should remain tracked separately.

## Desired final state

Destructuring assignment expressions parse into explicit assignment-pattern AST forms with diagnostics for invalid assignment targets.

## Scope

In scope:

- [x] Parse object destructuring assignment expressions.
- [x] Parse array destructuring assignment expressions.
- [x] Cover defaults, nesting, elisions, and rest placement at parser level.
- [x] Reject invalid assignment targets with issue-linked diagnostics.

Out of scope:

- Runtime assignment semantics unless this issue explicitly expands after parser support.
- `for-in` / `for-of` destructuring heads.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`

Do not touch:

- unrelated runtime builtins
- broad iterator protocol implementation

## Acceptance criteria

- [x] `({ x } = obj);` parses without being confused with an object literal statement.
- [x] `[a, b] = arr;` parses as a destructuring assignment target.
- [x] Invalid rest placement reports an explicit issue-linked diagnostic.
- [x] Parser/CLI dump tests cover supported assignment-pattern forms.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli --test dump_cli
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/destructuring/assignment/ --detail
```

Not run:

- `mise run reference-coverage -- test262 --path-filter reference/test262/test/language/destructuring/assignment/ --detail` could not run because `reference/test262` is not checked out in this worktree.

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] not affected; runtime support was not changed

Follow-up issues:

- [x] none

## Notes

This issue is split from issue 247 so binding parser support can close without claiming assignment-pattern support.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- branch HEAD for issue 252 close

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -p ts2wasm-frontend
result: pass (80 tests)
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli --test dump_cli
result: pass (36 tests)
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-29

command: mise run check issues
result: pass
date: 2026-04-29

command: cargo nextest run
result: pass (460 tests run, 460 passed, 4 skipped)
date: 2026-04-29

command: mise run reference-coverage -- test262 --path-filter reference/test262/test/language/destructuring/assignment/ --detail
result: not run; required source checkout `reference/test262` is missing
date: 2026-04-29
```

Remaining risks:

- Runtime destructuring assignment semantics are intentionally out of scope and remain unsupported.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/252-implement-destructuring-assignment-pattern-parser.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
