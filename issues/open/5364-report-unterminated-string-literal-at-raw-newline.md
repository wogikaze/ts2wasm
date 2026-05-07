---
id: 5364
title: "Report unterminated string literal at raw newline"
type: bug
area: frontend/lexer
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report a TypeScript-compatible unterminated string diagnostic when a string literal reaches a raw newline, covering the first blocker in `constructorWithIncompleteTypeAnnotation.ts`.

## Problem

The lexer currently classifies this malformed source as generic `UnsupportedSyntax`:

```text
raw newline in string literal is not allowed at 984..985
```

TypeScript accepts the file for diagnostic collection and reports TS1002 `Unterminated string literal` at the same newline boundary. The generic unsupported failure blocks later malformed constructs in the reference file from being triaged.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorWithIncompleteTypeAnnotation.ts
```

Equivalent mise task:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constructorWithIncompleteTypeAnnotation.ts
```

Source context:

```text
44 |                 retValue = bfs.OPERATOR ' );
45 |                 if (retValue != 0) {
46 |
47 |                     return 1;
```

Smart triage evidence on 2026-05-07:

```text
tokens: fail; raw newline in string literal at 984..985
AST/resolved: fail at same lexer diagnostic
visible symbols: File, Program, bfs, retValue
TypeScript oracle: TS1002 Unterminated string literal at line 44 character 45
coverage: executed=1, build_pass=0, unsupported=1
```

## Desired final state

The lexer/parser reports a source-spanned unterminated string literal diagnostic for raw newline inside a string, matching TypeScript's TS1002 behavior closely enough for reference triage to continue.

## Scope

In scope:

- [ ] Detect raw newline before a closing single-quoted or double-quoted string delimiter.
- [ ] Emit a source-spanned unterminated-string diagnostic instead of generic unsupported syntax.
- [ ] Add a focused lexer/parser regression for `const x = 'unterminated\nnext();`.
- [ ] Re-run `constructorWithIncompleteTypeAnnotation.ts` and split the next blocker if outside this issue.

Out of scope:

- Template literal recovery.
- Full parser recovery for every malformed construct in this reference file.
- TypeScript semantic diagnostics after parser recovery.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch: unrelated runtime/backend code.

## Acceptance criteria

- [ ] `constructorWithIncompleteTypeAnnotation.ts` no longer reports generic `raw newline in string literal is not allowed`.
- [ ] A focused test reports an unterminated string literal diagnostic for a raw newline in a single-quoted string.
- [ ] Existing valid escaped newline or escaped character string tests keep passing.
- [ ] Any next blocker from the reference path is recorded here or split to a follow-up.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(string) or test(lexer) or test(parser)'
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorWithIncompleteTypeAnnotation.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithIncompleteTypeAnnotation.ts --detail --no-dashboard-data
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated bucket `issues/open/1482-implement-constructorWithIncompleteTypeAnnotation.md` on 2026-05-07.

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
