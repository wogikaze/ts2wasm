---
id: 1339
title: "Implement Commentbeforestaticmethod"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1339.

## Summary

Closed after splitting the current blocker to
`issues/open/5275-parse-modified-static-class-methods.md`. Fresh triage shows
the first failure is a parser gap for `public static foo()`.

## Problem

Reference test results show 1 case failing in directory
`commentBeforeStaticMethod`. Fresh triage confirms tokens succeed, but AST
construction stops after `public static` before the method name `foo`.

Problem: `commentBeforeStaticMethod1.ts` reports `expected LeftParen, got
Some(Ident("foo"))` at `public static foo(): string`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentBeforeStaticMethod1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentBeforeStaticMethod1.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm related modified class member issues do not exactly own this method gap
- [x] Split one observable behavior into child issue 5275
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

- [x] Duplicate candidates below are confirmed and the exact behavior is split to 5275
- [x] Child issue 5275 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentBeforeStaticMethod1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentBeforeStaticMethod1.ts
```

Not run:

- `cargo fmt --all --check` (not run; issue metadata only)
- `cargo nextest run` (not run; issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5275-parse-modified-static-class-methods.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentBeforeStaticMethod1.ts`

Source context:

```ts
class C {
  /**
   * Returns bar
   */
  public static foo(): string {
    return "bar";
  }
}
```

## Duplicate detection

- `issues/open/5270-parse-modified-class-accessor-declarations.md` is related
  but owns `public static get name()`.
- `issues/open/5271-parse-modified-static-class-fields.md` is related but owns
  `public static name = expr`.
- `issues/open/5267-parse-string-literal-class-member-names.md` is related but
  owns quoted method names after modifiers.
- No exact implementation-ready issue owned identifier-named modified static
  methods, so this bucket was split to issue 5275.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentBeforeStaticMethod1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentBeforeStaticMethod1.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: expected LeftParen, got Some(Ident("foo")) at 79..82
Source: public static foo(): string {
tokens: ok; public, static, Ident("foo"), LeftParen are present
AST: fails before MethodDeclaration construction
TypeScript oracle: ok, no diagnostics; MethodDeclaration name is foo
Child issue: 5275
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5275-parse-modified-static-class-methods.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentBeforeStaticMethod1.ts
result: pass; reproduced modified static method parser failure and split child issue 5275
date: 2026-05-06
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5275
