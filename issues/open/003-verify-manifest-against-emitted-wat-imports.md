# Verify manifest against emitted WAT imports (audit reopened #003)

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Completed**: 2026-04-26
**ID**: 003
**Type**: test
**Area**: wasi/tests
**Priority**: P0
**Depends on**: 002
**Orchestration class**: implementation-ready

Problem: A manifest is only useful as a gate if it matches emitted WAT/wasm imports. The current project needs a test that cross-checks the manifest and the actual imports.

Scope:

- Add helper to extract WAT imports from emitted text.
- Verify `console.log` uses `fd_write` and manifest says stdout is required.
- Verify `fs.readFileSync(0, "utf8")` uses `fd_read` and manifest says stdin is required.
- Verify standalone fixtures have no Node host imports.
- Verify Node shim fixtures mark `node_host.required = true`.

Acceptance Criteria:

- [ ] Manifest WASI imports match WAT imports.
- [ ] Standalone target fails the test if host imports leak in.
- [ ] Node-host-required cases are explicitly represented.
- [ ] Test names can be used as Gate C/F evidence.

Validation:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(manifest)'
```

Completion evidence:

- `cargo fmt --all --check`: pass
- `cargo nextest run --filterset "test(manifest)"` (with `CARGO_TARGET_DIR=$env:TEMP\ts2wasm-target`): pass
- Tests passed: 9 manifest tests.

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/003-verify-manifest-against-emitted-wat-imports.md` before this move
- `issues/open/003-verify-manifest-against-emitted-wat-imports.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
