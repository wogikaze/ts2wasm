# Assignment: issue 233 module initialization runtime slice

Child run id: `233-module-init-runtime-20260428T114113Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-233-module-init-runtime-20260428T114113Z`
Branch: `agent/233-module-init-runtime-20260428T114113Z`
Assigned issues: `233`
Issue order: `233`

## Objective

Continue issue 233 from current master. Named imports now lower to `PropertyGet(ModuleLoad { module_id }, export_name)` and source modules populate `LoweredProgram.modules`. Implement one narrow, validated step toward dependency-order module initialization/runtime execution.

## Allowed files

- `crates/compiler/src/**`
- `crates/ir/src/**`
- `crates/backend-wasm/src/**`
- `crates/cli/tests/**`
- `fixtures/module-system/**`
- `issues/open/233-emit-static-es-module-bindings.md`
- `reports/runs/233-module-init-runtime-20260428T114113Z/`
- `reports/agents/233-module-init-runtime-20260428T114113Z/assignment.md`

## Forbidden files

- `artifacts/coverage/**`
- `scripts/**`
- `fixtures/core-semantics/**`
- JSON fixtures
- docs
- `crates/runtime-abi/**` unless strictly required and justified in the report

## Required workflow

1. Read `.agents/prompts/autonomous-child-worker.md`, `AGENTS.md`, issue 233, `docs/12-coding-standard.md`, and this assignment.
2. Inspect current `LoweredProgram.modules`, `ModuleInitializationStep`, `RuntimeLinkPlan`, and emitted module helpers.
3. Implement one smallest safe slice toward module initialization. Preferred safe slices:
   - encode dependency-first initialization steps into lowered metadata or compiler output, or
   - emit/call module initialization helpers for lowered module export statements in a way that still passes existing build smokes, or
   - add backend/compiler tests proving dependency-first module export statements are reachable through runtime helper selection.
4. Do not claim execution semantic parity unless a Node/iwasm fixture actually runs and matches.
5. Preserve existing static module build smokes: `static-entry.ts`, alias, and shadow.
6. Run validation:
   - `cargo fmt --all --check`
   - `cargo nextest run -p ts2wasm-ir`
   - `cargo nextest run -p ts2wasm-backend-wasm`
   - `cargo nextest run -p ts2wasm-compiler` or targeted compiler tests plus justification
   - `cargo nextest run -p ts2wasm-cli module`
   - `cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-233-init-runtime-entry.wasm`
   - alias/shadow build commands if touched
   - `scripts/manager check-issue-health`
   - `scripts/manager check-agent-state`
7. Write `reports/runs/233-module-init-runtime-20260428T114113Z/cycle_report.md` and schema-valid `test_report.json`.
8. Attempt `scripts/manager discord-report --run-id 233-module-init-runtime-20260428T114113Z`; if webhook is unavailable, save payload/error artifacts and continue.
9. Commit all validated changes. Request merge from parent.

## Expected outcome

Use `PROGRESS` unless every issue 233 acceptance criterion is met and the full close workflow is complete.

## Parent event

End with exactly one parent event line:

`PARENT_EVENT: PROGRESS issue=233 branch=agent/233-module-init-runtime-20260428T114113Z commit=<hash> merge_request=yes`

Use `BLOCKED` with evidence if runtime initialization requires a broader design decision.
