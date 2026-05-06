---
id: 5309
title: "Skip generic type arguments in type annotations"
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

Teach the shared TypeScript type-annotation skipper to treat `<...>` generic
type argument lists as nested type syntax, so commas inside generic type
arguments do not terminate function parameter annotations.

## Problem

`consistentAliasVsNonAliasRecordBehavior.ts` parses the leading mapped type
alias, then fails on the first function parameter annotation
`Record<'a', string>`. The skipper stops at the comma inside the generic type
argument list and the parameter parser then expects a parameter separator before
seeing the closing `>`.

Problem: function parameter annotations such as `x: Record<'a', string>` fail with `expected Comma, got Some(Greater)`.

## Current failure

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/consistentAliasVsNonAliasRecordBehavior.ts
```

Current diagnostic:

```text
UnsupportedSyntax: expected Comma, got Some(Greater) at 355..356
```

Source context:

```text
function defaultRecord(x: Record<'a', string>, y: Record<string, string>) {
    x = y;
}
```

TypeScript oracle parses this source and later reports type-system diagnostics:

```text
TS2741: Property 'a' is missing in type 'Record<string, string>' but required in type 'Record2<"a", string>'.
```

## Desired final state

The parser erases the full generic type annotation, including nested commas
inside `<...>`, and resumes parameter parsing at the comma between `x` and `y`.

## Scope

In scope:

- [ ] Track angle-bracket depth in `skip_type_annotation_until`.
- [ ] Preserve existing stop behavior for true parameter separators outside generic type arguments.
- [ ] Add focused parser coverage for a parameter annotation like `Record<'a', string>`.

Out of scope:

- Type alias runtime semantics.
- Type-system diagnostics for `Record` variance/assignability.
- Expression-level generic call type arguments.

## Affected paths

Expected:

- `crates/frontend/src/parser/tokens.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- backend/runtime lowering

## Acceptance criteria

- [ ] `consistentAliasVsNonAliasRecordBehavior.ts` no longer reports `expected Comma, got Some(Greater)` at `Record<'a', string>`.
- [ ] A focused parser test accepts `function f(x: Record<'a', string>, y: Record<string, string>) {}`.
- [ ] Existing typed parameter parsing without generic arguments still passes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/consistentAliasVsNonAliasRecordBehavior.ts
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
`issues/done/1438-implement-consistentAliasVsNonAliasRecordBehavior.md`.

The same reference case may expose type-system diagnostics after this parser
boundary is fixed.

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
