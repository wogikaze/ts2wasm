---
id: 5308
title: "Parse ASI after instance class field initializers"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Accept automatic semicolon insertion after initialized instance class fields
when the field initializer ends at a line break before the next class element or
closing brace.

## Problem

`conflictingTypeParameterSymbolTransfer.ts` parses the earlier generic class
declarations, but fails inside `class Foo<t>` after fields without semicolons.
The class member parser enters the initializer expression for `foo = this.t`
and does not stop at the line break / closing brace boundary.

Problem: `conflictingTypeParameterSymbolTransfer.ts` reports `expected property name, got Equal` after `foo = this.t` instead of accepting ASI after the instance field initializer.

## Current failure

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingTypeParameterSymbolTransfer.ts
```

Current diagnostic:

```text
UnsupportedSyntax: expected property name, got Equal at 453..457
```

Source context:

```text
class Foo<t> extends Leg {
    t = {} as t

    // should allow this access since t was declared as a property on Foo
    foo = this.t
}
```

TypeScript oracle parses this class and later reports unrelated semantic
diagnostics:

```text
TS2304: Cannot find name 'U'.
TS2564: Property 'data' has no initializer and is not definitely assigned in the constructor.
```

## Desired final state

The parser treats the line break / class element boundary after an initialized
instance field as ASI, so the representative case advances past the current
`expected property name, got Equal` parser failure.

## Scope

In scope:

- [ ] Stop instance class field initializer parsing at a valid ASI boundary.
- [ ] Cover `field = expr` followed by a later class member without an explicit semicolon.
- [ ] Preserve existing parsing for semicolon-terminated fields and methods.

Out of scope:

- Static class field ASI, tracked by issue 5254.
- TypeScript semantic diagnostics after parsing succeeds.
- Runtime lowering for class field initialization.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- backend/runtime lowering

## Acceptance criteria

- [ ] `conflictingTypeParameterSymbolTransfer.ts` no longer reports `expected property name, got Equal` after `foo = this.t`.
- [ ] A focused parser test accepts `class C { a = 1\nb = this.a\n}`.
- [ ] Existing method parsing for `name()` and semicolon-terminated field parsing remain unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/conflictingTypeParameterSymbolTransfer.ts
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

Split from generated bucket
`issues/done/1437-implement-conflictingTypeParameterSymbolTransfer.md`.

The same parser area has a static-field-only ASI issue, 5254; this issue is for
instance fields with initializers.

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
