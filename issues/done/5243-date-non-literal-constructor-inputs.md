---
id: 5243
title: "Implement non-literal Date constructor inputs"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Support Date() constructor with non-integer epoch-millisecond inputs, string parsing, and multi-argument forms.

## Problem

The current Date constructor only supports a single integer epoch-millisecond argument (`new Date(epochMs)`). Non-integer numeric inputs work through the existing `is_date_constructor_epoch_arg` path but string inputs like `new Date("2024-01-01")` and multi-argument forms like `new Date(2024, 0, 1)` hit:

```text
issue-050: only deterministic new Date(<epoch-ms integer>) is supported in this slice
```

TypeScript/node accepts ISO 8601 string inputs and multi-argument constructor forms and returns deterministic Date values.

## Scope

In scope:

- [ ] Non-integer numeric epoch-millisecond inputs to new Date() — **already works**
- [ ] String-based Date parsing for ISO 8601 format: `new Date("2024-01-01")`
- [ ] Multi-argument constructor forms: `new Date(year, month, day, ...)`

Out of scope:

- Timezone-aware output formatting (issue 5244)
- Date.parse() free function
- Non-ISO string formats

## Affected paths

Expected:
- `crates/ir/src/lowered/program_builtins.rs` — add date_string_arg and date_multi_arg helpers
- `crates/ir/src/lowered/resolver_expr.rs` — wire new paths in Date constructor lowering
- `crates/backend-wasm/src/runtime_builtins_host.rs` — add `$host_date_parse` host shim for string parsing
- `crates/backend-wasm/src/runtime_fn.rs` / `runtime_fn_impl.rs` — register new RuntimeFn

## Acceptance criteria

- [ ] `new Date("2024-01-01")` lowers through host shim or lowering
- [ ] `new Date(2024, 0, 1)` lowers to multi-argument runtime path
- [ ] Existing `new Date(epochMs)` and `new Date()` paths unchanged
- [ ] Focused fixture covers string and multi-argument Date construction

## Validation

```sh
cargo nextest run -E 'test(date)'
```


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
