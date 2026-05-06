---
id: 1150
title: "Implement Circularbaseconstraint"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: [5238]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---

## Summary

Triage circularBaseConstraint across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 1 case failing in directory `circularBaseConstraint` with diagnostics: unknown-unsupported. Fresh triage classifies the current failure as name-resolution on the template interpolation identifier `a`.

Problem: `circularBaseConstraint` is not a standalone circular type-system implementation order. The first current blocker is a template interpolation span bug: the unresolved `a` diagnostic is correct in kind but points at synthetic span `0..1`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularBaseConstraint.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularBaseConstraint.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularBaseConstraint.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularBaseConstraint.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5238-preserve-template-interpolation-expression-spans.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularBaseConstraint.ts`

## Duplicate detection

Fresh duplicate scan found broad name-resolution issues and the completed
general name-resolution contract, but no exact open issue for template
interpolation expression spans:

- `issues/done/056-implement-name-resolution.md` owns the general rule that
  genuinely unresolved identifiers report `UnresolvedName`.
- The current unresolved `a` is oracle-matching in diagnostic kind, but its
  compiler span is synthetic `0..1`, unlike TypeScript's source location inside
  the template interpolation.
- No open issue was found for offsetting spans from the temporary template
  interpolation parser.

Split result:

- `issues/open/5238-preserve-template-interpolation-expression-spans.md`

## Smart triage

Fresh triage shows the TypeScript-only aliases are erased and the compiler
reaches the function body. The current failure is the unresolved interpolation
identifier `a`; TypeScript also reports TS2304 for `a`, but the compiler span
is wrong.

### Smart triage: circularBaseConstraint

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Current compiler message: `unresolved name: a at 0..1`
- Path: `reference/typescript/tests/cases/compiler/circularBaseConstraint.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularBaseConstraint.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularBaseConstraint.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Source context:

```ts
type B<T> = T extends any[]
    ? never
    : A<T> extends infer key
    ? key extends keyof T
        ? B<T[key]>
        : never
    : never;

function foo<T>() {
    `${a}` as B<T>;
}
```

Compiler evidence:

```text
tokens: ok; includes conditional type aliases, template literal, and `as B<T>`
ast: ok; Function foo body contains Binary(String(""), Add, Ident("a" span=0..1))
resolved: UnresolvedName unresolved name `a` at 0..1
TypeScript oracle: TS2304 Cannot find name 'a' at line 18, character 8
```

Split result:

- `issues/open/5238-preserve-template-interpolation-expression-spans.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularBaseConstraint.ts --detail --no-dashboard-data
result: pass; reproduced current UnresolvedName/name-resolution bucket
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularBaseConstraint.ts
result: pass; reproduced template interpolation unresolved-name span bug and split to issue 5238
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5238 should leave the oracle-matching `UnresolvedName` diagnostic in place unless broader TypeScript semantic diagnostics are later enabled.
- Conditional type-system semantics remain out of scope for this first blocker.
