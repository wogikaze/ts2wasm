---
id: 5305
title: "Report merge conflict marker diagnostics"
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

Detect Git merge conflict marker lines such as `<<<<<<<`, `|||||||`,
`=======`, and `>>>>>>>` and report a source-spanned TypeScript-compatible
diagnostic instead of falling into generic parser errors.

## Problem

The representative reference case places diff3 conflict markers inside a class
body. The lexer tokenizes marker text as ordinary shift/operator tokens, and
the parser then reports a generic class member error at `<<<<<<<`.

Problem: merge conflict marker source currently reports generic parser syntax
errors instead of a clear conflict-marker diagnostic matching TypeScript's
TS1185 behavior.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/conflictMarkerDiff3Trivia1.ts
```

Current diagnostic:

```text
UnsupportedSyntax: expected property name, got LeftShift at 33..35
line 3, column 5
```

Source context:

```ts
class C {
// marker: <<<<<<< HEAD
    v = 1;
// marker: ||||||| merged common ancestors
    v = 3;
// marker: =======
    v = 2;
// marker: >>>>>>> Branch-a
}
```

Compiler evidence:

```text
tokens: ok; conflict markers tokenize as LeftShift/OrOr/StrictEqual/UnsignedRightShift groups
ast: fails before AST construction with expected property name, got LeftShift
resolved: fails with the same parser diagnostic
```

TypeScript oracle evidence:

```text
TypeScript diagnostics:
- TS1185 Merge conflict marker encountered at line 3
- TS1185 Merge conflict marker encountered at line 5
- TS1185 Merge conflict marker encountered at line 7
- TS1185 Merge conflict marker encountered at line 9
TypeScript AST still contains ClassDeclaration C and PropertyDeclaration v = 1.
```

## Desired final state

The frontend detects conflict marker lines and emits a clear source-spanned
diagnostic for merge conflict markers before generic class/property parsing
misclassifies the marker tokens.

## Scope

In scope:

- [ ] Detect conflict marker line starts for `<<<<<<<`, `|||||||`, `=======`, and `>>>>>>>`.
- [ ] Report a source-spanned diagnostic with wording equivalent to `Merge conflict marker encountered`.
- [ ] Cover conflict markers inside a class body.
- [ ] Re-run `conflictMarkerDiff3Trivia1.ts` and confirm it no longer reports the generic `expected property name, got LeftShift` diagnostic.

Out of scope:

- Recovering and continuing full AST construction after conflict marker diagnostics.
- Resolving or choosing conflict sides.
- General diff/parser recovery beyond conflict marker reporting.

## Affected paths

Expected:

- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/parser/`
- `crates/frontend/src/diagnostic.rs`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- runtime/backend lowering
- module graph or resolver behavior
- coverage dashboard generated data

## Acceptance criteria

- [ ] A focused test reports a merge conflict marker diagnostic for `class C { <<<<<<< HEAD ... }`.
- [ ] `conflictMarkerDiff3Trivia1.ts` no longer reports `expected property name, got LeftShift`.
- [ ] The diagnostic span points at the conflict marker line, not at a later class member parse fallback.
- [ ] Existing shift/operator parsing for normal expressions still passes.
- [ ] Issue state stays synchronized with `issues/index.md`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend conflict_marker
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/conflictMarkerDiff3Trivia1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/conflictMarkerDiff3Trivia1.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from generated bucket `issues/done/1431-implement-conflictMarkerDiff-parser-syntax.md`.
Also supersedes sibling generated bucket
`issues/done/1432-implement-conflictMarkerDiff-unknown-unsupported.md`, where
the same marker diagnostic gap appears inside a method body.
Also supersedes generated bucket
`issues/done/1433-implement-conflictMarkerTrivia-parser-syntax.md`, where the
same marker diagnostic gap appears for non-diff3 markers in a class body.
Also supersedes generated bucket
`issues/done/1434-implement-conflictMarkerTrivia-unknown-unsupported.md`, where
the same marker diagnostic gap appears for `conflictMarkerTrivia2.ts` and
`conflictMarkerTrivia4.ts`.

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
