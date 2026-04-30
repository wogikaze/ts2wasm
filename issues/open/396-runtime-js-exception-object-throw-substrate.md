---
id: 396
title: "Runtime JS exception object throwing substrate"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: [380, 381]
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Implement the minimal runtime substrate needed to throw JavaScript `Error` objects from runtime helpers instead of representing exceptional paths as wasm `unreachable` traps.

Problem: Runtime helper exceptional paths can only trap today, so issue 380 cannot produce a compatible catchable `RangeError` object for BigInt division/remainder by zero and issue 381 cannot produce a compatible `TypeError` object for mixed Number/BigInt arithmetic even after the operands reach runtime safely.

## Problem

The compiler can allocate builtin Error objects and represent `throw` syntax in the lowered IR, but the wasm runtime does not yet have a compatible propagation path for runtime-generated exception objects. Existing runtime helper exceptional paths such as BigInt division by zero, invalid BigInt string conversion, JSON parse errors, and mixed Number/BigInt arithmetic either trap or remain issue-linked diagnostics.

Issue 380 now proves BigInt `/ 0n` and `% 0n` can reach a runtime helper that reports `RangeError: Division by zero` before aborting. Issue 381 now proves the mixed arithmetic operands can be evaluated and routed to a dedicated runtime trap helper without silent number lowering. Closing either issue requires replacing those aborting diagnostics with compatible catchable Error objects.

## Scope

In scope:

- [ ] Define a minimal runtime exception ABI for runtime helpers to raise a builtin Error object.
- [ ] Support `RangeError` for BigInt division/remainder by zero and at least `TypeError` for mixed Number/BigInt arithmetic, while keeping `SyntaxError` extensible for existing trap parity gaps.
- [ ] Preserve operand evaluation order before throwing.
- [ ] Add Node/iwasm evidence that a runtime-generated `TypeError` is observable through the project's supported exception surface.
- [ ] Add Node/iwasm evidence that a runtime-generated `RangeError: Division by zero` is observable through the project's supported exception surface.
- [ ] Document the boundary in `docs/14-runtime-abi.md` and `current-state.md`.

Out of scope:

- Full ECMAScript try/catch/finally completion-record semantics beyond what the minimal runtime throw substrate requires.
- Multi-limb BigInt arithmetic.
- Changing parser BigInt syntax or literal folding policy.

## Affected paths

Expected:

- `crates/ir/src/lowered/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `docs/14-runtime-abi.md`
- `current-state.md`

Do not touch:

- Parser BigInt syntax.
- Multi-limb arithmetic algorithms.
- Unrelated host API behavior.

## Acceptance criteria

- [ ] Runtime helpers can raise at least one builtin Error object without relying on an `unreachable` trap as the only observable behavior.
- [ ] BigInt division/remainder by zero can use that substrate to produce compatible catchable `RangeError` parity for the issue 380 fixture slice.
- [ ] Mixed Number/BigInt arithmetic can use that substrate to produce compatible `TypeError` parity for the issue 381 fixture slice.
- [ ] Existing runtime trap parity tests are either migrated to the new substrate or explicitly left with issue references.
- [ ] Documentation names the remaining exception boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint_mixed
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -E 'test(bigint) or test(node_diff) or test(exception)'
```

## Completion evidence

Fill only when moving to `done/`.

## Progress evidence

2026-05-01 child-396 progress:

- Added the first runtime exception diagnostic substrate slice for mixed Number/BigInt arithmetic.
- `BigIntMixedArithmeticTypeError` now declares a `$write` dependency and runtime TypeError string through the `RuntimeFn` catalog.
- The runtime helper now writes `TypeError: Cannot mix BigInt and other types, use explicit conversions` before aborting, so the observable iwasm failure is not just a bare `unreachable` trap.
- Updated Node/iwasm mixed arithmetic tests to assert the TypeError diagnostic surface after successful build.
- Full catchable JavaScript Error-object propagation remains open, so this issue is PROGRESS rather than DONE.

