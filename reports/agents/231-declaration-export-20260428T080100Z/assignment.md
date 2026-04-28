# Assignment

- Run ID: `231-declaration-export-20260428T080100Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-231-declaration-export-20260428T080100Z`
- Branch: `agent/231-declaration-export-20260428T080100Z`
- Issue: `issues/open/231-parse-static-es-module-declarations.md`
- Slice: parser-only static declaration export support for the smallest safe form, preferably `export const value = 1;` as an explicit AST/module declaration form that preserves exported local name and spans.

## Scope

- Implement the narrow static declaration export parser/AST support needed for `export const value = 1;`.
- Preserve exported local name and spans.
- Keep `export default`, function/class declaration exports, module graph/resolution/lowering/backend/runtime execution, and broad fixture conversion out of scope.
- Add only minimal downstream unsupported guards if required for workspace compile.

## Allowed Files

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser.rs`
- Frontend parser tests
- Minimal downstream unsupported guards only if required:
  - `crates/compiler/src/`
  - `crates/ir/src/`
  - `crates/cli/tests/m9_modules.rs`
  - `fixtures/module-system/`
- `issues/open/231-parse-static-es-module-declarations.md`
- `reports/runs/231-declaration-export-20260428T080100Z/**`
- `reports/agents/231-declaration-export-20260428T080100Z/assignment.md`

## Forbidden Files

- `crates/backend-wasm/**`
- `crates/runtime-abi/**`
- `docs/**`
- Coverage artifacts
- Unrelated issue files

## Expected Validation

- `cargo fmt --all --check`
- `cargo nextest run -p ts2wasm-frontend`
- Targeted CLI module guard tests if added/updated
- `cargo check --workspace`
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`
- Full `cargo nextest run` if feasible before reporting merge

## Reporting

- Attempt `scripts/manager discord-report --run-id 231-declaration-export-20260428T080100Z`.
- If webhook env is absent or fails, save deferred payload/report under `reports/runs/231-declaration-export-20260428T080100Z/`.
- Commit validated progress on the assigned branch.
- Do not merge to parent.
