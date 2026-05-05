---
id: 373
title: "Handle BigInt object ToPrimitive invalid and out-of-range string returns"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [259, 261]
blocks: []
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Define and implement the direct object-literal/local `toString` return behavior for invalid or out-of-range BigInt string coercion in mixed BigInt comparisons.

Problem: Issue 368 implemented `toString: () => <supported decimal string>` for equality and relational comparisons, but invalid strings and strings outside the current signed-i32 StringToBigInt comparison helper boundary still need explicit diagnostics or compatible behavior.

## Current failure

Representative source-backed shapes:

```ts
console.log(({ toString: () => "not-a-bigint" }) == 1n);
console.log(({ toString: () => "2147483648" }) < 1n);
console.log(1n == ({ toString: () => "9007199254740993" }));
```

These are distinct from the already implemented supported decimal-string `toString` subset.

## Desired final state

Direct object-literal/local `toString: () => <string literal>` mixed BigInt comparisons either produce Node-compatible equality/relational results for invalid strings and supported out-of-range forms, or emit source-backed diagnostics before lowering when the current BigInt/String runtime boundary cannot represent the required comparison safely.

## Scope

In scope:

- [x] Direct object literals and simple locals with no-argument arrow `toString` returning a string literal.
- [x] Invalid StringToBigInt strings for abstract equality and relational comparison.
- [x] Source-backed out-of-range string literals that exceed the current signed-i32 comparison helper boundary.
- [x] Node/iwasm differential coverage for any newly supported case and diagnostic coverage for intentionally rejected cases.

Out of scope:

- Unknown non-source-backed dynamic strings; issue 375 owns runtime-only unknown input.
- Non-string primitive returns; issue 372 owns that category.
- General object/prototype/Proxy/side-effectful coercion; issue 374 owns that category.
- Broad multi-limb BigInt comparison helpers unless required by a narrowly proven safe path.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/*bigint*`
- `docs/05-compatibility-and-semantics.md`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- parser BigInt syntax
- broad runtime ABI representation unless a compile error proves it is required
- general BigInt builtin exception parity; issue 333 owns `BigInt(...)` unknown invalid-string exceptions

## Acceptance criteria

- [x] Invalid direct `toString` string return behavior is covered by Node/iwasm differential tests or source-backed diagnostic tests with explicit issue ownership.
- [x] Out-of-range direct `toString` string return behavior cannot silently produce an incorrect normal boolean.
- [x] Existing supported decimal-string `toString` fixtures from issue 368 continue to pass.
- [x] Docs/current-state/issues state the invalid/out-of-range string-return boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-cli --test m2_node_diff bigint
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -E 'test(bigint) or test(node_diff)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/05-compatibility-and-semantics.md`
- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

This issue is a direct follow-up split from issue 368. Prefer source-backed diagnostics over broad runtime work when the value is statically visible and outside the current helper bounds.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- cde1b185 issues: close ts boundary and progress test262 bigint slices

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-cli --test m2_node_diff bigint_runtime_mixed_object_toprimitive_string_boundary_reports_issue_373: pass
mise run update-issue-index -- --check: pass
mise run check issues: pass
```

Remaining risks:

- Runtime-only unknown strings are deliberately separate in issue 375.
- Broader object/prototype/Proxy/side-effectful coercion remains deliberately separate in issue 374.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/373-handle-bigint-object-toprimitive-invalid-out-of-range-string-returns.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
