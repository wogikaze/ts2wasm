---
id: 5479
title: "Bind DOM worker self/importScripts globals"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Bind the DOM/WebWorker globals needed by JavaScript reference inputs that use
`self.importScripts` and `importScripts(...)`, or report a precise unsupported
worker-global diagnostic before generic name resolution fails.

## Problem

Problem: `reference/typescript/tests/cases/compiler/noParameterReassignmentIIFEAnnotated.ts`
currently reports `UnresolvedName` for the top-level `self` in:

```js
self.importScripts = (function (importScripts) {
    return function () {
        return importScripts.apply(this, arguments);
    };
})(importScripts);
```

This blocks the generated no-parameter-reassignment bucket before it can reach
the intended IIFE parameter reassignment or `arguments` diagnostics.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noParameterReassignmentIIFEAnnotated.ts
```

Current compiler diagnostic:

```text
UnresolvedName: unresolved name: `self` at 100..104
```

Compiler evidence:

- Tokens are ok through `self.importScripts = (...)`.
- AST is ok and represents a top-level `PropertyAssign` whose object is
  `Ident self`.
- Name resolution stops before resolving the IIFE argument `importScripts`.
- TypeScript oracle reports later TS2683 for `this` and TS2345 for
  `arguments`, not an unresolved global diagnostic.

## Desired final state

The compiler no longer reports generic `UnresolvedName` for the worker globals
`self` and `importScripts` in this reference shape. The representative reference
advances to build-pass or to the next precise unsupported boundary.

## Scope

In scope:

- [ ] Treat `self` as a known DOM/WebWorker global for name resolution in
  reference inputs that request browser/worker globals.
- [ ] Treat top-level `importScripts` as a known worker global function for the
  IIFE argument position.
- [ ] Keep unknown non-worker globals rejected with ordinary unresolved-name
  diagnostics.
- [ ] Add focused coverage for `self.importScripts = fn(importScripts)`.

Out of scope:

- Full WebWorker runtime behavior.
- Loading external scripts from WASM/WASI.
- Complete DOM lib declaration modeling beyond this worker-global boundary.
- The later `this` / `arguments` semantic diagnostics in the nested function.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/name_resolver_tests.rs`
- reference runner or option plumbing if DOM/worker globals need to be gated

Do not touch:

- backend/runtime worker script loading unless a focused fixture proves it is
  the next blocker after name resolution.

## Acceptance criteria

- [ ] `noParameterReassignmentIIFEAnnotated.ts` no longer reports
  `UnresolvedName` for `self`.
- [ ] The same reference no longer reports `UnresolvedName` for the top-level
  `importScripts` argument after `self` resolves.
- [ ] A focused resolver test covers `self.importScripts =
  fn(importScripts);`.
- [ ] A negative resolver test proves unrelated unknown globals still fail.
- [ ] If worker globals remain runtime-unsupported, the diagnostic names the
  worker-global/runtime boundary instead of generic name resolution.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noParameterReassignmentIIFEAnnotated.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noParameterReassignmentIIFEAnnotated.ts --detail --no-dashboard-data
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
`issues/open/3564-implement-noParameterReassignmentIIFEAnnotated.md` after
fresh triage on 2026-05-08 showed the first blocker is `self`, not IIFE
parameter reassignment.

Also supersedes generated bucket
`issues/open/3565-implement-noParameterReassignmentJSIIFE.md`: fresh triage
shows the same top-level `self.importScripts = (...)` shape in
`noParameterReassignmentJSIIFE.ts`, with
`UnresolvedName: unresolved name: \`self\` at 127..131` before the compiler can
reach nested `this`, `arguments`, or parameter-reassignment semantics.

Related but distinct:

- `issues/open/5386-bind-dom-settimeout-global.md` handles the DOM timer global
  `setTimeout`. This issue is worker-global specific and includes
  `self.importScripts` plus top-level `importScripts`.

## Completion evidence

Fill when implemented.
