---
id: 5410
title: "Report namespace-as-base-type diagnostics"
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

Report TypeScript-style diagnostics when a namespace is used as a class base
value or as an interface/implements type.

This is the semantic follow-up exposed by `moduleAsBaseType.ts`, which now
build-passes.

## Problem

`moduleAsBaseType.ts` declares an empty namespace `M`, then uses `M` in three
heritage positions:

```ts
namespace M {}
class C extends M {}
interface I extends M { }
class C2 implements M { }
```

The current compiler erases the namespace declaration, keeps `class C extends
M`, drops the interface and `implements` clause, and returns a build pass.
TypeScript reports that `M` cannot be used as a value in the class `extends`
clause and cannot be used as a type in the interface/class type heritage
clauses.

Problem: namespace-as-base-type misuse currently produces a false build pass
instead of source-spanned TS2708/TS2709-style diagnostics.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAsBaseType.ts
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAsBaseType --detail --no-dashboard-data
```

Observed compiler result:

```text
moduleAsBaseType.ts: build_pass
coverage: executed=1, build_pass=1, unsupported=0
```

Compiler evidence:

```text
tokens: ok through namespace M, class C extends M, interface I extends M, and class C2 implements M
ast: ClassDecl C extends Ident("M"); ClassDecl C2 retained without implements; namespace and interface are erased
resolved: ClassDecl C extends "M"; ClassDecl C2 retained
```

TypeScript oracle:

```text
TS2708: Cannot use namespace 'M' as a value.        // class C extends M
TS2709: Cannot use namespace 'M' as a type.         // interface I extends M
TS2709: Cannot use namespace 'M' as a type.         // class C2 implements M
```

## Desired final state

The compiler reports source-spanned diagnostics for namespace identifiers used
in class `extends`, interface `extends`, and class `implements` heritage
positions instead of silently build-passing this reference case.

## Scope

In scope:

- [ ] Preserve enough same-file namespace declaration information for heritage
      diagnostics.
- [ ] Report a TS2708-style diagnostic for `class C extends M {}` when `M` is a
      namespace rather than constructor-valued.
- [ ] Report TS2709-style diagnostics for `interface I extends M {}` and
      `class C2 implements M {}` when `M` is a namespace rather than a type.
- [ ] Add focused coverage for namespace identifiers in class and interface
      heritage clauses.

Out of scope:

- Full namespace runtime lowering.
- Namespace member lookup or qualified namespace inheritance.
- General structural type checking for class/interface heritage.
- Unrelated class/namespace static-side compatibility, tracked by issue 5331.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused frontend/resolver tests or fixtures

Do not touch:

- backend namespace emit
- static ES module resolution
- package resolution

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAsBaseType.ts` no longer reports `BuildPass`; it reports a namespace-as-base diagnostic at `M` in a heritage clause.
- [ ] Focused coverage reports `Cannot use namespace 'M' as a value` for `class C extends M {}`.
- [ ] Focused coverage reports `Cannot use namespace 'M' as a type` for `interface I extends M {}` or `class C implements M {}`.
- [ ] Existing valid class inheritance fixtures continue to build.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(class) or test(heritage) or test(implements)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAsBaseType.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAsBaseType --detail --no-dashboard-data
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

Split from generated bucket `issues/done/3306-implement-moduleAsBaseType.md`.

Related but not duplicates:

- `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`
  covers qualified value access through namespaces, not heritage diagnostics.
- `issues/open/5314-report-non-constructor-local-class-heritage.md` covers
  non-constructor local values in class `extends`, not namespace-as-type/value
  diagnostics.
- `issues/open/5331-report-class-namespace-static-side-inheritance-diagnostic.md`
  covers static-side compatibility after class/namespace merging.

## Completion evidence

Fill when implemented.
