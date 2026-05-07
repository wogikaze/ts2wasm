---
id: 5390
title: "Report abstract property constructor access diagnostics"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report TypeScript-style diagnostics when constructors access abstract
properties on `this`, including callable abstract properties invoked as
`this.cb(...)`, instead of lowering those calls as missing class methods.

## Problem

Problem: abstract property accesses inside the declaring class constructor
currently fall through to missing-method lowering diagnostics instead of TS2715.

`abstractPropertyInConstructor.ts` now parses and reaches lowering, but the
first blocker is:

```text
UnsupportedSyntax: method `AbstractClass.cb` not found at 269..281
```

The relevant source is in the constructor of `AbstractClass`:

```ts
let val = this.prop.toLowerCase();
if (!str) {
    this.prop = "Hello World";
}
this.cb(str);
```

TypeScript reports TS2715 diagnostics for these accesses:

```text
Abstract property 'prop' in class 'AbstractClass' cannot be accessed in the constructor.
Abstract property 'cb' in class 'AbstractClass' cannot be accessed in the constructor.
```

## Current failure

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts
```

Fresh evidence from 2026-05-07:

```text
tokens: ok
ast: ok; AbstractClass constructor body parses, abstract properties are present
resolved: fails in lower_program
diagnostic: UnsupportedSyntax method `AbstractClass.cb` not found at 269..281
TypeScript oracle: TS2715 for `this.prop` and `this.cb` constructor accesses
```

## Desired final state

The compiler recognizes abstract class properties during lowering/diagnostic
classification and emits a source-spanned diagnostic for constructor accesses
before method dispatch tries to resolve callable abstract properties as class
methods.

## Scope

In scope:

- [ ] Preserve enough abstract property metadata for class members erased from runtime class bodies.
- [ ] Detect `this.prop`, `this.prop = ...`, and `this.cb(...)` inside the declaring class constructor when `prop` / `cb` is abstract.
- [ ] Emit a source-spanned diagnostic aligned with TS2715 instead of `method AbstractClass.cb not found`.
- [ ] Re-run `abstractPropertyInConstructor.ts` and record the next blocker or build-pass result.

Out of scope:

- Full strict-property-initialization diagnostics for concrete class fields.
- Runtime dispatch for arbitrary callable class fields.
- Auto-accessor callable fields covered by issue 5322.
- Class-typed ambient local missing-method diagnostics covered by issue 5261.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/ir/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/` unless lowering produces a supported runtime shape
- unrelated class-accessor or ambient-local method-call buckets

## Acceptance criteria

- [ ] `abstractPropertyInConstructor.ts` no longer reports `method AbstractClass.cb not found`.
- [ ] A focused fixture covers `abstract cb: (s: string) => string;` followed by `this.cb(str)` inside the same class constructor.
- [ ] A focused fixture covers `this.prop` read or assignment inside the same class constructor for an abstract property.
- [ ] The resulting diagnostic is source-spanned at the abstract property access and names the declaring class/property.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class) or test(method) or test(diagnostic)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from `issues/done/558-implement-abstractPropertyInConstructor.md`.

Related but not duplicate:

- `issues/done/5261-report-class-typed-missing-instance-method-calls.md`
  handles class-typed ambient locals whose requested method is absent.
- `issues/open/5322-support-callable-class-auto-accessor-fields.md` handles
  callable auto-accessor field dispatch through `this`.

## Completion Evidence

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
