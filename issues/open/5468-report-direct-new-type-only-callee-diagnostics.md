---
id: 5468
title: "Report direct new type-only callee diagnostics"
type: bug
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report TypeScript-style diagnostics for direct `new` expressions whose callee is
a type-only primitive name such as `any` or `boolean`.

Split from generated bucket
`issues/done/3504-implement-newNonReferenceType.md`.

## Problem

Problem: `newNonReferenceType.ts` now builds successfully, but TypeScript
reports TS2693 for both direct constructor calls:

```ts
var a = new any();
var b = new boolean(); // error
```

The compiler currently resolves those callees as constructor class names:

```text
Let("a", New { class_name: "any", args: [] })
Let("b", New { class_name: "boolean", args: [] })
```

This hides a TypeScript semantic diagnostic by treating type-only primitive
names as runtime values.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newNonReferenceType.ts
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newNonReferenceType.ts --detail --no-dashboard-data
```

Current compiler result:

```text
build_pass=1
unsupported=0
tokens: ok
ast: ok; New Ident("any") and New Ident("boolean")
resolved: ok; class_name "any" and class_name "boolean"
```

TypeScript oracle:

```text
TS2693: 'any' only refers to a type, but is being used as a value here.
TS2693: 'boolean' only refers to a type, but is being used as a value here.
```

## Desired final state

The resolver or diagnostic layer recognizes direct `new` callees rooted in
type-only primitive names and emits source-spanned TypeScript-style diagnostics
instead of treating them as runtime class names.

## Scope

In scope:

- [ ] Detect direct `New` expressions whose callee identifier is a type-only
  primitive name, starting with `any` and `boolean`.
- [ ] Emit a source-spanned diagnostic at the type-only callee identifier.
- [ ] Preserve supported `new ClassName(...)` and `new namespace.Class(...)`
  paths.
- [ ] Add focused fixture or CLI diagnostic coverage for `new any()` and
  `new boolean()`.

Out of scope:

- Indexed `new any[1]`, tracked by
  `issues/open/5203-report-indexed-new-type-only-callee-diagnostics.md`.
- Full TypeScript checker parity for every primitive or alias type.
- Runtime support for dynamic constructor expressions.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- unrelated runtime class emission

## Acceptance criteria

- [ ] `newNonReferenceType.ts` no longer reports `BuildPass` when TypeScript
  reports TS2693 for `new any()` and `new boolean()`.
- [ ] A focused regression covers both direct type-only `new` callees.
- [ ] Diagnostics are source-spanned at `any` and `boolean`.
- [ ] Existing valid `new ClassName(...)` and `new namespace.Class(...)`
  behavior remains green.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class) | test(new)'
env TS2WASM_BINARY=target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newNonReferenceType.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newNonReferenceType.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

This is the direct-callee counterpart to issue 5203. Keep it narrow: the goal is
to prevent primitive type names from becoming synthetic runtime constructors in
`new` expressions.

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
