---
id: 1380
title: "Implement Commentstypeparameters"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: [5290]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1380.

## Summary

Closed after splitting the executable parser work into
`issues/open/5290-parse-private-static-generic-class-method.md`.

Fresh triage shows the first blocker is not the general type-parameter comment
surface. The parser stops earlier at `private static privatestaticmethod`, the
private static generic method parser family split to issue 5290.

## Problem

Reference test results originally showed 1 case failing in directory
`commentsTypeParameters` with diagnostics: parser-syntax. Fresh focused triage
on 2026-05-07 shows tokenization succeeds, and parsing reaches a private static
generic method declaration before rejecting the method name.

Problem: `commentsTypeParameters.ts` currently cannot parse
`private static privatestaticmethod</**...*/ U>(a: U)`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsTypeParameters.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsTypeParameters.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5290-parse-private-static-generic-class-method.md`; type parameter
comment fidelity should be rechecked after the modified static method parser
blocker advances.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split this bucket into a private static generic method parser issue
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
- [x] Child issue contains the exact `private static name<T>(...)` parser diagnostic family
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsTypeParameters.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsTypeParameters.ts
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

- [x] created/updated: `issues/open/5290-parse-private-static-generic-class-method.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsTypeParameters.ts`

## Duplicate detection

- `issues/open/5290-parse-private-static-generic-class-method.md` owns the
  exact current parser boundary: `private static` followed by a generic
  identifier-named method.
- `issues/open/5275-parse-modified-static-class-methods.md` is related but
  broader; folding this generic/comment reference into it made the issue too
  large for readiness gates.
- Type-parameter comment behavior is adjacent but not yet reached because the
  parser fails before constructing the `private static` method declaration.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage method call: commentsTypeParameters

- Issue class: triage-needed
- Feature label: method-call
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/commentsTypeParameters.ts
```

Source context:

```text
10 |     private privatemethod</**docComment of method type parameter */ U extends T>(a: U) {
11 |     }
12 |     private static privatestaticmethod</**docComment of method type parameter */ U>(a: U) {
13 |     }
14 | }
```

Compiler evidence:

```text
tokens: ok; private, static, Ident("privatestaticmethod"), Less, Ident("U") are present
ast/resolved: expected LeftParen, got Some(Ident("privatestaticmethod")) at 404..423
visible symbols before failure: class C
```

TypeScript oracle:

```text
ok: true
diagnostics: []
AST path: ClassDeclaration -> MethodDeclaration -> Identifier privatestaticmethod
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsTypeParameters.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsTypeParameters.ts
result: private static generic method parser blocker; split to issue 5290
date: 2026-05-07
```

Remaining risks:

- Generic type parameter and comment/declaration emit behavior has not been
  reached yet; after issue 5275 advances this path, a later blocker may need
  separate triage.
