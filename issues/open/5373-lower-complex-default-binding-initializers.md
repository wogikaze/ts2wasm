---
id: 5373
title: "Lower complex default binding initializers"
type: feature
area: ir/lowering
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Implement the next issue-251 destructuring slice for object binding patterns
with non-trivial default initializers, starting with arrow/function parameters
like `({ a = 0 } = {}) => a`.

## Problem

`contextualTypeForInitalizedVariablesFiltersUndefined.ts` parses into arrow
functions with object binding parameters, but name resolution/lowering rejects
the first parameter pattern with the issue-251 runtime subset guard.

Problem: complex default binding initializers in object binding parameters are
not lowered, so reference cases cannot advance to later contextual typing or
undefined-filtering diagnostics.

Current diagnostic:

```text
UnsupportedRuntimeSubset: issue-251: complex default binding initializers are not supported in this runtime slice at 56..77
```

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeForInitalizedVariablesFiltersUndefined.ts
```

Source context:

```ts
const fInferred = ({ a = 0 } = {}) => a;
const fAnnotated: typeof fInferred = ({ a = 0 } = {}) => a;
declare var t: { s: string } | undefined;
const { s } = t;
function fst({ s } = t) { }
```

Compiler evidence observed 2026-05-07:

```text
tokens: ok through arrow parameter object binding defaults and later object destructuring/function parameter default
ast: ok; fInferred/fAnnotated ArrowFn params contain "{a = 0} = {}", later Let "{s}" = t and Function fst param "{s}" default t
resolved: fails in resolve_names with UnsupportedRuntimeSubset issue-251 at the first arrow parameter
TypeScript oracle: reports later TS2339 for destructuring from `{ s: string } | undefined`; fInferred/fAnnotated are typed as `({ a }?: { a?: number | undefined; }) => number`
```

Focused coverage:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeForInitalizedVariablesFiltersUndefined.ts --detail --no-dashboard-data
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

`reference-triage` classifies the same path as `UnsupportedRuntimeSubset` /
`runtime-subset`; this issue owns that implementation boundary.

## Desired final state

Object binding parameters with property defaults and a whole-pattern default are
lowered correctly for the simple literal-default case. The representative path
should advance past the first `({ a = 0 } = {}) => a` issue-251 diagnostic and
either build or expose the next narrower diagnostic.

## Scope

In scope:

- [ ] Lower `({ a = 0 } = {}) => a` without reporting issue-251 for the parameter default.
- [ ] Lower the annotated equivalent `const fAnnotated: typeof fInferred = ({ a = 0 } = {}) => a`.
- [ ] Preserve existing source-spanned issue-251 diagnostics for broader non-literal or effectful defaults that remain out of scope.
- [ ] Add focused IR/name-resolver or CLI fixture coverage for the literal object default plus literal property default pattern.
- [ ] Re-run `contextualTypeForInitalizedVariablesFiltersUndefined.ts` and record the next diagnostic.

Out of scope:

- Full TypeScript contextual typing or TS2339 semantic diagnostics for destructuring from possibly undefined values.
- Broad dynamic-source destructuring semantics beyond the existing issue-251 subset.
- Object rest binding and computed binding aliases; those are tracked by separate issue-251 slices.

## Affected paths

Expected:

- `crates/ir/src/binding_pattern.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/lowered/`
- `crates/cli/tests/common/`
- `fixtures/core-semantics/`

Do not touch:

- backend/runtime ABI unless triage proves the first blocker has advanced to emission/runtime behavior.

## Acceptance criteria

- [ ] `const f = ({ a = 0 } = {}) => a;` no longer reports `issue-251: complex default binding initializers`.
- [ ] The implementation includes a regression test or fixture that proves the parameter default object is applied before binding `a`.
- [ ] Existing unsupported cases for non-literal/effectful default binding initializers still report source-spanned issue-251 diagnostics.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeForInitalizedVariablesFiltersUndefined.ts` advances past the current `56..77` issue-251 diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-cli
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeForInitalizedVariablesFiltersUndefined.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeForInitalizedVariablesFiltersUndefined.ts --detail --no-dashboard-data
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

Split from `issues/done/1510-implement-contextualTypeForInitalizedVariablesFiltersUndefined.md`.
The broader destructuring implementation was completed in issue 251 with this
form deliberately left behind as a source-spanned runtime-subset guard.

## Completion evidence

Fill when implemented.
