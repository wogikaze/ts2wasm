---
id: 5197
title: "Report class called without new"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Report a precise diagnostic when a class constructor value is invoked as a
plain function call, such as `C()`, instead of falling through to the broad
`issue-5011` class-value unsupported boundary.

## Problem

The parser builds `ClassDecl C` and `var c = C();`, but name resolution rejects
the class identifier as a value before it can report the TypeScript-compatible
callability error. TypeScript reports TS2348: `typeof C` is not callable and
suggests using `new`.

Problem: direct calls to class constructors without `new` currently report generic `issue-5011` class-value unsupported diagnostics.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOnClass.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-5011: class `C` cannot be used as a value - class runtime is not yet supported at 39..40
```

Source:

```ts
class C { }
var c = C();
```

Triage evidence:

- AST succeeds with `ClassDecl { name: "C" }` and `Let c = Call(Ident C, [])`.
- Visible symbols include class `C` and binding `c`.
- TypeScript oracle reports TS2348: `Value of type 'typeof C' is not callable. Did you mean to include 'new'?`.

## Desired final state

Direct `C()` calls on known class declarations report a source-spanned
not-callable class diagnostic instead of the generic class-value unsupported
diagnostic. Supported `new C()` paths remain unchanged.

## Scope

In scope:

- [ ] Detect a known class declaration used as the callee of a plain `Call`
- [ ] Emit a source-spanned class-not-callable diagnostic at the class identifier
- [ ] Preserve `new C()` and supported class method/static method paths

Out of scope:

- First-class constructor value flow, tracked by `issues/open/5192-support-first-class-class-constructor-values.md`
- Full class runtime value aliasing
- Generic callable class or construct signature type checking

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- unrelated class runtime emission

## Acceptance criteria

- [ ] `callOnClass.ts` no longer reports generic `issue-5011` for `C()`
- [ ] A focused fixture for `class C {}; C();` reports a class-not-callable diagnostic at `C`
- [ ] `new C()` class fixtures still pass
- [ ] Class constructor values passed as ordinary arguments remain governed by issue 5192

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callOnClass.ts
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

Issue 5011 deliberately rejects unsafe class-value use. This issue carves out a
narrower diagnostic for the common invalid direct-call shape where TypeScript
can say the class constructor is construct-only.

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
