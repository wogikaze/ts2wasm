# Emit canonical capability manifest schema

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 002
**Type**: feature
**Area**: abi/wasi
**Priority**: P0
**Depends on**: 001
**Orchestration class**: implementation-ready

Problem: docs define a canonical manifest schema with `schema_version`, `target`, `standalone`, `wasi`, `node_host`, and `capability_reasons`, but the implementation appears to emit a transitional `target/imports/capabilities/runtime` shape. `crates/shared/src/capability.rs` also overlaps with this responsibility.

Scope:
- Use the canonical manifest schema as the only primary schema.
- Replace or adapt backend-local manifest types to use shared manifest types.
- Map `console.log` to `wasi.stdout = true`.
- Map `fs.readFileSync(0, "utf8")` to `wasi.stdin = true`.
- Mark Node host dependency with `standalone = false` and `node_host.required = true`.
- Emit capability reasons.

Out of scope:
- Full host shim generation.
- Full filesystem preopen implementation.
- Full Node API compatibility.

Acceptance Criteria:
- [ ] `--emit-manifest` emits the canonical schema.
- [ ] Deprecated alias `--emit-capabilities`, if retained, emits identical JSON.
- [ ] Transitional schema is not treated as the authoritative format.
- [ ] Manifest fixtures cover stdout, stdin, standalone, and Node-host-required cases.

Validation:
```sh
cargo fmt --all --check
cargo nextest run
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/hello.wasm --emit-manifest /tmp/hello.manifest.json
jq '.schema_version, .standalone, .wasi.stdout, .node_host.required' /tmp/hello.manifest.json
```

