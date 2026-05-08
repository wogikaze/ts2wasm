---
id: 5232
title: "W0: fixture-ize RuntimeLinkPlan linker structure tests"
type: cleanup
area: tests
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Convert the current RuntimeLinkPlan linker structure tests (in runtime_link_plan.rs) into snapshot-style fixture tests. Currently the tests enumerate expected RuntimeFn/HostImport/Capability sets inline. A fixture-based approach makes it easier to detect regressions when the linker plan changes.

## Problem

docs/12-coding-standard.md S20 lists "linker snapshot fixture 化" as a P1 task. Current linker tests duplicate assertion logic inline and are hard to maintain as the RuntimeLinkPlan evolves.

Each time a RuntimeFn dependency changes, the affected linker test must be manually updated.

## Scope

In scope:
- Design the fixture schema (JSON expected output for required RuntimeFn/imports/capabilities/runtime strings)
- Add a test runner that compiles a fixture file, extracts the RuntimeLinkPlan, and compares to the fixture
- Migrate existing inline linker tests to the fixture approach
- Keep existing inline tests until migration is complete

Out of scope:
- Modifying the RuntimeLinkPlan algorithm itself
- Changing the manifest output format
- WASM binary emission changes

## Affected paths

Expected:
- crates/backend-wasm/src/runtime_link_plan.rs
- crates/cli/tests/linker_structure.rs (new)
- fixtures/linker/

## Acceptance criteria

- [x] Fixture schema is documented
- [x] At least 5 fixture programs have linker snapshot tests
- [x] CI runs linker snapshot comparison
- [x] Existing inline tests still pass during migration

## Validation

```
cargo fmt --all --check
cargo nextest run -E 'test(linker)'
```



## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in issues/open/. Implemented by agent (commit fd789fa8).
