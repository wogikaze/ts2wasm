---
id: 5491
title: "Bind DOM document.querySelector global"
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

Bind the DOM `document` global far enough for `document.querySelector(...)`
references to pass name resolution, or report a precise unsupported DOM
document/querySelector diagnostic before generic name resolution fails.

## Problem

Problem: `reference/typescript/tests/cases/compiler/nonnullAssertionPropegatesContextualType.ts`
currently reports `UnresolvedName` for the DOM global receiver `document`:

```text
error: [UnresolvedName] unresolved name: `document` at 66..74
```

This blocks the reference before it can validate the intended non-null
assertion and contextual-type behavior.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonnullAssertionPropegatesContextualType.ts
```

Representative source:

```ts
// @target: es2015
// @strict: true
let rect2: SVGRectElement = document.querySelector('.svg-rectangle')!; // Error: Element
```

Compiler evidence:

- Tokens are ok through `document.querySelector('.svg-rectangle')!`.
- AST is ok and represents the initializer as a call whose callee is
  `Member(Ident document, querySelector)`.
- The non-null assertion is erased before the runtime AST; the current first
  blocker is not parser syntax.
- Name resolution stops on `document` with only local binding `rect2` visible.
- TypeScript oracle accepts the reference with no diagnostics.

Focused coverage result:

```text
suite=tsc
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

## Desired final state

The compiler no longer reports generic `UnresolvedName` for `document` in this
DOM reference shape. The representative reference advances to build-pass or to
the next precise unsupported boundary.

## Scope

In scope:

- [ ] Treat `document` as a known DOM global receiver for name resolution in
  reference inputs that request browser globals.
- [ ] Keep the diagnostic source-spanned at `document` or `querySelector` if
  the DOM API remains unsupported after binding.
- [ ] Add focused coverage for `document.querySelector('.svg-rectangle')!`.
- [ ] Preserve ordinary unresolved-name diagnostics for unrelated unknown
  globals.

Out of scope:

- Full DOM lib declaration modeling.
- Browser DOM runtime semantics in WASM/WASI.
- The DOM `setTimeout` global, tracked by issue 5386.
- DOM/WebWorker `self` and `importScripts`, tracked by issue 5479.
- Unrelated DOM APIs beyond `document.querySelector`.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/name_resolver_tests.rs`
- reference runner or builtin/global binding code

Do not touch:

- backend/runtime DOM implementation unless focused evidence proves it is the
  next blocker after name resolution.
- unrelated DOM globals already owned by existing issues.

## Acceptance criteria

- [ ] `nonnullAssertionPropegatesContextualType.ts` no longer reports
  `UnresolvedName` for `document`.
- [ ] A focused resolver/reference test covers
  `document.querySelector('.svg-rectangle')!`.
- [ ] A negative resolver test proves unrelated unknown globals still fail.
- [ ] If `querySelector` remains unsupported, the diagnostic names the DOM
  document/querySelector boundary instead of generic name resolution.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonnullAssertionPropegatesContextualType.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonnullAssertionPropegatesContextualType.ts --detail --no-dashboard-data
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
`issues/open/3606-implement-nonnullAssertionPropegatesContextualType.md` after
fresh triage on 2026-05-08 showed the first blocker is DOM `document` name
resolution, not type assertion or non-null assertion parsing.

Related but distinct:

- `issues/open/5386-bind-dom-settimeout-global.md` handles the DOM timer global
  `setTimeout`.
- `issues/open/5479-bind-dom-worker-self-importscripts-globals.md` handles
  worker-global `self.importScripts` and top-level `importScripts`.

## Completion evidence

Fill when implemented.
