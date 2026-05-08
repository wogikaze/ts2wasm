# Design GC strategy

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-27
**Closed**: 2026-04-27
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

- [x] Heap object header design is documented.
- [x] GC strategy (mark-and-sweep or arena) is chosen and justified.
- [x] GC trigger points are defined.
- [x] GC strategy is documented in docs/14-runtime-abi.md.

Evidence:

- Chose mark-and-sweep GC over arena allocator (easier migration from bump allocator, suitable for short-lived programs)
- Documented heap object header with size, type_tag, and mark bit
- Defined GC trigger at 64KB allocation threshold
- Added GC constants to layout.rs (GC_THRESHOLD, OBJECT_TYPE_MASK, GC_MARK_BIT)
- Documented mark and sweep phases in docs/14-runtime-abi.md

Validation:

```sh
cargo fmt --all --check
grep -A 20 "GC strategy" docs/14-runtime-abi.md
```

## Completion evidence

- 2026-04-27: `docs/14-runtime-abi.md` に GC 設計（ヘッダ、戦略、トリガー条件）を追加。
- 2026-04-27: `cargo fmt --all --check` 合格（docs 修正後）。
- 2026-04-27: `grep -A 20 "GC strategy" docs/14-runtime-abi.md` で設計節が確認可能。

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/017a-design-gc-strategy.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
