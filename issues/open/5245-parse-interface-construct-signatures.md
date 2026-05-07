---
id: 5245
title: "Parse interface construct signatures"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Parse and erase TypeScript construct signatures inside interfaces, such as
`new (props: P, context?: any): Component<P>;`.

## Problem

`circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts`
fails while parsing an interface member:

```ts
interface ComponentClass<P = {}> {
    new (props: P, context?: any): Component<P>;
}
```

The parser reaches the closing `>` in `Component<P>` and then reports:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Greater, ... })
```

Problem: interface construct signatures are parsed as if they were runtime
expressions instead of erasable TypeScript type members.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Greater, span: Span { start: 247, end: 248 } }) at 249..250
```

Representative source:

```ts
declare class Component<P> {}

interface ComponentClass<P = {}> {
    new (props: P, context?: any): Component<P>;
    propTypes?: WeakValidationMap<P>;
}
```

Triage evidence:

- Tokens succeed through `interface ComponentClass<P = {}>`.
- Failure occurs on the construct signature return type `Component<P>`.
- TypeScript AST accepts this as an interface `ConstructSignature`.
- TypeScript oracle reaches the later mapped/conditional type semantic
  diagnostic `TS2344`, proving the construct-signature syntax is valid.

## Desired final state

The parser consumes interface construct signatures as erasable TypeScript type
members, preserving subsequent runtime declarations and allowing the
representative case to advance to the next semantic blocker.

## Scope

In scope:

- [ ] Parse `new (...): Type;` members inside interfaces.
- [ ] Support optional parameters inside construct signatures.
- [ ] Skip return types with generic type arguments such as `Component<P>`.
- [ ] Preserve existing interface erasure behavior for property and method
  members.

Out of scope:

- Runtime construct-signature behavior.
- Type checking construct signatures.
- Object type literal construct signatures outside interfaces unless needed by
  the focused parser helper.
- Mapped/conditional type semantics later in the reference file.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- runtime semantics

## Acceptance criteria

- [ ] `circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts`
  no longer reports the current `unsupported expression ... Greater`
  diagnostic at the interface construct signature.
- [ ] A focused parser or CLI fixture covers `interface C { new (x?: T): U<T>; }`.
- [ ] Existing interface-erasure fixtures still pass.
- [ ] Later unsupported type-system diagnostics remain source-spanned if this
  case advances to mapped/conditional types.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli -E 'test(interface) | test(parser)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1168-implement-circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.md`.

## Completion evidence

Fill when implemented.
