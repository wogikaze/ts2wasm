---
id: 5395
title: "Report getter return mismatch with setter annotation"
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

Report TS2322 when a getter returns a value incompatible with its paired
setter parameter annotation.

## Problem

Problem: `accessors_spec_section-4.5_error-cases.ts` build-passes even though
TypeScript reports TS2322 for getters returning `""` where paired setters
annotate the property type as `number`.

Fresh triage on 2026-05-08 shows tokens, AST, and resolved output all succeed.
The compiler records `set AnnotatedSetter_*` and `get AnnotatedSetter_*`
methods but does not compare the getter return expression against the setter
parameter annotation.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts
```

Equivalent repo task:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts
```

Representative source:

```ts
class C {
    public set P(a: number) { }
    public get P() { return ""; }
}
```

Concrete current failure:

```text
coverage: build_pass=1
triage: BuildPass
oracle: TS2322 Type 'string' is not assignable to type 'number' at getter return
```

## Desired final state

The getter-return mismatch no longer silently build-passes when the paired
setter annotation fixes the accessor property type.

## Scope

In scope:

- [ ] Compare a direct getter string return against the paired setter `number` annotation.
- [ ] Emit a TS2322-like diagnostic for the `AnnotatedSetter_*` getter-return mismatch.

Out of scope:

- Setter-body assignment diagnostics, tracked by issue 5396.
- Full TypeScript assignability.
- Runtime accessor property descriptors.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`

Do not touch:

- backend/runtime code unless this slice changes lowered accessor representation

## Acceptance criteria

- [ ] `accessors_spec_section-4.5_error-cases.ts` no longer build-passes the `AnnotatedSetter_*` getter-return mismatch.
- [ ] A focused fixture covers `set P(a: number) { } get P() { return ""; }`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend accessor
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/accessors_spec_section-4.5 --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Notes

Split from generated bucket `574` on 2026-05-08. The sibling getter-annotation
setter-body mismatch is tracked by issue 5396.
