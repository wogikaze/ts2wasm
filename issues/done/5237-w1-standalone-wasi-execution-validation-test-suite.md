---
id: 5237
title: "W1: standalone WASI execution validation test suite"
type: test
area: tests
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Create a comprehensive validation test suite that confirms standalone WASI programs require zero Node.js host imports. Each test compiles a fixture, emits the manifest and wasm binary, and verifies host-deny (no Node.js imports required).

## Problem

W1 (Standalone WASI execution) requires that certain programs can run without Node.js. Currently there is no systematic validation that:
- A given fixture produces standalone: true in the manifest
- The emitted wasm binary contains zero Node.js host imports
- Running under iwasm succeeds without Node.js

## Scope

In scope:
- Create a test harness that verifies standalone execution per fixture
- Define a standalone fixture manifest/catalog
- Verify --host-deny node passes for all standalone fixtures
- Verify capability manifest standalone: true for standalone fixtures
- Add CI gate for standalone validation

Out of scope:
- Node.js host-required fixtures (test those only under Node-enabled mode)
- Performance benchmarking of standalone vs Node mode

## Affected paths

Expected:
- crates/cli/tests/m_standalone_wasi.rs (new)
- fixtures/standalone-wasi/
- scripts/check/

## Acceptance criteria

- [x] 10+ standalone fixtures pass host-deny validation
- [x] CI includes standalone validation gate
- [x] Each fixture's manifest confirms standalone: true
- [x] Regression: adding a Node host import to a standalone fixture is caught

## Validation

```
cargo fmt --all --check
cargo nextest run -E 'test(standalone|wasi)'
```

