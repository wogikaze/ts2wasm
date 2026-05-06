---
id: 5203
title: "Report indexed new type-only callee diagnostics"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Report a precise TypeScript-style diagnostic for `new any[1]` instead of the
generic `issue-062` new-expression class-name guard.

## Problem

`cannotInvokeNewOnIndexExpression.ts` parses successfully as a `New` expression
whose callee is an `Index(Ident any, Number 1)`. Name resolution then rejects
the non-identifier callee with `issue-062: new requires a class name identifier`
before it can report that `any` is a type-only name used as a value.

Problem: indexed `new` callees that start with type-only identifiers fall into the generic issue-062 class-name requirement.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cannotInvokeNewOnIndexExpression.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-062: new requires a class name identifier at 37..47
```

Representative source:

```ts
var test: any[] = new any[1];
```

Triage evidence:

- Tokens and AST succeed.
- AST contains `Let test = New { expr: Index { object: Ident any, index:
  Number 1 } }`.
- TypeScript oracle reports TS2693: `'any' only refers to a type, but is being
  used as a value here.`

## Desired final state

The resolver classifies the `new any[1]` callee before the generic `issue-062`
new-expression guard and emits a source-spanned diagnostic at `any` for using a
type-only name as a value. Supported `new ClassName(...)` and `new ns.Class(...)`
paths remain unchanged.

## Scope

In scope:

- [ ] Detect `New` expressions whose callee is an indexed expression rooted at
  a type-only identifier
- [ ] Emit a source-spanned type-only value-use diagnostic for `any` in
  `new any[1]`
- [ ] Preserve existing supported class-name and member-name constructor paths
- [ ] Preserve a narrower unsupported diagnostic for genuinely dynamic indexed
  constructor callees

Out of scope:

- Full dynamic constructor indexing support
- Runtime semantics for `new expr[index](...)`
- Namespace/type merge support beyond the representative diagnostic

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- unrelated class runtime emission

## Acceptance criteria

- [ ] `cannotInvokeNewOnIndexExpression.ts` no longer reports generic
  `issue-062` for `new any[1]`
- [ ] A focused fixture covers `var test: any[] = new any[1];`
- [ ] The diagnostic is source-spanned at the type-only `any` identifier
- [ ] Existing `new ClassName(...)` and `new namespace.ClassName(...)` fixtures
  keep passing

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class) | test(new)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cannotInvokeNewOnIndexExpression.ts
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

This is narrower than first-class dynamic constructor support. It only prevents
the generic constructor-callee guard from hiding a known TypeScript type-only
value-use diagnostic.

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
