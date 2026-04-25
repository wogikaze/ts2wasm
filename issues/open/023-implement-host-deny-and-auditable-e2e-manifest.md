# Implement host-deny and auditable E2E manifest

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 023
**Type**: feature
**Area**: security/capability
**Priority**: P1
**Depends on**: 002, 003
**Orchestration class**: implementation-ready

Problem: host-deny / capability manifest "auditable E2E" is planned but not implemented. docs/06 specifies required test classes for capability audit. docs/11 Gate C requires manifest emission and Gate F requires standalone verification.

Scope:

- Implement host-deny mode to reject Node host imports.
- Add E2E tests for capability manifest audit.
- Verify standalone programs have no Node host imports.
- Verify host-required programs are explicitly marked.
- Align with docs/06 required test classes.

Acceptance Criteria:

- [ ] Host-deny mode rejects Node host imports.
- [ ] E2E tests verify manifest matches actual imports.
- [ ] Standalone programs pass host-deny test (Gate F).
- [ ] Host-required programs are correctly marked in manifest.
- [ ] Capability audit tests cover required classes from docs/06.

Validation:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(host_deny)'
cargo run -q -p ts2wasm-cli -- build fixtures/basics-hello/hello.ts -o /tmp/hello.wasm --emit-manifest /tmp/hello.manifest.json
```
