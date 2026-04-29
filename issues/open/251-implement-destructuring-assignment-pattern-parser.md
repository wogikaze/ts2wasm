---
id: 251
title: "Implement destructuring assignment pattern parser support"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: ["247"]
blocks: []
created: 2026-04-29
updated: 2026-04-29
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

- [ ] Parse object destructuring assignment expressions.
- [ ] Parse array destructuring assignment expressions.
- [ ] Cover defaults, nesting, elisions, and rest placement at parser level.
- [ ] Reject invalid assignment targets with issue-linked diagnostics.

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

- [ ] `({ x } = obj);` parses without being confused with an object literal statement.
- [ ] `[a, b] = arr;` parses as a destructuring assignment target.
- [ ] Invalid rest placement reports an explicit issue-linked diagnostic.
- [ ] Parser/CLI dump tests cover supported assignment-pattern forms.

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

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/language-reference/javascript-features.md`

Current state:

- [ ] not affected unless runtime support changes

Follow-up issues:

- [ ] none

## Notes

This issue is split from issue 247 so binding parser support can close without claiming assignment-pattern support.

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
