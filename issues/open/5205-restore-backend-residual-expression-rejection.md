---
id: 5205
title: "Restore backend residual expression rejection"
type: bug
area: backend-wasm
class: verification-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Restore `emit_wat` invariant checks for residual `LoweredExpr::MethodCall` and `LoweredExpr::This` before WAT emission.

## Problem

The full `cargo nextest run` gate currently fails because backend tests that expect residual unsupported expressions to be rejected now receive generated WAT instead.

Problem: backend emission no longer rejects residual `MethodCall` and `This` expressions before WAT generation.

## Current failure

Reproduction:

```sh
cargo nextest run
```

Current failures:

```text
ts2wasm-backend-wasm tests::emit_wat_rejects_residual_this_before_emission
ts2wasm-backend-wasm tests::emit_wat_rejects_residual_method_call_before_emission
```

Both tests call `emit_wat(&program).expect_err(...)`, but `emit_wat` returns generated WAT instead of an `InvariantViolation`.

## Desired final state

`emit_wat` rejects residual unsupported lowered expressions before emission, preserving the backend invariant that unsupported frontend/IR shapes cannot silently become WAT.

## Scope

In scope:

- [x] Identify where residual-expression validation should run before WAT emission.
- [x] Restore `InvariantViolation` diagnostics for residual `MethodCall` and `This`.
- [x] Keep supported method-call and receiver lowering behavior intact.

Out of scope:

- Implementing new method-call semantics.
- Changing issue 5134 test262 harness behavior.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/ir/src/lowered/validate.rs`

Do not touch:

- `scripts/lib/test262_harness.py`
- `crates/compiler/src/test262_preprocessor.rs`

## Acceptance criteria

- [x] `cargo nextest run -p ts2wasm-backend-wasm emit_wat_rejects_residual` passes.
- [x] The full `cargo nextest run` no longer fails at these two backend residual tests.
- [x] No supported class/object method receiver fixture regresses.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-backend-wasm emit_wat_rejects_residual
cargo nextest run
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
git diff --check
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli class_new_expression_method_call
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] 5215 tracks the next unrelated full-gate failure found after the residual-expression tests passed.

## Notes

Discovered while closing issue 5134.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- This commit.

Validation result:

```text
command: cargo nextest run -p ts2wasm-backend-wasm emit_wat_rejects_residual
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-cli class_new_expression_method_call
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-cli --test dump_cli dump_ast_unparse_erases_ambient_declarations
result: pass
date: 2026-05-06

command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run
result: fail after the 5205 residual-expression failures were cleared; stopped at issue 5215 (`array-includes.ts` wat2wasm stack mismatch)
date: 2026-05-06
```

Remaining risks:

- Full `cargo nextest run` still fails on unrelated issue 5215.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

