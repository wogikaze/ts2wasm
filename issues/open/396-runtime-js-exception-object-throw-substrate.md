---
id: 396
title: "Runtime JS exception object throwing substrate"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: [381]
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Implement the minimal runtime substrate needed to throw JavaScript `Error` objects from runtime helpers instead of representing exceptional paths as wasm `unreachable` traps.

Problem: Runtime helper exceptional paths can only trap today, so issue 381 cannot produce a compatible `TypeError` object for mixed Number/BigInt arithmetic even after the operands reach runtime safely.

## Problem

The compiler can allocate builtin Error objects and represent `throw` syntax in the lowered IR, but the wasm runtime does not yet have a compatible propagation path for runtime-generated exception objects. Existing runtime helper exceptional paths such as BigInt division by zero, invalid BigInt string conversion, JSON parse errors, and mixed Number/BigInt arithmetic either trap or remain issue-linked diagnostics.

Issue 381 now proves the mixed arithmetic operands can be evaluated and routed to a dedicated runtime trap helper without silent number lowering. Closing issue 381 requires replacing that trap with a compatible `TypeError` throw object.

## Scope

In scope:

- [ ] Define a minimal runtime exception ABI for runtime helpers to raise a builtin Error object.
- [ ] Support at least `TypeError` for mixed Number/BigInt arithmetic and keep `RangeError`/`SyntaxError` extensible for existing trap parity gaps.
- [ ] Preserve operand evaluation order before throwing.
- [ ] Add Node/iwasm evidence that a runtime-generated `TypeError` is observable through the project's supported exception surface.
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

Commits:

- none yet; issue is open

Validation result:

```text
not run; issue is open
```
