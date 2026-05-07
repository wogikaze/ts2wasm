---
id: 5380
title: "Report array literal index-signature element mismatch"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report a source-spanned type diagnostic when an array literal assigned to an
interface with a numeric index signature contains an element that violates the
index value type.

## Problem

Problem: array literals assigned to numeric-index-signature interfaces can skip the invalid element diagnostic and fall through to generic issue-211 method receiver lowering.

`contextualTypingOfArrayLiterals1.ts` parses successfully, but the compiler
erases the interface/index-signature annotation and later reaches the generic
method-call lowering fallback:

```text
UnsupportedSyntax: issue-211: unknown receiver class for method `getDate` at 131..143
```

TypeScript reports the earlier semantic diagnostic instead:

```text
TS2322: Type 'number' is not assignable to type 'Date'.
```

The diagnostic is at the `1` element in:

```ts
interface I {
   [x: number]: Date;
}

var x3: I = [new Date(), 1];
var r2 = x3[1];
r2.getDate();
```

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfArrayLiterals1.ts
```

Current compiler evidence:

```text
tokens: ok
AST: ok; interface is erased, x3 is an Array literal with new Date() and 1
resolved/lowered: issue-211 unknown receiver class for method `getDate`
TypeScript oracle: TS2322 at the numeric literal `1`
```

## Desired final state

The compiler classifies the annotated array literal before the generic
`issue-211` receiver fallback. The representative path should report a
source-spanned type diagnostic for the invalid numeric element or advance to a
more specific semantic blocker; it should not stop at `r2.getDate()`.

## Scope

In scope:

- [ ] Preserve enough numeric index-signature annotation information for
  `var x3: I = [...]` where `I` declares `[x: number]: Date`.
- [ ] Check array literal elements against the numeric index-signature value
  type for this focused assignment shape.
- [ ] Report the invalid `1` element before lowering reaches `r2.getDate()`.
- [ ] Add focused coverage for an array literal assigned to an interface with a
  numeric index signature.

Out of scope:

- Full TypeScript assignability for arbitrary interfaces.
- Runtime support for `Date.prototype.getDate`.
- General interface-typed method receiver support; see
  `issues/done/5222-parse-ambient-generic-variable-type-annotations.md`.
- Array callback receiver typing; see
  `issues/done/5234-w0-implement-host-deny-and-auditable-e2e-manifest-verification.md`.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- focused fixtures or reference assertions

Do not touch:

- `crates/backend-wasm/` unless a focused test proves lowering needs a local
  diagnostic hook there.
- Date runtime builtins.

## Acceptance criteria

- [ ] `contextualTypingOfArrayLiterals1.ts` no longer reports
  `issue-211: unknown receiver class for method getDate`.
- [ ] The invalid array literal element `1` is diagnosed at or before the
  `var x3: I = [new Date(), 1]` assignment.
- [ ] A focused test covers `interface I { [x: number]: Date }` with an array
  literal containing a non-`Date` element.
- [ ] Existing array literal parsing/lowering fixtures still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(array) or test(index) or test(type)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfArrayLiterals1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfArrayLiterals1.ts --detail --no-dashboard-data
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
`issues/open/1524-implement-contextualTypingOfArrayLiterals.md`.

Related but distinct:

- `issues/done/5222-parse-ambient-generic-variable-type-annotations.md`
  owns method calls through erased interface-typed locals such as
  `s.groupBy(...)`.
- `issues/done/5234-w0-implement-host-deny-and-auditable-e2e-manifest-verification.md`
  owns array-shaped parameter annotations for callback methods such as
  `x.forEach(...)`.

## Completion evidence

Fill when implemented.
