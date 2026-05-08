---
id: 5339
title: "Preserve var after object type declaration"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Fix one parser case: a TypeScript-erased `var` declaration with an object type
annotation followed by another `var` declaration.

```ts
var console: {
    log(message: any);
}
var _super = 10;
```

## Problem

`collisionSuperAndNameResolution.ts` currently tokenizes successfully, but the
AST drops the following `_super` declaration. The current AST contains a single
top-level `Let console = Number(10)` instead of preserving the separate
`var _super = 10` binding, so name resolution later fails in
`console.log(_super)`.

Problem: a `var name: { ... }` object type annotation consumes the next
initialized `var` declaration, causing `UnresolvedName` for the following
binding.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionSuperAndNameResolution.ts
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionSuperAndNameResolution.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
Diagnostic: UnresolvedName / resolver-symbol
error: [UnresolvedName] unresolved name: `_super`
coverage: executed=1 build_pass=0 unsupported=1 blocked=0
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Source context:

```ts
var console: {
    log(message: any);
}
var _super = 10; // No error
class base {
}
class Foo extends base {
    x() {
        console.log(_super);
    }
}
```

Compiler evidence:

```text
tokens: ok; includes typed `var console`, following `var _super`, `class base`, and `class Foo extends base`
ast: wrong shape; top-level statements contain `Let console = Number(10)` and no separate `_super` binding
resolved: fails in resolve_names with UnresolvedName for `_super`
TypeScript oracle: sees `_super` as a number binding; current TS diagnostic is only duplicate `console`
```

## Desired final state

The parser skips the object type annotation for `var console: { ... }` without
consuming the following `var _super = 10` declaration.

## Scope

In scope:

- [x] Preserve the initialized declaration following `var typed: { ... }`.
- [x] Add one focused parser regression for `var typed: { m(x: any); }\nvar next = 10;`.
- [x] Re-run `collisionSuperAndNameResolution.ts` and record the next diagnostic.

Out of scope:

- Full TypeScript structural type support.
- The existing TypeScript oracle diagnostic for duplicate global `console`.
- General subclass `super` runtime semantics.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- backend emit or runtime ABI
- unrelated resolver builtins
- TypeScript oracle tooling

## Acceptance criteria

- [x] The parser no longer folds `var _super = 10` into the preceding `var console: { ... }` declaration.
- [x] A focused parser regression proves `var next = 10` is preserved after an object type annotation declaration.
- [x] `reference-triage` for `collisionSuperAndNameResolution.ts` no longer reports `UnresolvedName: unresolved name: \`_super\`` or records the next diagnostic in this issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend parser
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionSuperAndNameResolution.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionSuperAndNameResolution.ts --detail --no-dashboard-data
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

Split from `issues/open/1315-implement-collisionSuperAndNameResolution.md`.

The broad `issues/open/064-implement-name-resolution.md` is not a duplicate; it
documents older test262 metadata-related name-resolution buckets. This issue is
specific to TypeScript object type annotation parsing dropping a following
runtime binding.

## Completion Evidence

Fill when implemented.
