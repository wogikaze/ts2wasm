---
id: 1377
title: "Implement Commentsonreturnstatement"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: [5275]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1377.

## Summary

Closed as superseded by
`issues/open/5275-parse-modified-static-class-methods.md`.

Fresh triage shows this generated bucket is not currently blocked by return
statement parsing. The parser stops earlier at `public static debugFunc()`,
the same modified static class method form owned by issue 5275.

## Problem

Reference test results originally showed 1 case failing in directory
`commentsOnReturnStatement` with diagnostics: parser-syntax. Fresh focused
triage on 2026-05-07 shows tokenization succeeds, but class member parsing
expects a parameter list after `public static` and rejects the method name.

Problem: `commentsOnReturnStatement1.ts` currently cannot parse
`public static debugFunc()`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnReturnStatement1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnReturnStatement1.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Smart triage reports:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("debugFunc")) at 85..94
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5275-parse-modified-static-class-methods.md`; return statement
comment fidelity should be rechecked after the modified static method parser
blocker advances.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing modified static method parser issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Superseding issue contains the exact `public static name(...)` parser diagnostic family
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, parser/TypeScript AST evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnReturnStatement1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnReturnStatement1.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/open/5275-parse-modified-static-class-methods.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsOnReturnStatement1.ts`

## Duplicate detection

- `issues/open/5275-parse-modified-static-class-methods.md` owns the exact
  parser family: class method declarations with accessibility modifiers before
  `static`, followed by an identifier-named method.
- `issues/open/5270-parse-modified-class-accessor-declarations.md` and
  `issues/open/5271-parse-modified-static-class-fields.md` are adjacent but not
  exact.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage parser syntax: commentsOnReturnStatement1

- Issue class: triage-needed
- Feature label: parser-syntax
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/commentsOnReturnStatement1.ts
```

Source context:

```text
1 | // @target: es2015
2 | // @removeComments: false
3 | class DebugClass {
4 |     public static debugFunc() {
5 |         // Start Debugger Test Code
6 |         var i = 0;
```

Compiler evidence:

```text
tokens: ok; public, static, Ident("debugFunc"), LeftParen, Return, True are present
ast/resolved: expected LeftParen, got Some(Ident("debugFunc")) at 85..94
```

TypeScript oracle:

```text
ok: true
diagnostics: []
AST path: ClassDeclaration -> MethodDeclaration -> Identifier debugFunc
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnReturnStatement1.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnReturnStatement1.ts
result: public static method parser blocker; superseded by issue 5275
date: 2026-05-07
```

Remaining risks:

- Return statement comment behavior has not been reached yet; after issue 5275
  advances this path, a later blocker may need separate triage.
