# Child Assignment: 233 module lowered IR slice

Child run id: `233-module-lowered-ir-20260428T103829Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-233-module-lowered-ir-20260428T103829Z`
Branch: `agent/233-module-lowered-ir-20260428T103829Z`
Parent branch at assignment: `master` @ `d7de6e1`

You are not alone in this repository. Other agents are active in separate worktrees; do not revert or overwrite changes made by others, do not touch parent `master`, and stay within this assignment.

## Assigned Issue List

1. `issues/open/233-emit-static-es-module-bindings.md`

## Objective

Make one safe forward step toward explicit lowered module IR for issue 233. The previous slice made `RuntimeLinkPlan` scan `LoweredProgram.modules`; this slice should add or harden the explicit lowered representation or a compiler test that populates it without claiming full runtime execution parity.

Preferred slice:

- Add a narrow `LoweredProgram.modules` population path for simple source modules with literal named exports, or a focused IR/compiler test that proves the representation can carry module export statements.
- Ensure runtime link-plan tests from the previous slice remain green.
- Preserve the temporary static named import build rewrite and all current module build smokes.

Do not close issue 233 unless all acceptance criteria are fully satisfied. Do not edit issue 234.

## Allowed Files

- `crates/ir/src/`
- `crates/compiler/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/m9_modules.rs`
- `fixtures/module-system/`
- `issues/open/233-emit-static-es-module-bindings.md`
- `current-state.md` only if behavior facts changed
- `reports/agents/233-module-lowered-ir-20260428T103829Z/`
- `reports/runs/233-module-lowered-ir-20260428T103829Z/`

## Forbidden Files

- `docs/`
- Coverage artifacts or issue 060 files
- Logical-assignment files or issue 236/237 files
- JSON files or issue 052 files
- Parent branch or any other agent worktree

## Required Validation

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-compiler
cargo nextest run -p ts2wasm-backend-wasm
cargo nextest run -p ts2wasm-cli module
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-233-lowered-entry.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-233-lowered-alias.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-shadow.ts -o /tmp/ts2wasm-233-lowered-shadow.wasm
scripts/manager check-issue-health
scripts/manager check-agent-state
```

Run full `scripts/manager nextest` only if attempting to close issue 233 or changing broad runtime behavior.

## Reporting

- Write `reports/runs/233-module-lowered-ir-20260428T103829Z/cycle_report.md`.
- Write/validate `test_report.json` when practical.
- Attempt `scripts/manager discord-report --run-id 233-module-lowered-ir-20260428T103829Z`; if unavailable, commit deferred payload/error artifacts.
- Commit all validated useful work.
- Merge latest parent `master` before final event.
- End with exactly one parent event line:

```text
PARENT_EVENT: PROGRESS issue=233 branch=agent/233-module-lowered-ir-20260428T103829Z commit=<hash> validation="<summary>" report=reports/runs/233-module-lowered-ir-20260428T103829Z/cycle_report.md merge_request=yes
```
