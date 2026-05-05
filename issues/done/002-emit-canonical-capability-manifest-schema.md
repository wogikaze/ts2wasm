---
id: 002
title: "Emit canonical capability manifest schema"
type: feature
area: abi/wasi
class: implementation-ready
priority: P0
depends_on: []
blocks: [003, 006, 023]
status: done
created: 2026-04-26
updated: 2026-04-26
completed: 2026-04-26
---

## Problem

docs define a canonical manifest schema with `schema_version`, `target`, `standalone`, `wasi`, `node_host`, and `capability_reasons`, but the implementation appears to emit a transitional `target/imports/capabilities/runtime` shape. `crates/shared/src/capability.rs` also overlaps with this responsibility.

## Scope

- Use the canonical manifest schema as the only primary schema.
- Replace or adapt backend-local manifest types to use shared manifest types.
- Map `console.log` to `wasi.stdout = true`.
- Map `fs.readFileSync(0, "utf8")` to `wasi.stdin = true`.
- Mark Node host dependency with `standalone = false` and `node_host.required = true`.
- Emit capability reasons.

## Out of scope

- Full host shim generation.
- Full filesystem preopen implementation.
- Full Node API compatibility.

## Acceptance Criteria

- [x] `--emit-manifest` emits the canonical schema.
- [x] Deprecated alias `--emit-capabilities`, if retained, emits identical JSON.
- [x] Transitional schema is not treated as the authoritative format.
- [x] Manifest fixtures cover stdout, stdin, standalone, and Node-host-required cases.

## Validation

```sh
cargo fmt --all --check
cargo nextest run
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/hello.wasm --emit-manifest /tmp/hello.manifest.json
jq '.schema_version, .standalone, .wasi.stdout, .node_host.required' /tmp/hello.manifest.json
```

## Completion evidence

- All validation commands passed (2026-04-26)
- `--emit-manifest` outputs canonical schema with `schema_version: 1`, `standalone: true`, `wasi.stdout: true`, `node_host.required: false` for hello.ts
- Test suite `cargo nextest run manifest` passes all 5 tests
- Implementation in `crates/backend-wasm/src/capability_manifest.rs` uses `CapabilityManifest` from `crates/shared`
- Deprecated alias `--emit-capabilities` handled in main.rs (same code path)

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/002-emit-canonical-capability-manifest-schema.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
