# Implement GC call-frame roots for closure escape

**Status**: open
**Created**: 2026-04-28
**Updated**: 2026-04-28
**ID**: 221
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 220
**Orchestration class**: implementation-ready

Problem: Top-level locals are mirrored into GC roots, but function/call-frame locals and closure captures are still not registered. GC cannot safely collect while heap values are live only in function locals or closure environments.

Scope:

- Define root registration for function/call-frame locals that can hold heap values.
- Register and unregister roots around function execution without corrupting nested calls.
- Add closure/call-frame escape fixtures that trigger collection while preserving live heap values.
- Wire Node differential coverage for those fixtures.

Out of scope:

- Generational GC and finalizers.

Acceptance Criteria:

- [ ] Function/call-frame heap locals are marked across collection.
- [ ] Closure/call-frame escape fixtures trigger collection and preserve semantics.
- [ ] Node differential tests pass for closure/call-frame GC fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
```

Progress notes:

- 2026-04-28: initial call-frame local mirroring and `fixtures/core-semantics/gc-call-frame-root.ts`
  differential coverage were added as a partial slice. This issue remains open because the current
  root table is conservative/static and does not yet register/unregister activation frames for
  nested calls or closure captures.
- 2026-04-28: issue 222 added backend temporary root mirroring for the caller-side temporary value
  that crosses a collecting function call. Precise activation-frame push/pop and closure capture
  semantics remain open here.
