---
id: 5386
title: "Bind DOM setTimeout global"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #5386.

## Summary

Bind the DOM `setTimeout` global for TypeScript references that request a DOM
lib, or report a more precise unsupported DOM-timer diagnostic before generic
name resolution fails.

## Problem

Problem: `contextuallyTypeArgumentsKeyword.ts` requests `// @lib: es2017, dom`
but name resolution stops on `setTimeout` before the reference can reach the
intended `arguments` contextual-typing behavior.

Current diagnostic:

```text
error: [UnresolvedName] unresolved name: `setTimeout` at 189..199
```

TypeScript accepts the reference with no diagnostics.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypeArgumentsKeyword.ts
```

Representative source:

```ts
// @lib: es2017, dom
// @Filename: foo.js
const x = {
    bar() {
        setTimeout(function() { arguments }, 0);
    }
}
```

Compiler evidence:

```text
tokens: ok; includes `setTimeout(function() { arguments }, 0)`
ast: ok; object method `bar` contains Call(Ident setTimeout, FunctionExpr, Number 0)
resolved: UnresolvedName for `setTimeout`
TypeScript oracle: ok, diagnostics []
```

## Desired final state

The compiler no longer reports generic `UnresolvedName` for `setTimeout` in
DOM-lib TypeScript references. The representative reference advances to
build-pass or to the next more specific unsupported boundary.

## Scope

In scope:

- [ ] Bind `setTimeout` as a known DOM/global callable when the source requests
  a DOM lib or when the reference runner provides DOM globals.
- [ ] Keep the diagnostic source-spanned at the `setTimeout` identifier if the
  runtime remains unsupported.
- [ ] Add focused coverage for `setTimeout(function() {}, 0)` in a DOM-lib
  TypeScript input.

Out of scope:

- Full browser event-loop or timer runtime semantics.
- Implementing callback scheduling in WASM/WASI.
- Arguments-object semantics inside the callback body.
- Broad DOM lib declaration modeling beyond the timer global needed here.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- reference runner or builtin/global binding code

Do not touch:

- `crates/backend-wasm/` unless a focused fixture proves a stub lowering is the
  narrowest way to remove this name-resolution blocker.
- unrelated DOM APIs.

## Acceptance criteria

- [ ] `contextuallyTypeArgumentsKeyword.ts` no longer reports
  `UnresolvedName` for `setTimeout`.
- [ ] A focused fixture covers a DOM-lib `setTimeout(function() {}, 0)` call.
- [ ] Non-DOM unknown globals still report ordinary unresolved-name
  diagnostics.
- [ ] If timers remain unsupported, the diagnostic names the DOM timer boundary
  instead of generic name resolution.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(global) or test(builtin) or test(name)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypeArgumentsKeyword.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypeArgumentsKeyword.ts --detail --no-dashboard-data
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
`issues/done/1535-implement-contextuallyTypeArgumentsKeyword.md`.

Related but distinct:

- `issues/done/412-implement-arguments-object.md` explains that many
  arguments-object buckets were classifier artifacts. This reference currently
  fails before `arguments` semantics because the DOM timer global is absent.

## Completion evidence

Fill when implemented.
