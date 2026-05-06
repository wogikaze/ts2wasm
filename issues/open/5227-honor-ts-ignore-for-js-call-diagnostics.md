---
id: 5227
title: "Honor @ts-ignore for JavaScript call diagnostics"
type: feature
area: frontend/diagnostics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Honor TypeScript-style `// @ts-ignore` line directives for source-spanned
call-expression diagnostics in JavaScript `checkJs` files.

## Problem

`checkJsFiles_skipDiagnostics.ts` binds `x` as a number and then calls it under
several `// @ts-ignore` comments. TypeScript suppresses those line-comment
diagnostics, but the compiler stops on the first ignored call with issue-211.

Problem: the representative reports issue-211 at byte span `130..133` for an
ignored `x()` call instead of applying the adjacent `/// @ts-ignore`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts
```

Coverage command:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts --detail --no-dashboard-data
```

Source shape:

```ts
// @allowJs: true
// @checkJs: true
// @noEmit: true

// @fileName: a.js
var x = 0;

/// @ts-ignore
x();

/// @ts-ignore
x();

/// @ts-ignore
x(
    2,
    3);
```

Compiler evidence:

```text
coverage: unsupported=1; unsupported_diagcodes=UnresolvedFunction:1
tokens/ast: ok; each x(...) is represented as Expr(Call(Ident("x"), ...))
resolved/lowered: issue-211 function-valued local call at byte span 130..133
TypeScript oracle: TS2349 only for the block-comment pseudo-directive calls at lines 35 and 41
```

## Desired final state

The diagnostic path recognizes line-comment `@ts-ignore` directives and
suppresses a diagnostic whose primary span starts on the ignored statement.

## Scope

In scope:

- [ ] Build a reusable line-comment `@ts-ignore` map from source text.
- [ ] Suppress source-spanned call diagnostics on ignored JavaScript checkJs statements.
- [ ] Add one focused fixture with an ignored `x()` call and an unsuppressed `x()` call.

Out of scope:

- Full TypeScript semantic type checking.
- Making arbitrary function-valued local calls executable.
- Block-comment directive support.
- TS2349 wording for unsuppressed non-callable values.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- unrelated runtime method-call semantics

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts` no longer reports issue-211 at byte span 130..133 for the first ignored `x()` call.
- [ ] A focused fixture proves `/// @ts-ignore` suppresses the next JavaScript checkJs call diagnostic.
- [ ] The same fixture keeps an adjacent unsuppressed call diagnostic visible.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(ts_ignore) or test(diagnostic)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1135-implement-checkJsFiles.md`.

The existing issue 211 receiver/local-call work intentionally leaves dynamic
function-valued local calls unsupported. This issue is narrower: it should not
make `x()` executable, only honor the directive for diagnostics that TypeScript
suppresses in the representative JavaScript checkJs fixture.

## Completion evidence

Fill when implemented.
