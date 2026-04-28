# Assignment: 231 namespace re-export parser slice

Run ID: `231-namespace-reexport-20260428T074900Z`
Branch: `agent/231-namespace-reexport-20260428T074900Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-231-namespace-reexport-20260428T074900Z`
Issue: `issues/open/231-parse-static-es-module-declarations.md`

## Scope

Implement parser-only support for namespace re-export declarations such as:

```ts
export * as ns from "./module-source";
```

Preserve the exported namespace name, module specifier, and spans in an explicit AST representation.

Out of scope: module graph/resolution/lowering/backend/runtime, dynamic import, export default, declaration exports, and forms not implemented by this slice. Unsupported forms must remain issue-linked diagnostics.

## Allowed Files

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser.rs`
- Frontend tests in those modules
- Downstream module diagnostic guards only if required:
  - `crates/compiler/src/`
  - `crates/ir/src/`
  - `crates/cli/tests/m9_modules.rs`
  - `fixtures/module-system/`
- `issues/open/231-parse-static-es-module-declarations.md`
- `reports/runs/231-namespace-reexport-20260428T074900Z/**`
- `reports/agents/231-namespace-reexport-20260428T074900Z/assignment.md`

## Forbidden Files

- `crates/backend-wasm/**`
- `crates/runtime-abi/**`
- `docs/**`
- Coverage artifacts
- Unrelated issue files

## Expected Validation

- `cargo fmt --all --check`
- `cargo nextest run -p ts2wasm-frontend`
- Targeted CLI module guard tests added or updated if required
- `cargo check --workspace`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`
- Full `cargo nextest run` if feasible before reporting merge

## Reporting

Attempt `scripts/manager discord-report --run-id 231-namespace-reexport-20260428T074900Z`. If webhook configuration is absent or reporting fails, save deferred payload/report under the run directory and continue.
