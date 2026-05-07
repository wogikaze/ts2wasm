---
id: 5438
title: "Support named exports of local interfaces"
type: feature
area: frontend/name-resolution
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Allow `export { A, B }` to reference local interface declarations in
declaration-oriented TypeScript reference inputs without reporting unknown
local bindings.

Split from generated bucket `3420`.

## Problem

`mutuallyRecursiveInterfaceDeclaration.ts` contains two local interfaces that
refer to each other, followed by a named export list:

```ts
interface A {
    b: B
}

interface B {
    a: A
}
export {A, B}
```

Current AST output erases the interface declarations and keeps only the named
export. Name resolution then reports that `A` is an unknown local binding.

Problem: named exports of local type-only interface declarations fail with `UnsupportedSyntax` unknown local binding.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveInterfaceDeclaration.ts
```

Observed result:

```text
UnsupportedSyntax: issue-5005: entry module `export { A }` references unknown local binding `A` at 109..110
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Compiler evidence:

```text
tokens: ok through both interface declarations and `export {A, B}`
ast: contains ExportNamed specifiers A and B, but does not retain InterfaceDeclaration bindings
resolved: fails in resolve_names with unknown local binding A
```

TypeScript oracle:

```text
ok; no diagnostics
AST top-level: InterfaceDeclaration A, InterfaceDeclaration B, ExportDeclaration
```

## Desired final state

The frontend preserves local interface declarations enough for named export
resolution, and `export { A, B }` over local type-only interfaces no longer
reports an unknown local binding.

## Scope

In scope:

- [ ] Preserve local interface declarations as type-only bindings for named export resolution.
- [ ] Allow `export { InterfaceName }` when the exported name is a local type-only interface binding.
- [ ] Add focused resolver coverage for `interface A { b: B } interface B { a: A } export {A, B}`.
- [ ] Re-triage `mutuallyRecursiveInterfaceDeclaration.ts` and record the next diagnostic or build result.

Out of scope:

- `export default InterfaceName;`, tracked by `issues/open/5403-support-type-only-default-exports-of-local-interfaces.md`.
- `export default interface Name { ... }`, tracked by `issues/open/5401-parse-export-default-interface-declarations.md`.
- Runtime exports for arbitrary type-only names.
- General module loading or re-export resolution.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser/`
- `crates/frontend/src/resolver.rs`
- focused parser/resolver/reference tests

Do not touch:

- backend/runtime lowering unless fresh implementation evidence proves it is required
- package or non-reference module resolution

## Acceptance criteria

- [ ] `mutuallyRecursiveInterfaceDeclaration.ts` no longer reports `UnsupportedSyntax` for unknown local binding `A`.
- [ ] A focused test covers named-exporting local interface declarations through `export {A, B}`.
- [ ] The implementation records the names as type-only and does not create runtime values for arbitrary interfaces.
- [ ] Re-triage records either build success or the next narrower diagnostic after the named interface export blocker.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend export
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveInterfaceDeclaration.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveInterfaceDeclaration.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Related but separate owner: issue `5403` handles `export default InterfaceName;`.
This issue is only for named export specifiers that point at local interface
declarations.

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
