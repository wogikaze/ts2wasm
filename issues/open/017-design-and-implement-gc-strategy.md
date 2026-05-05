# Design and implement GC strategy (audit reopened #017)

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 017
**Type**: feature
**Area**: runtime/memory
**Priority**: P1
**Depends on**: 013
**Orchestration class**: implementation-ready

Problem: Current runtime has no GC. Long-running programs and programs with closure escape will leak memory. docs/04 specifies initial mark-and-sweep or arena + explicit lifetime management.

Scope:

This is a parent issue coordinating GC work. Sub-issues:
- 017a: Design GC strategy (design-ready)
- 017b: Implement GC (implementation-ready, depends on 017a)

Out of scope:

- Design and implementation are tracked in sub-issues.

Acceptance Criteria:

- [ ] 017a (design) is complete.
- [ ] 017b (implementation) is complete.
- [ ] GC prevents memory leaks in test fixtures.
- [ ] Node differential test passes for GC-relevant fixtures.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/core-semantics/closure-escape.wasm
```

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/017-design-and-implement-gc-strategy.md` before this move
- `issues/open/017-design-and-implement-gc-strategy.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
