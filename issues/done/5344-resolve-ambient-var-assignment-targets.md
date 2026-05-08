---
id: 5344
title: "Resolve ambient var assignment targets"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Preserve declaration-only ambient `var` bindings well enough for assignment
targets such as `x = 2` to resolve without emitting a runtime declaration.

## Problem

`commentOnAmbientVariable2.ts` tokenizes and parses successfully, but the
parser erases `declare var x: number;` from the runtime AST. Name resolution
then sees only `var y = 1;` and `x = 2;`, and reports `UnresolvedName` for the
assignment target `x`.

Problem: declaration-only ambient variable assignment targets are not
resolver-visible after ambient erasure.

Current diagnostic:

```text
UnresolvedName: unresolved name: `x` at 206..212
```

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnAmbientVariable2.ts
```

Source context:

```ts
// @Filename: commentOnAmbientVariable2_1.ts
var y = 1;

// @Filename: commentOnAmbientVariable2_2.ts
/// <reference path='commentOnAmbientVariable2_1.ts'/>
declare var x: number;
x = 2;
```

Compiler evidence observed 2026-05-07:

```text
tokens: ok through var y = 1, declare var x: number, x = 2
ast: ok; Let y = 1, Assign x = 2
resolved: fail in resolve_names with UnresolvedName for x
visible symbols: y only
TypeScript oracle: ok, diagnostics=[], hints y:number and x:number
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnAmbientVariable2.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
semantic_enabled=0
```

## Desired final state

Declaration-only ambient `var` bindings are visible to name resolution as
ambient values when they are used as assignment targets. The representative
path should no longer fail at `x = 2`; it should either build or advance to the
next narrower diagnostic without emitting a runtime declaration for `x`.

## Scope

In scope:

- [x] Preserve resolver-visible metadata for declaration-only ambient `var` declarations used as assignment targets.
- [x] Resolve `declare var x: number; x = 2;` without emitting a runtime declaration for `x`.
- [x] Add focused resolver coverage for an ambient `var` assignment target.
- [x] Re-run `commentOnAmbientVariable2.ts` and record the next diagnostic if the file advances.

Out of scope:

- General expression references to ambient values, tracked by `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`.
- Full runtime implementation or initialization of ambient values.
- Ambient declarations with initializers; those must remain rejected at the existing unsupported boundary.
- TypeScript semantic assignment compatibility checks after the name resolves.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/ast.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/name_resolver_tests.rs`

Do not touch:

- backend emit or runtime ABI unless triage proves the unresolved-name blocker has advanced to backend emission

## Acceptance criteria

- [x] `declare var x: number; x = 2;` no longer reports `UnresolvedName` for `x`.
- [x] `contextualSignatureInstatiationContravariance.ts` no longer reports
  `UnresolvedName` for ambient assignment target `g2` in `g2 = f2`.
- [x] A focused resolver test proves the ambient `var` binding is resolver-visible for assignment targets without adding a runtime local.
- [x] Existing ambient value expression cases in issue 5161 remain unchanged or are explicitly advanced by the same implementation.
- [x] Ambient declarations with initializers, such as `declare var x = 1;`, remain rejected.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnAmbientVariable2.ts` advances past the current `UnresolvedName` for `x`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-ir
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnAmbientVariable2.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnAmbientVariable2.ts --detail --no-dashboard-data
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

Split from `issues/open/1346-implement-commentOnAmbientVariable.md`.

Related but not duplicate:

- `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`
  covers ambient values referenced in expression positions such as array
  literals, object literal shorthand, and method receivers. This issue owns the
  assignment target form from `commentOnAmbientVariable2.ts`.
- `issues/done/5193-parse-asi-after-ambient-variable-declarations.md` covers
  parser ASI after ambient variable declarations, not name resolution after
  successful parse.

2026-05-07 fold-in:

- `issues/open/1504-implement-contextualSignatureInstatiationContravariance.md`
  reaches the same ambient assignment-target resolver boundary for
  `declare var g2: ...; g2 = f2;`.
- Current diagnostic: `UnresolvedName: unresolved name: \`g2\`` at the assignment.
- TypeScript oracle reports the later TS2322 contravariance diagnostic once the
  ambient assignment target resolves.

## Completion evidence

Fill when implemented.
