---
id: 5011
title: "Represent or reject class runtime values in lowered IR (audit reopened #5011)"
type: feature
area: ir/backend
class: design
priority: P3
depends_on: []
blocks: []
created: 2026-05-02
updated: 2026-05-05status: open
---

## Summary

Class declarations are currently partially lowered: constructors and methods are extracted as standalone functions, static initializers are pushed to the top level, but the class statement itself is dropped from the lowered IR. There is no representation of the class runtime value (constructor object, prototype chain, `[[Prototype]]` slot).

## Current behavior

```ts
class C {
  method() { return 1; }
}
export const y = C;       // y is incorrectly undefined
const c = new C();        // new undefined — runtime error or silently wrong
```

What works:
- Class method bodies (env cells, captures, `this`)
- Constructor bodies
- Static initializer blocks
- Private field analysis (metadata only)

What is missing or dropped:
- Class runtime value (binding to constructor function)
- Prototype object / `%C.prototype%`
- `extends` (inheritance chain)
- `new C()` (constructor call)
- `C.staticMethod` / `C.staticField`
- Class value exports (`export { C }`, `export default class`)
- Class expressions

## Design options

### A. Add non-fatal diagnostics infrastructure

The pipeline currently uses `Result<T, Diagnostic>`, which makes it impossible to emit warnings while continuing compilation. A `CompileReport<T>` with diagnostic severity (`Error` / `Warning` / `Note`) would allow:

```rust
pub struct CompileReport<T> {
    pub value: T,
    pub diagnostics: Vec<Diagnostic>,
}
```

With this, class methods can still be compiled while class-value usage produces a diagnostic. See `lowered/program.rs` for the current drop site.

### B. Reject class-value usage structurally

If a class binding is referenced as a value (not just for method extraction), emit an `UnsupportedSyntax` error. This requires name-resolution-level tracking of class binding usage.

### C. Full class runtime support

Implement constructor function objects, prototype chain, `extends`, `new`, `super`, and class field initialization in the runtime. This is the most complete but most expensive option.

## Related code locations

- `crates/ir/src/lowered/program.rs` — ClassDecl is caught and dropped (line ~216)
- `crates/ir/src/lowered/resolver.rs` — dead code path for ClassDecl (line ~671)
- `crates/ir/src/lowered/validate.rs` — dead variant in lowered validator (line ~243)
- `crates/ir/src/name_resolver.rs` — pass-through with partial-support note (line ~250)

## Acceptance criteria

- [ ] Class name used as pure value (`const y = C`, `export { C }`) rejected with UnsupportedSyntax diagnostic
- [ ] `new C()` and `C.staticMethod()` continue to work
- [ ] Class method/constructor compilation continues to work
- [ ] No silent runtime correctness bugs from erased class values (rejected at name resolution)

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5011-class-runtime-value-semantics.md` before this move
- `issues/open/5011-class-runtime-value-semantics.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
