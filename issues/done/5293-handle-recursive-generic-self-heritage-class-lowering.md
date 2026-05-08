---
id: 5293
title: "Handle recursive generic self-heritage class lowering"
type: bug
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Handle the TypeScript reference case where a generic class extends an
instantiation of itself, as in `class S18<B, A, C> extends S18<A[], ...>`.

The current parser reaches AST, but `lower_program` stops with an opaque
`Unknown` / `unknown` reference-triage diagnostic and no span or message.

## Problem

`complicatedGenericRecursiveBaseClassReference.ts` is no longer a broad
generated bucket: focused triage shows one concrete lowering/semantic boundary.
Tokens and AST are produced, visible symbols include class `S18`, and TypeScript
reports TS2506 for a direct or indirect self-reference in the base expression.
The ts2wasm runner instead marks the file `blocked` with an empty diagnostic
after `lower_program`.

Problem: recursive generic self-heritage classes fail with an opaque
`Unknown` lower-program blocker instead of a supported build result or a
specific unsupported diagnostic.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
Diagnostic: Unknown / unknown
message: ""
feature_label: type-system
resolved: ok=False
pipeline: validate_ast -> module_graph -> resolve_names -> resolve_builtins -> build_typed_ir -> lower_program
coverage: executed=1, build_pass=0, blocked=1, unsupported=0
```

Source:

```ts
// @target: es2015
class S18<B, A, C> extends S18<A[], { S19: A; (): A }[], C[]>
{
}
(new S18(123)).S18 = 0;
```

Current AST evidence:

```text
ClassDecl name="S18" extends=Ident("S18") body=[]
Expr PropertyAssign object=New Ident("S18") args=[123] property="S18" value=0
```

TypeScript oracle evidence:

```text
TS2506: 'S18' is referenced directly or indirectly in its own base expression.
TS2554: Expected 0 arguments, but got 1.
TS2339: Property 'S18' does not exist on type 'S18<unknown, unknown, unknown>'.
```

## Desired final state

The representative reference file no longer reports an opaque
`Unknown` / `unknown` lower-program blocker. The compiler either advances this
class heritage form through the current class-lowering pipeline or emits a
specific unsupported diagnostic with a span on the recursive class heritage.

## Scope

In scope:

- [x] Identify the `lower_program` failure path for a class whose heritage
      expression resolves to the class currently being declared.
- [x] Preserve current parsing of generic type arguments in the heritage clause;
      this issue starts after AST is available.
- [x] Replace the empty `Unknown` blocker with either successful lowering or a
      named unsupported diagnostic such as `recursive-class-heritage`.
- [x] Add a focused regression fixture or reference assertion for the exact
      `complicatedGenericRecursiveBaseClassReference.ts` shape.

Out of scope:

- Full TypeScript semantic parity for TS2506, TS2554, or TS2339.
- Generic type argument preservation at runtime; TypeScript erases these in the
      emitted JavaScript.
- Broader mutually recursive interface/class hierarchy checking.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- focused compiler/frontend tests or fixtures

Do not touch:

- `crates/backend-wasm/` unless triage after the frontend/lowering fix proves a
  backend-only blocker remains.

## Acceptance criteria

- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts` no longer reports `Diagnostic: Unknown / unknown`.
- [x] Focused coverage for the same path no longer reports `blocked=1` with an empty diagnostic.
- [x] A regression test or fixture covers `class S18<B, A, C> extends S18<A[], ...>`.
- [x] If the final behavior is an unsupported diagnostic, it has a stable diagnostic code/name and a span on the recursive heritage clause.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complicatedGenericRecursiveBaseClassReference.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from generated bucket
`issues/open/1397-implement-complicatedGenericRecursiveBaseClassReference.md`.
Related parser issue 5156 covers heritage type-argument parsing failures; this
case already reaches AST and fails later.

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

- Supporting the current lowering boundary may expose later runtime behavior for
  JavaScript's `class S18 extends S18 {}` temporal-dead-zone failure.
