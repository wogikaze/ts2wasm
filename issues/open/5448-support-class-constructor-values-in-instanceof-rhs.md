---
id: 5448
title: "Support class constructor values in instanceof RHS"
type: feature
area: ir/runtime
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Allow direct class constructor bindings to be used as the right-hand side of
`instanceof` expressions.

Split from generated bucket `issues/open/3448-implement-narrowTypeByInstanceof.md`.

## Problem

Problem: `narrowTypeByInstanceof.ts` parses successfully, but name resolution
rejects `FileMatch` in `elementA instanceof FileMatch` with `issue-5011`.

The parser already builds `ClassDecl` nodes, the `FileMatchOrMatch` type alias,
the multi-variable declaration, `instanceof` expressions, and chained member
calls. The current blocker is the class constructor binding used as an
expression value in an `instanceof` RHS.

Current diagnostic:

```text
UnsupportedSyntax: issue-5011: class `FileMatch` cannot be used as a value - class runtime is not yet supported at 367..376
```

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowTypeByInstanceof.ts
```

Representative source:

```ts
class Match {
    public range(): any {
        return undefined;
    }
}

class FileMatch {
    public resource(): any {
        return undefined;
    }
}

type FileMatchOrMatch = FileMatch | Match;

let elementA: FileMatchOrMatch, elementB: FileMatchOrMatch;

if (elementA instanceof FileMatch && elementB instanceof FileMatch) {
    let a = elementA.resource().path;
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; class declarations, type alias, let declaration, instanceof expressions, and chained member calls parse
resolved: issue-5011 at `FileMatch` used as the first `instanceof` RHS
TypeScript oracle: parses the class RHS shape and reports later TS2454 definite-assignment diagnostics for elementA/elementB
```

## Desired final state

Direct class constructor bindings can be represented as runtime constructor
values when they appear as `instanceof` RHS operands. The representative path
should advance beyond the current `issue-5011` diagnostic.

## Scope

In scope:

- [ ] Resolve a direct class declaration binding as a supported constructor
  value when used as an `instanceof` RHS.
- [ ] Preserve the existing unsupported diagnostic for class value positions
  outside this direct `instanceof` RHS slice.
- [ ] Add focused coverage for `value instanceof C` where `C` is a class
  declaration in scope.
- [ ] Re-run the representative reference triage and record the next blocker.

Out of scope:

- General first-class class constructor value flow through aliases or function
  arguments, tracked by `issues/open/5192-support-first-class-class-constructor-values.md`.
- Callable/prototype object `instanceof` RHS support, tracked by
  `issues/open/5447-support-instanceof-callable-prototype-rhs.md`.
- Full TypeScript `instanceof` narrowing semantics after the RHS resolves.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused compiler fixtures or tests

Do not touch:

- unrelated parser syntax
- broad class alias/runtime factory flows owned by issue 5192

## Acceptance criteria

- [ ] `narrowTypeByInstanceof.ts` no longer reports `issue-5011` for
  `FileMatch` in `elementA instanceof FileMatch`.
- [ ] A focused fixture covers `class C {}; if (x instanceof C) { ... }`.
- [ ] Existing `issue-5011` diagnostics for non-`instanceof` class value uses
  remain source-spanned.
- [ ] Any later narrowing or definite-assignment diagnostic from the
  representative path is recorded here or split into a follow-up issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(instanceof) or test(class)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowTypeByInstanceof.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowTypeByInstanceof.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Related but distinct:

- `issues/open/5192-support-first-class-class-constructor-values.md` owns
  constructor values flowing through ordinary expressions, such as factory
  arguments.
- `issues/open/5447-support-instanceof-callable-prototype-rhs.md` owns
  non-class callable/prototype RHS values that currently report `issue-207`.

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
