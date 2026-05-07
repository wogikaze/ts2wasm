---
id: 5396
title: "Report setter body mismatch with getter annotation"
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

Report TS2322 when a setter body assigns a value incompatible with a type
inferred from the paired getter annotation.

## Problem

Problem: `accessors_spec_section-4.5_error-cases.ts` build-passes even though
TypeScript reports TS2322 for setter bodies assigning `0` to a setter parameter
whose effective type is inferred from a paired getter annotated as `string`.

Fresh triage on 2026-05-08 shows tokens, AST, and resolved output all succeed.
The compiler records `get AnnotatedGetter_*` and `set AnnotatedGetter_*`
methods but does not use the getter annotation to diagnose the setter body.

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
    public get P(): string { return ""; }
    public set P(aStr) { aStr = 0; }
}
```

Concrete current failure:

```text
coverage: build_pass=1
triage: BuildPass
oracle: TS2322 Type 'number' is not assignable to type 'string' at setter assignment
```

## Desired final state

The setter-body mismatch no longer silently build-passes when the paired getter
annotation fixes the accessor property type.

## Scope

In scope:

- [ ] Infer a setter parameter type from the paired getter `string` annotation for direct assignment checks.
- [ ] Emit a TS2322-like diagnostic for the `AnnotatedGetter_*` setter-body mismatch.

Out of scope:

- Getter-return diagnostics against setter annotations, tracked by issue 5395.
- Full TypeScript assignability.
- Runtime accessor property descriptors.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`

Do not touch:

- backend/runtime code unless this slice changes lowered accessor representation

## Acceptance criteria

- [ ] `accessors_spec_section-4.5_error-cases.ts` no longer build-passes the `AnnotatedGetter_*` setter-body mismatch.
- [ ] A focused fixture covers `get P(): string { return ""; } set P(aStr) { aStr = 0; }`.

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

Split from generated bucket `574` on 2026-05-08. The sibling setter-annotation
getter-return mismatch is tracked by issue 5395.
