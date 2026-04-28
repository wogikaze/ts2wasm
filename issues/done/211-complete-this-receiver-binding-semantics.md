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
completed: 2026-04-28
---

## Summary

Replace placeholder `this` emission with receiver-aware runtime semantics for supported method calls.

## Problem

Issue 037 added AST/IR plumbing for `this` but recorded that backend emission currently returns `undefined` pending method-call behavior. That placeholder needs a dedicated semantic issue and differential evidence.

## Desired final state

Inside supported method calls, `this` evaluates to the call-site receiver. Unsupported call forms are diagnosed or explicitly tracked.

## Scope

In scope:

- [x] Thread receiver values through method-call lowering and emission.
- [x] Emit `this` loads from the active call frame instead of placeholder `undefined`.
- [x] Add Node differential fixtures for object method calls and nested method receiver boundaries; extracted method calls are covered as issue-linked unsupported diagnostics.
- [x] Update docs/current-state/issues when semantic status changes.

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

- [x] `this` no longer emits as a fixed placeholder in supported method-call contexts.
- [x] Receiver binding matches Node.js for supported object method calls.
- [x] Unsupported receiver/call forms have issue-linked diagnostics or follow-up tracking.
- [x] Docs/current-state/issues are synchronized after behavior changes.

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

- [x] update `docs/language-reference/javascript-features.md`

Current state:

- [x] update `current-state.md`

Follow-up issues:

- [x] 223 tracks span-bearing source diagnostics for the new issue-211 unsupported forms.

## Notes

Created from issue 203 audit of `issues/done/037-implement-this-binding.md`.

## Completion evidence

Commits:

- `cf15528` issue-211: implement receiver-bound this semantics
- close commit for docs/issue sync

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: cargo nextest run -E 'test(this_receiver_method) | test(emit_wat_rejects_residual_this)'
result: pass (3 passed)
date: 2026-04-28

command: cargo nextest run -E 'test(this) | test(method)'
result: pass (33 passed; corrected OR form for assigned impacted selector)
date: 2026-04-28

command: cargo nextest run -E 'test(this|method)'
result: no tests selected (nextest treats `this|method` literally in this filter position; equivalent OR selector above was run)
date: 2026-04-28

command: cargo nextest run
result: pass (233 passed, 4 skipped)
date: 2026-04-28

command: mise run check-agent-state
result: pass
date: 2026-04-28

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-28

command: mise run check-issue-health
result: pass
date: 2026-04-28

command: mise run check-repo-smoke
result: pass
date: 2026-04-28
```

Remaining risks:

- Arrow lexical `this` remains out of scope and tracked by issue 210.
- Static `this`, top-level `this`, extracted method calls, and dynamic/function-valued local calls are explicitly unsupported with issue-211 diagnostics rather than counted as semantic pass.
- The new unsupported-form diagnostics are issue-linked but still use the current IR/lowering `span: None` diagnostic path; issue 223 tracks carrying source spans through that layer.
