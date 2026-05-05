---
id: 021
title: "Implement full wasm backend"
type: feature
area: backend
class: blocked
priority: P2
depends_on: [008, 020]
blocks: []
created: 2026-04-26
updated: 2026-05-06
---

## Summary

Implement a full direct wasm backend path beyond the current WAT-centered emission flow.

## Problem

Problem: Full wasm backend work is an epic blocked on narrower backend slices; direct wasm binary emission is not yet implemented as a selectable work item.

The current implementation remains WAT-centric. `docs/04-compiler-architecture-and-runtime.md` specifies an initial linear-memory backend path with future Wasm GC support, but this parent issue is too broad to execute directly.

## Current failure

There is no single focused failing test for this epic. The current observable gap is that CLI builds still flow through WAT-oriented emission rather than a direct wasm binary backend slice.

Queue design note:

- This is an epic-level issue and must not be selected directly from the Ready queue.
- Execute it through child slices such as issue 021a, each with one observable backend behavior and parity validation.
- Move this issue back to an active class only when all child slices needed for the current backend milestone are closed.

## Desired final state

The backend has direct wasm binary emission, remains compatible with the runtime ABI, and has child-slice validation showing functional parity with the existing WAT path.

## Scope

In scope:

- [ ] Split this epic into direct-backend implementation children before assignment.
- [ ] Track direct wasm binary emission work outside the WAT-only path.
- [ ] Preserve compatibility with the existing runtime ABI.
- [ ] Reuse the typed WAT writer skeleton foundation from issue 008 where applicable.
- [ ] Keep future Wasm GC backend design separate unless a child slice explicitly targets it.

Out of scope:

- Direct implementation from this parent epic.
- Runtime ABI redesign without a dedicated issue.
- Frontend or semantic feature work unrelated to backend emission.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`
- `crates/cli/src/`

Do not touch:

- `crates/frontend/src/` unless a child issue proves frontend output needs to change.
- Generated coverage artifacts unless a child validation requires them.

## Acceptance criteria

- [ ] Direct wasm binary emission is implemented.
- [ ] Generated wasm is functionally equivalent to WAT path.
- [ ] Runtime ABI compatibility is maintained.
- [ ] Typed WAT writer skeleton is used as foundation.
- [ ] Node differential test passes for wasm backend fixtures.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/hello.wasm
iwasm /tmp/hello.wasm
```

Impacted commands:

```sh
cargo test -p ts2wasm-backend-wasm
cargo check -p ts2wasm-backend-wasm
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] update `docs/04-compiler-architecture-and-runtime.md` if the backend architecture changes
- [ ] update `docs/14-runtime-abi.md` if direct backend emission changes ABI assumptions

Current state:

- [ ] update `current-state.md` when this epic is split or closed

Follow-up issues:

- [ ] create focused child issues for each direct backend slice
