---
id: 1378
title: "Implement Commentsonstaticmembers"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: [5288]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1378.

## Summary

Closed after splitting the executable parser work into
`issues/open/5288-parse-typed-modified-static-class-fields.md`.

Fresh triage shows this generated bucket is not currently blocked by comment
emit fidelity. The parser stops earlier at `public static p1: string = ""`,
the typed modified static class field parser family now owned by issue 5288.

## Problem

Reference test results originally showed 1 case failing in directory
`commentsOnStaticMembers` with diagnostics: parser-syntax. Fresh focused
triage on 2026-05-07 shows tokenization succeeds, but class member parsing
expects a parameter list after `public static` and rejects the field name.

Problem: `commentsOnStaticMembers.ts` currently cannot parse
`public static p1: string = "";`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5288-parse-typed-modified-static-class-fields.md`; static member comment
emit fidelity should be rechecked after the modified static field parser blocker
advances.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split this bucket into a typed modified static field parser issue
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
- [x] Child issue contains the exact `public/private static name: Type` parser diagnostic family
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts
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

- [x] created/updated: `issues/open/5288-parse-typed-modified-static-class-fields.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts`

## Duplicate detection

- `issues/open/5288-parse-typed-modified-static-class-fields.md` owns the exact
  parser family: class field declarations with accessibility modifiers before
  `static`, followed by an identifier-named field with a TypeScript type
  annotation and optional initializer.
- `issues/open/5271-parse-modified-static-class-fields.md` is adjacent but not
  exact; it owns the untyped modified static field parser boundary.
- `issues/open/5275-parse-modified-static-class-methods.md` is adjacent but not
  exact; it owns modified static methods, not property declarations.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage parser syntax: commentsOnStaticMembers

- Issue class: triage-needed
- Feature label: parser-syntax
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts
```

Source context:

```text
5 |     /**
6 |      * p1 comment appears in output
7 |      */
8 |     public static p1: string = "";
9 |     /**
10 |      * p2 comment does not appear in output
11 |      */
```

Compiler evidence:

```text
tokens: ok; public, static, Ident("p1"), Colon, Ident("string"), Equal, String("") are present
ast/resolved: expected LeftParen, got Some(Ident("p1")) at 129..131
visible symbols before failure: class test
```

TypeScript oracle:

```text
ok: true
diagnostics: []
AST path: ClassDeclaration -> PropertyDeclaration "public static p1: string = \"\";" -> Identifier p1
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnStaticMembers.ts
result: public static typed field parser blocker; split to issue 5288
date: 2026-05-07
```

Remaining risks:

- Static member comment emit behavior has not been reached yet; after issue
  5271 advances this path, a later blocker may need separate triage.
