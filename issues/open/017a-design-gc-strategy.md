# Design GC strategy

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 017a
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 013
**Orchestration class**: design-ready

Problem: Current runtime has no GC. Long-running programs and programs with closure escape will leak memory. docs/04 specifies initial mark-and-sweep or arena + explicit lifetime management. A design decision is needed before implementation.

Scope:

- Design heap object header with type tag, mark bit, size, and field layout.
- Choose between mark-and-sweep GC or arena allocator.
- Define GC trigger points (allocation threshold, explicit collection).
- Document GC strategy in runtime ABI docs.

Out of scope:

- Implementation of GC (see 017b)
- Test fixtures (see 017b)

Acceptance Criteria:

- [ ] Heap object header design is documented.
- [ ] GC strategy (mark-and-sweep or arena) is chosen and justified.
- [ ] GC trigger points are defined.
- [ ] GC strategy is documented in docs/14-runtime-abi.md.

Validation:

```sh
cargo fmt --all --check
grep -A 20 "GC strategy" docs/14-runtime-abi.md
```