Commits:

- none yet; issue is open

Validation result:

```text
not run; issue is open
```

2026-05-01 child-396 RangeError substrate progress:

- Extracted the BigInt division/remainder-by-zero diagnostic into a named runtime helper, `bigint_division_by_zero_range_error`.
- `BigIntDiv` now depends on that helper through the `RuntimeFn` catalog, so the RangeError message and `$write` dependency are declared by the runtime exception substrate instead of being inline inside the arithmetic helper.
- `BigIntRem` continues to share the same path through `BigIntDiv`, preserving the existing operand evaluation and division/remainder helper behavior.
- Updated the runtime ABI documentation to name both current diagnostic/abort helpers and leave catchable Error-object propagation open.

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-cli --test m2_node_diff rangeerror: pass (2 passed; 201 filtered out)
mise run update-issue-index -- --check && mise run check issues: pass
```

2026-05-01 child-396 diagnostic ABI factoring progress:

- Factored the backend WAT emission for runtime diagnostic/abort helpers into a shared `emit_runtime_diagnostic_abort` boundary.
- `bigint_mixed_arithmetic_type_error`, `bigint_division_by_zero_range_error`, `bigint_string_comparison_boundary_error`, and `private_brand_type_error` now share the same backend emission path while retaining their `RuntimeFn` catalog entries, runtime strings, and `$write` dependencies.
- Documented the cataloged TypeError/RangeError/private-brand diagnostic helpers in `docs/14-runtime-abi.md` and clarified that catchable JavaScript Error-object propagation remains the issue-396 blocker.

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-cli --test m2_node_diff rangeerror: pass (2 passed; 203 filtered out)
cargo test -p ts2wasm-cli --test m2_node_diff bigint_mixed: failed in existing issue-281 diagnostic-kind expectation; the touched TypeError cases passed (9 passed, 1 failed: bigint_mixed_number_model_gap_reports_issue_281 expected UnsupportedSyntax but got UnsupportedBuiltin for Number.NaN)
cargo test -p ts2wasm-backend-wasm private_field_runtime_calls: pass (3 passed; 28 filtered out)
mise run update-issue-index -- --check && mise run check issues: pass
```

2026-05-01 child-396 catchable runtime Error-like object progress:

- Added a minimal pending-exception runtime state (`$exception_pending` plus `$exception_handler_depth`) used only by selected runtime helper errors and the current `TryCatch` emitter.
- `bigint_mixed_arithmetic_type_error` now preserves the existing uncaught diagnostic/abort behavior when no handler is active, but inside supported `try/catch` it allocates a TypeError-like heap object with the builtin TypeError prototype and `message` property, stores it as pending, and returns `undefined` for statement-boundary catch propagation.
- `bigint_division_by_zero_range_error` now uses the same active-handler path for a RangeError-like heap object with `message = "Division by zero"`; uncaught division/remainder by zero remains the existing diagnostic/abort surface.
- Added Node/iwasm differential fixtures for `catch (e) { console.log(e.message); }` over mixed Number/BigInt arithmetic and BigInt division by zero.
- Full ECMAScript completion-record propagation, nested expression unwinding, and broader helper adoption remain open, so this issue is PROGRESS rather than DONE.

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-cli --test m2_node_diff bigint_mixed: failed on pre-existing issue-281 diagnostic-kind expectation; new bigint_mixed_runtime_typeerror_catch fixture passed in the same run (10 passed, 1 failed)
cargo test -p ts2wasm-cli --test m2_node_diff bigint_mixed_runtime_typeerror_catch: pass (1 passed; 208 filtered out)
cargo test -p ts2wasm-cli --test m2_node_diff rangeerror: pass (3 passed; 206 filtered out)
mise run update-issue-index -- --check: pass
mise run check issues: pass
```
