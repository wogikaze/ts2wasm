---
id: 1370
title: "Implement Commentsinterface"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: [5222]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1370.

## Summary

Closed as superseded by the existing implementation-ready issue 5222, which
already tracks interface-typed method calls on erased locals.

## Problem

Reference test results originally showed 1 case failing in directory
`commentsInterface` with diagnostics: class. Fresh focused triage on 2026-05-07
shows the parser and AST advance through the interface declarations and uses;
lowering rejects an interface-typed receiver method call with the generic
issue-211 diagnostic.

Problem: `commentsInterface.ts` is blocked by `i2_i.foo(30)` where `i2_i` is an
erased local annotated with interface type `i2`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsInterface.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsInterface.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Smart triage reports:

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `foo` at 885..897
```

## Desired final state

This generated bucket is superseded by
`issues/done/5222-parse-ambient-generic-variable-type-annotations.md`.
Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with an existing implementation-ready issue
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
- [x] Superseding issue contains an exact `reference-triage` command
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsInterface.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsInterface.ts
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

- [x] superseded by: `issues/done/5222-parse-ambient-generic-variable-type-annotations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsInterface.ts`

## Duplicate detection

`issues/done/5222-parse-ambient-generic-variable-type-annotations.md` is
an exact behavioral match. Its representative case is `var s: Sequence<string>;
s.groupBy(...)`; this bucket's representative shape is the same unsupported
receiver family:

```ts
interface i2 {
    foo: (b: number) => string;
}
var i2_i: i2;
var i2_i_foo_r = i2_i.foo(30);
```

Both fail after AST success with `issue-211: unknown receiver class for method`
because the receiver is an erased interface-typed local rather than a known
runtime class instance.

Other candidates are related but not exact:

- `issues/done/211-complete-this-receiver-binding-semantics.md` completed
  receiver-bound `this` semantics and explicitly left unsupported receiver
  forms as issue-linked diagnostics.
- `issues/open/435-implement-method-call.md` is broader method-call support and
  should not duplicate the narrower interface-typed receiver issue.
- `issues/done/5217-preserve-ambient-value-declarations-through-name-resolution.md` covers
  call-expression receivers, not identifier locals annotated with interface
  types.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage class: commentsInterface

- Issue class: triage-needed
- Feature label: class
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/commentsInterface.ts
```

Source context:

```text
34 | var i2_i_x = i2_i.x;
35 | var i2_i_foo = i2_i.foo;
36 | var i2_i_foo_r = i2_i.foo(30);
37 | var i2_i_i2_si = i2_i["hello"];
38 | var i2_i_i2_ii = i2_i[30];
39 | var i2_i_n = new i2_i(i1_i);
40 | var i2_i_nc_x = i2_i.nc_x;
```

Compiler evidence:

```text
tokens: ok through interfaces, method signatures, index signatures, and uses
ast: ok; Let i2_i_foo_r = Call(Member(Ident("i2_i"), "foo"), [30])
resolved/lowered: issue-211 unknown receiver class for method `foo`
```

Visible symbols before failure include `i1_i`, `nc_i1_i`, `i2_i`, `i2_i_x`,
`i2_i_foo`, `i2_i_foo_r`, and `i2_i_i2_si`.

TypeScript oracle:

```text
The source parses and types interface members. Oracle diagnostics are TS2454
used-before-assigned diagnostics for i2_i and i1_i, not receiver-class
diagnostics.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsInterface.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsInterface.ts
result: issue-211 unknown receiver class for method `foo`; superseded by issue 5222
date: 2026-05-07
```

Remaining risks:

- Issue 5222 must still decide whether to support the interface method-call
  shape or replace the generic issue-211 fallthrough with a precise
  source-spanned diagnostic.
