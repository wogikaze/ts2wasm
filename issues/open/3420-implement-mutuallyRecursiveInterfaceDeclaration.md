---
id: 3420
title: "Implement Mutuallyrecursiveinterfacedeclaration"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Closed after splitting the representative failure into implementation-ready
child issue `5438-support-named-exports-of-local-interfaces.md`.

Fresh triage showed the current failure is not mutually recursive interface
typing itself. The frontend tokenizes the interfaces and named export, but the
AST erases the local interface declarations before name resolution, so
`export {A, B}` reports an unknown local binding for `A`.

## Problem

Reference test results show 1 cases fail in directory `mutuallyRecursiveInterfaceDeclaration` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: mutuallyRecursiveInterfaceDeclaration has 1 reference failure and
needed smart-triage evidence before implementation starts.

Disposition: implementation work is tracked by child issue `5438`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveInterfaceDeclaration.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveInterfaceDeclaration.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveInterfaceDeclaration.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveInterfaceDeclaration.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5438-support-named-exports-of-local-interfaces.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/mutuallyRecursiveInterfaceDeclaration.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveInterfaceDeclaration.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
reference/typescript/tests/cases/compiler/mutuallyRecursiveInterfaceDeclaration.ts: UnsupportedSyntax: unknown-unsupported
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveInterfaceDeclaration.ts

result:
UnsupportedSyntax: issue-5005: entry module `export { A }` references unknown local binding `A` at 109..110
```

Source context:

```ts
interface A {
    b: B
}

interface B {
    a: A
}
export {A, B}
```

Compiler evidence:

```text
tokens: ok through both interface declarations and `export {A, B}`
ast: ok but only retains ExportNamed specifiers A and B; local InterfaceDeclaration bindings are erased
resolved: fails in resolve_names with unknown local binding A
visible symbols before failure: []
```

TypeScript oracle evidence:

```text
ok; no diagnostics
AST top-level: InterfaceDeclaration A, InterfaceDeclaration B, ExportDeclaration
```

## Completion evidence

Split into:

- `5438-support-named-exports-of-local-interfaces.md`

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/mutuallyRecursiveInterfaceDeclaration.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/mutuallyRecursiveInterfaceDeclaration.ts
result: pass; UnsupportedSyntax unknown local binding A in named export
date: 2026-05-08
```

Remaining risks:

- Implementation remains open in `5438`.
