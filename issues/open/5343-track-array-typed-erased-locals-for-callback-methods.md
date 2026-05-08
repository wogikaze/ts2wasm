---
id: 5343
title: "Track array-typed erased locals for callback methods"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Preserve enough TypeScript array type annotation information on declaration-only
locals for supported array callback methods such as `s.map(...)` to use the
existing known-array receiver path, or to report a precise definite-assignment
diagnostic before the generic method-receiver fallback.

## Problem

`commentInMethodCall.ts` tokenizes and parses successfully, including the
comment inside the call argument list. The current failure is not comment
handling. Lowering rejects `s.map(function () { })` because `var s: string[];`
is erased to `Undefined` and not tracked as an array-shaped local receiver.

Current diagnostic:

```text
UnsupportedSyntax: issue-211: unknown receiver class for method `map` at 55..98
```

TypeScript accepts the syntax and reports the semantic definite-assignment
diagnostic:

```text
TS2454: Variable 's' is used before being assigned.
```

Problem: declaration-only array locals such as `var s: string[];` lose their
array-shaped annotation before lowering, so `s.map(...)` reports generic
`issue-211` unknown receiver class instead of using array callback receiver
handling or a TypeScript-aligned definite-assignment diagnostic.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentInMethodCall.ts
```

Source context:

```ts
// @target: es2015
//commment here
var s: string[];
s.map(// do something
    function () { });
```

Compiler evidence observed 2026-05-07:

```text
tokens: ok; comments are skipped and the stream includes var s: string[]; followed by s.map(function () { })
ast: ok; Let s = Undefined, Expr Call(Member(Ident("s"), "map"), args=[FunctionExpr {}])
resolved/lowered: UnsupportedSyntax issue-211 unknown receiver class for method map at 55..98
visible symbol: binding s at line 3, column 1
TypeScript oracle: TS2454 at line 4, column 1; binding s has typeText string[]
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentInMethodCall.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

## Desired final state

The frontend/resolver records `T[]` and `Array<T>` annotations on
declaration-only locals sufficiently for supported array callback methods on
those locals to avoid the generic unknown receiver diagnostic. The
representative case either reaches the existing supported array callback path
or reports a source-spanned definite-assignment diagnostic for `s` before
generic issue-211 fallback.

## Scope

In scope:

- [ ] Preserve `T[]` and `Array<T>` local variable annotation metadata for declaration-only locals.
- [ ] Mark declaration-only locals with array-shaped annotations as known array locals where existing array callback method lowering can use them, or emit a source-spanned definite-assignment diagnostic before generic issue-211.
- [ ] Add a focused fixture for `var s: string[]; s.map(function () { })`.
- [ ] Re-run the representative reference triage and record the next diagnostic.

Out of scope:

- Array-typed parameters, tracked by `issues/done/5234-w0-implement-host-deny-and-auditable-e2e-manifest-verification.md`.
- Interface-typed erased locals, tracked by `issues/done/5222-parse-ambient-generic-variable-type-annotations.md`.
- Runtime implementation for arbitrary uninitialized arrays.
- Comment emit preservation; the current lexer/parser path already skips the comments and builds the call AST.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/lowered/`
- focused parser/IR/CLI tests

Do not touch:

- backend/runtime Array method implementations unless receiver tracking advances into an existing supported callback path that exposes a real backend/runtime gap
- unrelated interface or class method receiver support

## Acceptance criteria

- [ ] `var s: string[]; s.map(function () { })` no longer reports `issue-211: unknown receiver class for method map`.
- [ ] Declaration-only locals annotated as `T[]` and `Array<T>` are classified before the generic unknown-receiver fallback.
- [ ] Untyped declaration-only locals and non-array annotations continue to avoid being treated as arrays.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentInMethodCall.ts` advances past the current `map` receiver boundary or reports a TypeScript-aligned definite-assignment diagnostic for `s`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-ir
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentInMethodCall.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentInMethodCall.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from `issues/done/1342-implement-commentInMethodCall.md`.

Related but not duplicate:

- `issues/done/5234-w0-implement-host-deny-and-auditable-e2e-manifest-verification.md` covers array-shaped parameter annotations, not declaration-only locals.
- `issues/done/5222-parse-ambient-generic-variable-type-annotations.md` covers interface-typed erased locals such as `Sequence<string>`, not array callback receivers.
- `issues/done/297-track-pushed-dense-array-locals-for-map.md` covers initialized dense arrays built through pushes, not erased declaration-only locals.
- `issues/open/435-implement-method-call.md` is the broad method-call bucket; this issue owns the narrow array-typed local receiver evidence.

## Completion evidence

Fill when implemented.
