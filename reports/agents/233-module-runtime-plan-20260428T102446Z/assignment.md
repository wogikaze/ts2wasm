# Child Assignment: 233 module runtime plan slice

Child run id: `233-module-runtime-plan-20260428T102446Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-233-module-runtime-plan-20260428T102446Z`
Branch: `agent/233-module-runtime-plan-20260428T102446Z`
Parent branch at assignment: `master` @ `849fc10`

You are not alone in this repository. Other agents are active in separate worktrees; do not revert or overwrite changes made by others, do not touch parent `master`, and stay within this assignment.

## Assigned Issue List

1. `issues/open/233-emit-static-es-module-bindings.md`

## Objective

Make one safe forward step from the current module graph/init-order contracts toward backend/runtime module initialization without claiming execution parity.

Preferred slice:

- Add a narrow IR/backend contract or link-plan test proving ES module initialization/export helpers are included only when a future explicit module init/export representation is present.
- If a small explicit lowered representation is feasible, add it behind tests without wiring broad runtime execution.
- Preserve all existing static module build smokes and CommonJS module-cache behavior.

Avoid broad runtime execution changes unless they can be validated narrowly. Issue 234 remains blocked until issue 233 is truly complete; do not move or edit issue 234.

## Allowed Files

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/compiler/src/`
- `crates/cli/tests/m9_modules.rs`
- `fixtures/module-system/`
- `issues/open/233-emit-static-es-module-bindings.md`
- `current-state.md` only if implementation facts changed
- `reports/agents/233-module-runtime-plan-20260428T102446Z/`
- `reports/runs/233-module-runtime-plan-20260428T102446Z/`

## Forbidden Files

- `docs/`
- Logical-assignment files or issue 236 files
- Coverage artifacts or issue 060 files
- JSON files or issue 052 files
- `crates/runtime-abi/` unless a blocker note explains why it is unavoidable
- Parent branch or any other agent worktree

## Required Validation

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-backend-wasm
cargo nextest run -p ts2wasm-compiler
cargo nextest run -p ts2wasm-cli module
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-233-runtime-entry.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-233-runtime-alias.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-shadow.ts -o /tmp/ts2wasm-233-runtime-shadow.wasm
scripts/manager check-issue-health
scripts/manager check-agent-state
```

Run full `scripts/manager nextest` if attempting to close issue 233 or if runtime helper behavior changes broadly.

## Reporting

- Write `reports/runs/233-module-runtime-plan-20260428T102446Z/cycle_report.md`.
- Write/validate `test_report.json` when practical.
- Attempt `scripts/manager discord-report --run-id 233-module-runtime-plan-20260428T102446Z`; if unavailable, commit deferred payload/error artifacts.
- Commit all validated useful work.
- Merge latest parent `master` before final event.
- End with exactly one parent event line:

```text
PARENT_EVENT: PROGRESS issue=233 branch=agent/233-module-runtime-plan-20260428T102446Z commit=<hash> validation="<summary>" report=reports/runs/233-module-runtime-plan-20260428T102446Z/cycle_report.md merge_request=yes
```
