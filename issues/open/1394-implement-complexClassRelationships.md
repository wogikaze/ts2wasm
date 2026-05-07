---
id: 1394
title: "Implement Complexclassrelationships"
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
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1394.

## Summary

Closed as superseded by
`issues/open/5275-parse-modified-static-class-methods.md`.

Fresh focused triage shows `complexClassRelationships.ts` currently stops at
the same modified static method parser boundary already owned by issue 5275:
`public static createEmpty(): Derived`.

## Problem

Reference test results originally showed 1 case failing in directory
`complexClassRelationships` with diagnostics: parser-syntax. Fresh focused
triage on 2026-05-07 reports `UnsupportedSyntax` at the method name
`createEmpty` after the parser consumes `public static`.

Problem: the class member parser expects a parameter list immediately after the
`static` token and rejects the actual method identifier in
`public static createEmpty(): Derived`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complexClassRelationships.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complexClassRelationships.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
complexClassRelationships.ts: UnsupportedSyntax
expected LeftParen, got Some(Ident("createEmpty")) at 113..124
coverage: executed=1, build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

TypeScript oracle evidence:

```text
TypeScript parses the method declaration and reports later semantic diagnostics:
- TS2449: Class 'Base' used before its declaration.
- TS2564: Property 'ownerCollection' has no initializer.
- TS2322: Type 'null' is not assignable to type 'ComponentCollection<any>'.
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5275,
which owns parsing `public static name(...) { ... }` class method declarations.

After issue 5275 lands, this reference path may need fresh triage for class
heritage, strict property initialization, getter return diagnostics, or later
class runtime behavior.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5275's modified static method parser work
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Class heritage semantics after parser support
- Strict property initialization diagnostics
- Getter return type diagnostics

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
- [x] Existing issue 5275 covers `public static name(...)` class method parsing
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complexClassRelationships.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complexClassRelationships.ts
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

- `reference/typescript/tests/cases/compiler/complexClassRelationships.ts`

## Duplicate detection

- `issues/open/5275-parse-modified-static-class-methods.md` owns parsing
  `public static name(...) { ... }` class methods after TypeScript
  accessibility and `static` modifiers.
- `issues/open/5270-parse-modified-class-accessor-declarations.md` is related
  but owns `public static get name()`.
- `issues/open/5271-parse-modified-static-class-fields.md` and
  `issues/open/5288-parse-typed-modified-static-class-fields.md` are related
  but own static field declarations, not methods.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage parser syntax: complexClassRelationships

- Issue class: triage-needed
- Feature label: parser-syntax
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/complexClassRelationships.ts
```

Source context:

```text
class Derived extends Base {
    public static createEmpty(): Derived {
        var item = new Derived();
        return item;
    }
}
```

Parser evidence:

```text
tokens: ok; `public`, `static`, `createEmpty`, `(`, `)`, `:`, `Derived`
ast/resolved: expected LeftParen, got Some(Ident("createEmpty")) at 113..124
```

TypeScript AST evidence:

```text
ClassDeclaration Derived -> MethodDeclaration
text: public static createEmpty(): Derived { ... }
Identifier: createEmpty
```

## Completion evidence

Commits:

- superseded by `issues/open/5275-parse-modified-static-class-methods.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complexClassRelationships.ts
result: pass; reproduced modified static method parser failure at `createEmpty`
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complexClassRelationships.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07
```

Remaining risks:

- Later class semantic diagnostics will only become visible after issue 5275
  advances the parser past the first modified static method.
