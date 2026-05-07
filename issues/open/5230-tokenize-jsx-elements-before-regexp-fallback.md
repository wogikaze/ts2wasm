---
id: 5230
title: "Tokenize JSX elements before RegExp fallback"
type: feature
area: frontend/lexer
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Prevent JSX closing tags such as `</div>` from being tokenized as
unterminated RegExp literals in `.jsx` reference sections.

## Problem

`checkJsxNotSetError.ts` contains a `// @Filename: /foo.jsx` section with a
JSX element. The lexer reaches `</div>` and reports issue-202 instead of
letting the frontend represent or deliberately reject the JSX element.

Problem: `</div>` in a JSX element reports `UnsupportedRegExp: unterminated RegExp literal`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsxNotSetError.ts
```

Source shape:

```tsx
// @Filename: /foo.jsx
const Foo = () => (
    <div>foo</div>
);
export default Foo;
```

Compiler evidence:

```text
tokens: UnsupportedRegExp issue-202 at </div>
ast: same lexer failure
TypeScript oracle: TS2304 for div, TS2552 for foo, TS1161 for the closing tag in this jsx-not-set case
```

## Desired final state

The lexer/parser classifies JSX-looking `<...>` input in `.jsx` sections as a
JSX boundary instead of falling through to RegExp literal scanning at the
closing tag.

## Scope

In scope:

- [ ] Detect the representative JSX element shape before RegExp fallback.
- [ ] Emit a source-spanned JSX unsupported diagnostic or parse enough JSX to reach the intended later diagnostic.
- [ ] Add one focused lexer/parser fixture for `<div>foo</div>`.

Out of scope:

- Full JSX emit or React runtime semantics.
- JSX attribute parsing.
- JSX spread tags.
- Absolute import resolution for `/foo`.

## Affected paths

Expected:

- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/parser.rs`
- focused frontend or CLI tests

Do not touch:

- `crates/backend-wasm/`
- unrelated RegExp runtime behavior

## Acceptance criteria

- [ ] `checkJsxNotSetError.ts` no longer reports `UnsupportedRegExp` for `</div>`.
- [ ] A focused fixture proves `<div>foo</div>` does not enter the unterminated RegExp diagnostic path.
- [ ] Existing real unterminated RegExp diagnostics still report issue-202.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(jsx) or test(regexp) or test(lexer)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsxNotSetError.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsxNotSetError.ts --detail --no-dashboard-data
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated bucket `issues/done/1139-implement-checkJsxNotSetError.md`.

## Completion evidence

Fill when implemented.
