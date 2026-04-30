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
