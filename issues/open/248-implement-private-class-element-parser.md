---
id: 248
title: "Implement private class element parser support"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: []
blocks: ["059"]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement parser recognition for ECMA-262 `PrivateIdentifier` and private class elements.

Problem: The lexer rejects `#` before the class parser can classify private fields or methods, so private class syntax reports an unsupported character diagnostic.

## Current failure

Representative reproduction:

```sh
tmp=/tmp/ts2wasm-248-private-class.ts
printf 'class C { #x = 1; }\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- dump --ast --unparse "$tmp"
```

Current result:

```text
error: [UnsupportedSyntax] unsupported character: # at 10..11
```

Spec refs:

- `reference/ecma262/spec.html`: PrivateIdentifier, ClassElementName, FieldDefinition
- `reference/test262/test/language/expressions/class/elements/`

## Desired final state

Private identifiers and private class elements are recognized by the lexer/parser. Unsupported private-field runtime behavior is reported after syntax classification with an issue-linked diagnostic rather than as an unknown character.

## Scope

In scope:

- [ ] Tokenize `#name` as a private identifier with spans.
- [ ] Parse private fields, private methods, private getters, and private setters in class bodies.
- [ ] Parse static private fields/methods as syntax.
- [ ] Reject invalid private identifier forms and duplicate private declarations with stable diagnostics where parser-level checks own them.
- [ ] Add parser and CLI diagnostic coverage for private class elements.

Out of scope:

- Implementing private field storage and runtime access semantics.
- Optional chaining of private fields.
- Decorators.
- Class static blocks; tracked separately by issue 249.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`

Do not touch:

- unrelated class runtime behavior unless required for stable diagnostics

## Acceptance criteria

- [ ] `class C { #x = 1; }` no longer reports `unsupported character: #`.
- [ ] Private method/accessor syntax parses or reports an issue-linked unsupported diagnostic after private identifier tokenization.
- [ ] Invalid private identifier forms are covered by diagnostics.
- [ ] A focused reference slice under `reference/test262/test/language/expressions/class/elements/` no longer fails at the first `#` character.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli --test dump_cli
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/class/elements/ --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] update `docs/language-reference/javascript-features.md` when support status changes

Current state:

- [ ] update `current-state.md` if private class support boundary changes

Follow-up issues:

- [ ] create runtime private-field semantics issue if this slice stops at parser classification

## Notes

This is a frontend/parser wave child issue split from issue 059 and `docs/language-reference/frontend-parser-wave.md`.

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
