---
id: 1228
title: "Implement Classnamereferencesinstaticelements"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5192]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1228.

## Summary

Closed as superseded by
`issues/done/5192-support-first-class-class-constructor-values.md`. Fresh triage
shows the static class elements parse, and the current blocker is the existing
`issue-5011` class constructor value boundary at `const oldFoo = Foo`.

## Problem

Reference test results previously showed 1 case failing in directory
`classNameReferencesInStaticElements` with diagnostics: parser-syntax. Fresh
triage shows the parser and AST construction now succeed.

Problem: the current blocker is not a static-element parser issue; it is class
runtime value usage:
`issue-5011: class Foo cannot be used as a value` at `const oldFoo = Foo`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classNameReferencesInStaticElements.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classNameReferencesInStaticElements.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing open issue 5192 covers the current boundary
- [x] Close this generated bucket as superseded rather than duplicating a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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

- [x] Duplicate candidates below are confirmed and this issue is superseded by 5192
- [x] No child issue needed because current blocker is already tracked
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and superseding issue

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classNameReferencesInStaticElements.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classNameReferencesInStaticElements.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/done/5192-support-first-class-class-constructor-values.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classNameReferencesInStaticElements.ts`

Source context:

```ts
class Foo {
    static { console.log(this, Foo) }
    static x = () => { console.log(this, Foo) }
    static y = function(this: unknown) { console.log(this, Foo) }
    static #x() { console.log(Foo) }
    x() { this.#x() }
}

const oldFoo = Foo;
(Foo as any) = null;
oldFoo.x();
oldFoo.y();
new oldFoo().x();
```

## Duplicate detection

- `issues/done/5192-support-first-class-class-constructor-values.md` is an exact
  owner for the current `issue-5011` class constructor value boundary.
- `issues/done/5011-class-runtime-value-semantics.md` documents the current
  structural rejection of class values, but 5192 is the open implementation
  issue for supporting them.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classNameReferencesInStaticElements.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classNameReferencesInStaticElements.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: issue-5011: class `Foo` cannot be used as a value at 352..355
Source: const oldFoo = Foo;
tokens: ok; static block, static fields, private static method, class value uses
AST: ok; ClassDecl Foo with static block/private method/methods
resolved: fails in resolve_names on class value `Foo`
TypeScript oracle: ok, diagnostics=[]
Superseding issue: 5192
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by `issues/done/5192-support-first-class-class-constructor-values.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classNameReferencesInStaticElements.ts
result: pass; current blocker is issue-5011 class value usage tracked by issue 5192
date: 2026-05-06
```

Remaining risks:

- After issue 5192 is implemented, this reference may expose additional static
  element runtime semantics, but the current blocker is already tracked.
