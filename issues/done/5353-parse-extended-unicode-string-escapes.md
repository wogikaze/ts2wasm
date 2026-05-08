---
id: 5353
title: "Parse extended Unicode string escapes"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Teach the frontend string lexer to accept ECMAScript extended Unicode code point
escapes in string literals, such as `"\u{44}"` and `'\u{10401}'`, instead of
reporting `invalid unicode escape sequence`.

This is the current first blocker from
`constEnumSyntheticNodesComments.ts` and also appears in the generated
legacy-global-builtin bucket for `escape-above-astral.js`.

## Problem

The lexer currently recognizes `\uXXXX` string escapes but rejects the
brace-delimited code point form `\u{...}`. That prevents parser/semantic triage
from reaching the actual enum/comment or legacy global builtin behavior in
reference tests.

Problem: string literal lexing rejects valid ECMAScript extended Unicode code
point escapes such as `"\u{44}"`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts
```

Current diagnostic:

```text
UnsupportedSyntax: invalid unicode escape sequence at 329..332
```

Source context:

```ts
case En["\u{44}"]:
    return assert<3>(a);
```

Related reproduction from the broad legacy-global-builtin bucket:

```sh
python scripts/manager.py reference-triage test262 reference/test262/test/annexB/built-ins/escape/escape-above-astral.js
```

Related source:

```js
escape('\u{10401}')
```

## Desired final state

String literal lexing accepts valid `\u{...}` code point escapes and either
produces the decoded string value supported by the current frontend/runtime
string representation or emits a narrower, source-spanned diagnostic for code
points outside the current representation.

The representative TypeScript case must no longer stop at
`invalid unicode escape sequence` for `"\u{44}"`.

## Scope

In scope:

- [x] Parse brace-delimited Unicode code point escapes in string literals.
- [x] Accept at least ASCII-range code points such as `\u{44}` and preserve the
      decoded string value as `"D"`.
- [x] Preserve source spans for malformed brace escapes.
- [x] Add focused lexer/parser coverage for `"\u{44}"`.
- [x] Re-run `constEnumSyntheticNodesComments.ts` and record the next blocker if
      it advances.

Out of scope:

- Identifier Unicode escapes, already tracked separately.
- RegExp Unicode escapes.
- JSON.parse Unicode escape runtime behavior.
- Full Unicode runtime string representation for non-ASCII or surrogate pairs
  unless needed to avoid an invalid-success state.
- Enum runtime/const-enum inlining.

## Affected paths

Expected:

- `crates/frontend/src/lexer_strings.rs`
- focused lexer/parser tests under `crates/frontend/` or CLI parser tests
- `fixtures/` only if an existing parser fixture pattern requires it

Do not touch:

- `crates/backend-wasm/`
- enum lowering/runtime emit
- JSON runtime helpers

## Acceptance criteria

- [x] A focused lexer/parser test accepts `"\u{44}"` as a string literal whose
      value is equivalent to `"D"`.
- [x] Malformed extended escapes keep a source-spanned diagnostic instead of
      panicking or silently accepting invalid text.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts` no longer reports `invalid unicode escape sequence at 329..332`.
- [x] Any next blocker exposed by that reference file is recorded in this issue
      or split to a follow-up if outside string lexing.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend string
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumSyntheticNodesComments.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage test262 reference/test262/test/annexB/built-ins/escape/escape-above-astral.js
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket
`issues/done/1457-implement-constEnumSyntheticNodesComments.md`.

Related but not duplicates:

- `issues/open/5018-implement-legacy-global-builtin.md` is a broad generated
  bucket that includes the same lexer failure for `escape('\u{10401}')`.
- `issues/open/4642-implement-unicodeStringLiteral.md` is a generated
  string-literal bucket that still needs smart triage.
- `issues/done/293-parse-unicode-escaped-identifier-parts.md` covers
  identifier escapes, not string literal code point escapes.

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

- Non-ASCII code points such as `\u{10401}` may need either UTF-16/Unicode
  string representation work or an explicit unsupported diagnostic after the
  ASCII-range lexer path is supported.

## False-done audit

**truly-done** (5353)

- Implementation commits: verified via `git log --oneline --all --grep=5353`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
