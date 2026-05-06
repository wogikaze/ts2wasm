---
id: 5255
title: "Resolve super property accesses"
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

Resolve or explicitly classify `super` property accesses so parsed
`super.x` and `super["x"]` expressions no longer fail as bare unresolved
identifier lookups.

## Problem

Problem: `classExtendingAny.ts` parses class declarations, `extends Err`, and
`super` property expressions, but name resolution treats the `super` receiver
as a normal identifier and fails with this diagnostic:

```text
UnresolvedName: unresolved name: `super`
```

Current representative:

```ts
declare var Err: any
class A extends Err {
    constructor() {
        super(1,2,3,3,4,56)
        super.unknown
        super['unknown']
    }
}
```

Fresh triage points at the element-access form:

```text
UnresolvedName: unresolved name: `super` at 252..257
```

The same file also contains an object-literal method with `super.unknown`,
which should be preserved as a related negative or deferred semantic boundary
instead of being hidden behind a generic unresolved-name failure.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingAny.ts
```

Current diagnostic:

```text
UnresolvedName: unresolved name: `super` at 252..257
```

## Scope

In scope:

- [ ] Treat `super` as a special receiver in member and element access
  expressions during name resolution.
- [ ] Advance `classExtendingAny.ts` past the bare unresolved-name diagnostic
  at `super['unknown']`.
- [ ] Preserve valid `super()` constructor-call handling in derived classes.
- [ ] Emit a source-spanned unsupported/semantic diagnostic if full runtime
  support for `super.x` or `super["x"]` remains deferred.
- [ ] Keep invalid `super` use outside class/object method contexts diagnostic.

Out of scope:

- Full runtime semantics for every `super` property read/write.
- `super[...]()` call lowering; broad call-expression cases remain under
  `issues/open/420-implement-call-expression.md`.
- Lexical `super` capture in arrow arguments to `super(...)`, tracked by
  `issues/open/5204-resolve-lexical-super-property-captures-in-super-call-arguments.md`.
- Non-derived `super()` diagnostics, tracked by
  `issues/open/5233-report-super-call-in-non-derived-class.md`.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- unrelated class runtime or backend WASM behavior unless resolver/lowering
  exposes a supported representation
- broad call-expression lowering

## Acceptance criteria

- [ ] `classExtendingAny.ts` no longer reports
  `UnresolvedName` for the `super` receiver.
- [ ] A focused fixture covers `class A extends B { constructor() { super.x; super["x"]; } }`.
- [ ] Invalid `super` use outside class/object method contexts remains
  diagnosed with a source span.
- [ ] If runtime support is deferred, the new diagnostic names the super
  property-access semantic boundary instead of resolver-symbol failure.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class) | test(super)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingAny.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendingAny.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/done/1194-implement-classExtendingAny.md`.

Related but not owners for this exact boundary:

- `issues/open/420-implement-call-expression.md` covers `super[...]()` call
  expression semantics in broader test262 windows.
- `issues/open/5204-resolve-lexical-super-property-captures-in-super-call-arguments.md`
  excludes dynamic `super[expr]` property access.
- `issues/open/5233-report-super-call-in-non-derived-class.md` excludes
  `super.method(...)` and `super.x` beyond its non-derived `super()` diagnostic.
