---
id: 5234
title: "W0: implement host-deny and auditable E2E manifest verification"
type: feature
area: cli
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement a --host-deny CLI flag and an end-to-end manifest verification pass. When --host-deny node is passed, the compiler must reject programs that require any Node.js host import. The capability manifest must be verifiable against the actual emitted wasm imports.

## Problem

docs/12-coding-standard.md S20 lists "host-deny and auditable E2E manifest" as a P1 task.

Current gaps:
- No CLI flag to reject Node host imports at compile time
- Manifest is generated but not verified against the actual emitted imports
- An E2E test can produce a manifest claiming standalone: true while the wasm binary contains host imports
- Auditing capability requires manual inspection

## Scope

In scope:
- Add --host-deny <host> CLI option (e.g., --host-deny node)
- Cross-check capability manifest against actual wasm imports after emission
- Add E2E test that confirms --host-deny node rejects Node-host programs
- Add E2E test that confirms manifest matches actual wasm imports
- Update current-state.md

Out of scope:
- Host-deny for WASI imports (always expected)
- Network-level deny (firewall, seccomp)
- Manifest schema changes

## Affected paths

Expected:
- crates/cli/src/lib.rs or main.rs
- crates/compiler/src/lib.rs
- crates/cli/tests/

## Acceptance criteria

- [x] --host-deny node produces a compile error for Node-host programs
- [x] --host-deny node passes for standalone programs
- [x] Manifest verification detects mismatches between manifest and actual wasm imports
- [x] E2E tests cover both pass and fail cases

## Validation

```
cargo fmt --all --check
cargo nextest run -E 'test(host_deny|manifest)'
```

## Docs / current-state / issue sync

Current state:
- [x] updated: current-state.md



## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in issues/done/. Implemented by child agent.
