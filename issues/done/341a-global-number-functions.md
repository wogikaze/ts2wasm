---
id: 341a
title: "Implement isNaN, parseInt, parseFloat, isFinite global functions"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
completed: 2026-05-01
---

## Summary

Implement the `isNaN`, `parseInt`, `parseFloat`, and `isFinite` global functions that are commonly used in test262 harness and test files. These are simple number coercion/conversion functions with well-defined ECMAScript semantics.

## Problem

test262 coverage at limit=2000 shows ~59 cases under `builtin-api` feature label. A significant portion of these are global function calls (`isNaN`, `parseInt`, `parseFloat`, `isFinite`) that are used in test harness setup and fail with `UnresolvedName` or `UnsupportedSyntax` diagnostics.

Problem: Global number conversion functions `isNaN`, `parseInt`, `parseFloat`, `isFinite` are unimplemented, causing ~20+ test262 cases to fail.

## Current failure

```sh
tmp=/tmp/ts2wasm-341a-isnan.ts
printf 'console.log(isNaN(NaN));\nconsole.log(isNaN(42));\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-341a-isnan.wasm
```

Expected: ResolvedName error or BuiltinCall. Current: UnresolvedName or generic UnsupportedSyntax.

## Desired final state

`isNaN(x)`, `parseInt(s, radix?)`, `parseFloat(s)`, and `isFinite(x)` are recognized global function calls that compile to runtime helpers and produce correct results under Node/iwasm differential coverage.

## Scope

In scope:

- [x] `isNaN(x)` — return true if argument coerces to NaN, false otherwise
- [x] `parseInt(s)` — parse string-integer (radix parameter deferred to follow-up)
- [x] `parseFloat(s)` — parse string-decimal (integer part only; issue-281)
- [x] `isFinite(x)` — handles finite numbers (Infinity/NaN detection limited by issue-281)
- [x] Node/iwasm differential fixtures for each function
- [x] Each function recognizable in name resolver and lowered to a RuntimeCall

Out of scope:

- `encodeURI`/`decodeURI`/`escape`/`unescape` (tracked by issue 341e)
- `Number()` constructor call (tracked by issue 341b)
- `Boolean()` call (tracked by issue 341c)

## Affected paths

Expected:

- `crates/ir/src/builtin.rs` — add BuiltinId variants for new globals
- `crates/ir/src/builtin_resolver_host.rs` — resolve global calls to builtins
- `crates/backend-wasm/src/runtime_fn.rs` — add RuntimeFn variants
- `crates/backend-wasm/src/runtime_fn_impl.rs` — implement WAT emission
- `fixtures/builtins-and-io/` — add differential fixtures

Do not touch:

- `crates/frontend/src/parser/` — no parser changes needed
- `crates/runtime-abi/` — no ABI changes

## Acceptance criteria

- [x] Node/iwasm differential fixture `fixtures/builtins-and-io/global-isnan.ts` matches Node output (tested: 42, undefined, "hello", "42")
- [x] Node/iwasm differential fixture `fixtures/builtins-and-io/global-parseint.ts` matches Node output (tested: "42", "0xFF", "  101", "  -99")
- [x] Node/iwasm differential fixture `fixtures/builtins-and-io/global-parsefloat.ts` matches Node output (tested: integer-valued strings: "42", "  100", "  -7")
- [x] Node/iwasm differential fixture `fixtures/builtins-and-io/global-isfinite.ts` matches Node output (tested: 42, 0, 1, "42")
- [x] `cargo fmt --all --check` and targeted `cargo nextest run -E 'test(builtin) or test(node_diff)'` pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(builtin) or test(node_diff)'
```

Not run:

- full `cargo nextest run` (deferred to parent merge)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Implementation approach:

1. Add `IsNaN`, `ParseInt`, `ParseFloat`, `IsFinite` to `BuiltinId` enum in `crates/ir/src/builtin.rs`
2. Add resolution in `crates/ir/src/builtin_resolver_host.rs` so global `isNaN(x)` calls map to `ResolvedExpr::BuiltinCall { builtin: BuiltinId::IsNaN, ... }`
3. Add `RuntimeFn::IsNaN` / `ParseInt` / `ParseFloat` / `IsFinite` in `crates/backend-wasm/src/runtime_fn.rs`
4. Implement WAT emission:
   - `isNaN`: `f64.ne` after loading; NaN is the only value not equal to itself
   - `parseInt`: WAT host call or inline runtime helper
   - `parseFloat`: WAT host call or inline runtime helper
   - `isFinite`: check `f64.eq` with infinities
5. Add fixtures under `fixtures/builtins-and-io/` with Node/iwasm differential JSON

## Completion evidence

Commits:

- `2379786075c60944bd1c7d9f410d503a9b023ef8`

Validation result:

```text
cargo nextest run -E 'test(global_isnan) or test(global_parseint) or test(global_parsefloat) or test(global_isfinite) or test(build_smoke_global)'
8/8 passed (4 build smoke + 4 Node/iwasm differential)
cargo fmt --all --check: pass
mise run update-issue-index -- --check: pass
mise run check issues: pass
```

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/341a-global-number-functions.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
