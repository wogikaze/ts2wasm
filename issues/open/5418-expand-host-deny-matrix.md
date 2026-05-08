---
id: 5418
title: "W7: Expand host-deny test matrix and WASI-only audit"
type: feature
area: cli
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Expand the host-deny test matrix in `m11_host_deny.rs` to cover all runtime function categories, add negative tests for each Node host import, and create a WASI-only runtime function audit that verifies standalone purity.

## Problem

- `m11_host_deny.rs` has only 12 test functions — does not cover all runtime function categories
- No systematic audit exists to verify that WASI-only runtime functions don't accidentally introduce Node host dependencies
- Each new RuntimeFn addition needs manual classification, risking silent host-import creep

Problem: Host-deny coverage limited to 12 tests; WASI-only audit missing.

## Current failure

```sh
# Current manifest emission shows standalone: true for known fixtures
# But unknown whether all runtime functions preserve standalone purity
```

## Desired final state

- `m11_host_deny.rs` covers all runtime function categories as standalone or host-required
- Each Node host import has a negative test verifying rejection under `--host-deny`
- A script or test validates that all WASI-only runtime functions compile standalone
- Capability manifest reason strings are audited for correctness

## Scope

In scope:

- [ ] Audit all runtime functions in `runtime_fn.rs` for WASI-only vs host-required classification
- [ ] Add host-deny test for each runtime function category (at least one representative per category)
- [ ] Add negative test: each Node host import must be rejected under `--host-deny`
- [ ] Add standalone fixture for each WASI-only category under `--host-deny`
- [ ] Add test: all standalone fixtures pass `--host-deny` without Node host imports
- [ ] Update `crates/shared/src/capability.rs` if schema needs reason-string improvements
- [ ] Verify `--emit-manifest` output matches actual wasm imports for all fixture categories

Out of scope:

- Implementing new runtime functions or host imports
- Changing the capability manifest schema significantly
- Gateway/CI integration (separate infra issue)
- W7 policy checklist documentation (already covered in w7-remaining.md)

## Affected paths

Expected:

- `crates/cli/tests/m11_host_deny.rs` — add test functions
- `crates/shared/src/capability.rs` — minor reason-string improvements if needed
- `fixtures/builtins-and-io/` — add standalone fixture if needed for a new category

Do not touch:

- `crates/frontend/` — parser out of scope
- `crates/ir/` — IR out of scope
-  — runtime function code out of scope
- `crates/backend-wasm/src/runtime_fn.rs` — catalog out of scope

## Acceptance criteria

- [ ] At least one standalone fixture per runtime function category passes `--host-deny`
- [ ] Each Node host import (fs, crypto, path, process, http) has a test verifying rejection under `--host-deny`
- [ ] `m11_host_deny.rs` has >= 20 test functions (up from 12)
- [ ] `--emit-manifest` on all standalone fixtures shows `standalone: true` without unexpected host imports
- [ ] All existing m11_host_deny tests still pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -- m11_host_deny
```

Impacted commands:

```sh
# Verify manifest for all categories
ts2wasm build fixtures/builtins-and-io/math-floor.ts --emit-manifest
cat output.wasm.manifest.json | jq '.standalone'
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

- Current runtime function categories: Math, String, Array, Object, JSON, Date, RegExp, Map/Set, BigInt, Error, Global, console
- Each category has at least one function that should be WASI-only (uses no Node APIs)
- For each category, identify the test fixture and verify standalone purity
- Use the existing `compile_fixture_with_host_deny` helper pattern in m11_host_deny.rs
- Look at `runtime_fn_impl.rs` for the `manifest_name()` and `emission_order()` patterns to understand category classification
