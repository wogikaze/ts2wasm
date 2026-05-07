---
id: 5397
title: "Report missing namespace alias member diagnostics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TS2694-like diagnostic when an import-equals alias targets a namespace
and a qualified type reference asks for a member that the target namespace does
not export.

## Problem

Problem: `aliasBug.ts` now build-passes even though TypeScript reports TS2694
for `var p3: booz.bar;`, where `booz` aliases `foo.bar.baz` and namespace
`baz` exports class `boo` but not member `bar`.

Fresh triage on 2026-05-08 shows tokens, AST, and resolved output all succeed.
The parser erases the namespace alias statements to `Undefined`, and the
semantic checker does not preserve enough namespace export membership
information to report the missing member.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/aliasBug.ts
```

Equivalent repo task:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasBug.ts
```

Representative source:

```ts
namespace foo {
    export class Provide {}
    export namespace bar { export namespace baz { export class boo {} } }
}

import booz = foo.bar.baz;

function use() {
    var p3: booz.bar;
}
```

Concrete current failure:

```text
coverage: build_pass=1
triage: BuildPass
oracle: TS2694 Namespace 'foo.bar.baz' has no exported member 'bar'.
```

Compiler evidence:

```text
tokens: ok through `namespace foo`, nested exported namespaces/classes, `import booz = foo.bar.baz`, and `var p3: booz.bar`
ast: ok; import-equals alias statements are erased to `Expr Undefined`; `p3` is a `Let` with erased type annotation
resolved: ok; `p3` lowers to `Let("p3", Undefined)`
visible symbols: classes `Provide` and `boo`, bindings `p`, `p1`, `p2`, `p3`, `p22`
```

TypeScript oracle:

```text
TS2694 at aliasBug.ts:19:15, message: Namespace 'foo.bar.baz' has no exported member 'bar'.
```

## Desired final state

The compiler reports a source-spanned diagnostic for missing members reached
through namespace import-equals aliases instead of silently build-passing the
invalid qualified type reference.

## Scope

In scope:

- [ ] Preserve namespace import-equals alias targets enough to resolve
  `booz.bar` in type positions.
- [ ] Track exported namespace/class members for nested namespaces in this
  non-runtime diagnostic slice.
- [ ] Emit a TS2694-like diagnostic when the requested member is absent.

Out of scope:

- Runtime namespace object emission.
- General declaration emit for import-equals aliases.
- External `require(...)` import-equals module loading.
- Full TypeScript type checking for all namespace aliases.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused diagnostics fixtures or tests

Do not touch:

- backend/runtime code unless this slice changes lowered namespace metadata

## Acceptance criteria

- [ ] `aliasBug.ts` no longer build-passes the missing `booz.bar` member silently.
- [ ] The diagnostic points at the `bar` member in `var p3: booz.bar`.
- [ ] A focused fixture covers `import booz = foo.bar.baz; var p3: booz.bar;` when `baz` exports `boo` but not `bar`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(import) or test(alias)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/aliasBug.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/aliasBug.ts --detail --no-dashboard-data
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

Split from generated bucket `578` on 2026-05-08. The sibling
`aliasErrors.ts` reference has historically exposed the same TS2694 shape and
should be rechecked when triaging issue 580.

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
