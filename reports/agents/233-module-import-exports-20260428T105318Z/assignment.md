# Assignment: issue 233 named imports from lowered module exports

Child run id: `233-module-import-exports-20260428T105318Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-233-module-import-exports-20260428T105318Z`
Branch: `agent/233-module-import-exports-20260428T105318Z`
Assigned issues: `233`
Issue order: `233`

## Objective

Continue issue 233 by making a narrow, explicit step toward named imports reading from lowered module exports instead of relying solely on temporary lexical literal rewrites.

## Allowed files

- `crates/ir/src/**`
- `crates/compiler/src/**`
- `crates/backend-wasm/src/**`
- `crates/cli/tests/**`
- `fixtures/module-system/**`
- `issues/open/233-emit-static-es-module-bindings.md`
- `reports/runs/233-module-import-exports-20260428T105318Z/`
- `reports/agents/233-module-import-exports-20260428T105318Z/assignment.md`

## Forbidden files

- `docs/**`
- `artifacts/coverage/**`
- issue 236 logical-assignment fixtures
- `crates/runtime-abi/**` unless absolutely required and explicitly justified in the cycle report

## Required workflow

1. Read `.agents/prompts/autonomous-child-worker.md`, `AGENTS.md`, issue 233, `docs/12-coding-standard.md`, and this assignment.
2. Inspect the current issue 233 progress around:
   - `LoweredProgram.modules`
   - `LoweredStmt::Export`
   - `StaticNamedImportBinding`
   - runtime link-plan module helper selection
3. Implement one smallest safe slice. Preferred slices:
   - represent named import reads explicitly in resolved/lowered IR using module IDs and export names, or
   - add backend/compiler tests proving import reads can be connected to the populated `LoweredProgram.modules`, or
   - replace a narrow part of the temporary build rewrite with an explicit module-export read path.
4. Keep runtime/capability helper selection through the existing runtime/link-plan catalog. Do not hard-code host imports.
5. Preserve existing static module build smokes: `static-entry.ts`, alias, and shadow.
6. Run validation:
   - `cargo fmt --all --check`
   - `cargo nextest run -p ts2wasm-ir`
   - `cargo nextest run -p ts2wasm-backend-wasm`
   - `cargo nextest run -p ts2wasm-compiler` or targeted compiler tests plus justification
   - `cargo nextest run -p ts2wasm-cli module`
   - `cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-233-import-exports-entry.wasm`
   - alias/shadow build commands if touched
   - `scripts/manager check-issue-health`
   - `scripts/manager check-agent-state`
7. Write `reports/runs/233-module-import-exports-20260428T105318Z/cycle_report.md` and schema-valid `test_report.json`.
8. Attempt Discord report; if webhook is unavailable, save payload/error artifacts and continue.
9. Commit all validated changes. Request merge from parent.

## Expected outcome

Use `PROGRESS` unless every issue 233 acceptance criterion is met and full close workflow is complete.

## Parent event

End with exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=233 branch=agent/233-module-import-exports-20260428T105318Z commit=<hash> merge_request=yes`

Use `BLOCKED` with evidence if the next safe step requires a larger runtime design.
