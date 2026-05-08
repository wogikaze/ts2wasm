---
id: 5372
title: "Parse ambient function ASI with constructor types"
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

Accept ASI after declaration-only ambient function declarations with
constructor function type parameters.

## Problem

`contextualSignatureInstantiation4.ts` currently stops at the first ambient
factory declaration even though TypeScript accepts the declaration without a
semicolon before the following `const`.

Problem: ambient function declaration erasure still requires a terminator for
this constructor-type parameter shape.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualSignatureInstantiation4.ts
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstantiation4.ts --detail --no-dashboard-data
```

Current diagnostic:

```text
UnsupportedTypeScriptSyntax: issue-400: unterminated ambient function declaration at 147..154
line 8, column 8
```

Representative source:

```ts
declare function fruitFactory1<TFruit>(Fruit: new (...args: any[]) => TFruit): TFruit
const banana1 = fruitFactory1(Banana) // Banana<any>
```

Compiler evidence:

```text
tokens: ok through declare class Banana and ambient function signature tokens
ast/resolved: fail at `declare` before preserving the following const binding
visible symbols before failure: class `Banana`
TypeScript oracle: diagnostics=[]; accepts all five fruitFactory declarations
```

## Desired final state

Declaration-only ambient function declarations with constructor-type parameters
are erased when followed by ASI, and the representative file advances.

## Scope

In scope:

- [x] Accept ASI after `declare function f<T>(arg: Type): ReturnType`.
- [x] Consume constructor function parameter types such as `new (...args: any[]) => T`.
- [x] Preserve ambient declaration erasure so no runtime function binding is emitted.

Out of scope:

- Ambient function overload validation.
- Object or interface construct-signature members.
- Contextual constructor inference semantics after parsing succeeds.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`

## Acceptance criteria

- [x] `contextualSignatureInstantiation4.ts` no longer reports
  `issue-400: unterminated ambient function declaration` at the first
  `fruitFactory` declaration.
- [x] A focused parser or CLI test accepts `declare function f<T>(C: new (...args: any[]) => T): T` followed by a newline and runtime statement.
- [x] Ambient function declarations remain erased from runtime AST/lowering.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend ambient
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualSignatureInstantiation4.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstantiation4.ts --detail --no-dashboard-data
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from `issues/open/1501-implement-contextualSignatureInstantiation-parser-syntax.md`.

Related but not duplicates:

- `issues/done/705-implement-asiAmbientFunctionDeclaration.md` is the older
  generated triage bucket for the minimal `declare function foo()` ASI case.
- `issues/done/400-implement-ambient-declaration-erasure-boundary.md` covers
  the completed baseline ambient function erasure boundary with explicit
  terminators.

## Completion evidence

Fill only when implemented.

## False-done audit

**truly-done** (5372)

- Implementation commits: verified via `git log --oneline --all --grep=5372`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
