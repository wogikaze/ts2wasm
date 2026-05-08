---
id: 5327
title: "Report class method overload wrong implementation name"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report TypeScript-compatible diagnostics when a class method overload
declaration is followed by a concrete method with a different name, and when a
later overload signature is left without an immediately following
implementation.

## Problem

`classWithOverloadImplementationOfWrongName2.ts` parses successfully, but the
compiler treats the two bodyless `foo` class method declarations as duplicate
methods and reports a generic `DuplicateFunction` before it can surface the
actual TypeScript overload grouping errors.

Problem: `classWithOverloadImplementationOfWrongName2.ts` reports `DuplicateFunction: duplicate method definition: C.foo` instead of TS2389/TS2391-equivalent diagnostics for the wrong implementation name and missing implementation.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts
```

Current diagnostic:

```text
error: [DuplicateFunction] duplicate method definition: `C.foo`
```

Source context:

```ts
class C {
    foo(): string;
    bar(x): any { }
    foo(x): number;
}
```

Triage evidence:

```text
tokens: ok
ast: ok; ClassDecl C contains bodyless method `foo`, concrete method `bar`, and bodyless method `foo`
resolved/lowering: DuplicateFunction duplicate method definition: `C.foo`
TypeScript oracle:
  TS2389 Function implementation name must be 'foo' at `bar`
  TS2391 Function implementation is missing or not immediately following the declaration at second `foo`
```

## Desired final state

The class method overload validator distinguishes bodyless overload signatures
from concrete method implementations and reports source-spanned diagnostics for
wrong-name or missing class method overload implementations instead of the
generic duplicate-method diagnostic.

## Scope

In scope:

- [ ] Detect a bodyless class method overload signature followed by a concrete
  class method with a different name.
- [ ] Report a source-spanned TS2389-equivalent diagnostic at the wrong concrete
  implementation name.
- [ ] Report a source-spanned TS2391-equivalent diagnostic when a class method
  overload signature has no immediately following same-name implementation.
- [ ] Preserve valid class method overload merging tracked by issue 5198.
- [ ] Preserve duplicate concrete method diagnostics for real duplicate method
  implementations.

Out of scope:

- Top-level function overload validation, tracked by issue 5200.
- Valid class method overload call behavior through element access, tracked by
  issue 5198.
- Constructor overload diagnostics.
- Full overload resolution or type checking.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/` unless triage proves the diagnostic has advanced past
  frontend/IR validation.

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts`
  no longer reports `DuplicateFunction: duplicate method definition: C.foo`.
- [ ] The representative path reports a TS2389-equivalent diagnostic at `bar`
  or a TS2391-equivalent diagnostic at the later bodyless `foo`.
- [ ] A focused fixture or test covers `class C { foo(): string; bar(x): any {} foo(x): number; }`.
- [ ] The valid overload group from `issues/done/5198-support-class-method-overload-signatures-for-element-access-calls.md`
  remains under issue 5198 and is not converted into an error by this slice.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class) or test(overload) or test(duplicate)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts --detail --no-dashboard-data
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

Split from generated bucket
`issues/done/1248-implement-classWithOverloadImplementationOfWrongName.md`.

Related but distinct issues:

- `issues/done/5198-support-class-method-overload-signatures-for-element-access-calls.md`
  handles valid class method overload signature merging.
- `issues/open/5200-validate-top-level-function-overload-implementations.md`
  handles top-level function overload implementation grouping.

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
