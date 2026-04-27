---
id: 211
title: "Complete this receiver binding semantics"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Replace placeholder `this` emission with receiver-aware runtime semantics for supported method calls.

## Problem

Issue 037 added AST/IR plumbing for `this` but recorded that backend emission currently returns `undefined` pending method-call behavior. That placeholder needs a dedicated semantic issue and differential evidence.

## Desired final state

Inside supported method calls, `this` evaluates to the call-site receiver. Unsupported call forms are diagnosed or explicitly tracked.

## Scope

In scope:

- [ ] Thread receiver values through method-call lowering and emission.
- [ ] Emit `this` loads from the active call frame instead of placeholder `undefined`.
- [ ] Add Node differential fixtures for object method calls, extracted method calls, and nested function boundaries in the supported subset.
- [ ] Update docs/current-state/issues when semantic status changes.

Out of scope:

- Arrow lexical `this`; tracked by issue 210.
- Constructor/class `this` semantics beyond supported method calls.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- none

## Acceptance criteria

- [ ] `this` no longer emits as a fixed placeholder in supported method-call contexts.
- [ ] Receiver binding matches Node.js for supported object method calls.
- [ ] Unsupported receiver/call forms have issue-linked diagnostics or follow-up tracking.
- [ ] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(this|method)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] update `docs/language-reference/javascript-features.md`

Current state:

- [ ] update `current-state.md`

Follow-up issues:

- [ ] none

## Notes

Created from issue 203 audit of `issues/done/037-implement-this-binding.md`.

## Completion evidence

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
