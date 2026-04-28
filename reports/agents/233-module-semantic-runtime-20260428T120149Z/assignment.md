# Child Assignment: 233-module-semantic-runtime-20260428T120149Z

- Parent cycle: autonomous multi-worktree compiler development
- Worktree: `/home/wogikaze/wgkz/ts2wasm-233-module-semantic-runtime-20260428T120149Z`
- Branch: `agent/233-module-semantic-runtime-20260428T120149Z`
- Assigned issues: `233`
- Issue order: `233`

## Scope

Complete or make validated progress on issue 233, focusing on the remaining static ES module runtime semantics after the merged module-init helper slice.

Primary target:

- Add the narrowest Node/iwasm semantic evidence for static named import/export execution if the current runtime path already supports it.
- If it fails, isolate the smallest missing runtime/lowering/backend step, implement only that step, and add regression coverage.
- Preserve the explicit distinction between build smoke and semantic parity.
- Close issue 233 only if all acceptance criteria are satisfied and full required validation passes; otherwise record PROGRESS with evidence and leave the issue open.

## Allowed Files

- `issues/open/233-emit-static-es-module-bindings.md`
- `issues/done/233-emit-static-es-module-bindings.md` only if DONE criteria are fully met
- `issues/index.md`
- `current-state.md` only if observed facts change
- `crates/compiler/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/module-system/`
- `reports/agents/233-module-semantic-runtime-20260428T120149Z/`
- `reports/runs/233-module-semantic-runtime-20260428T120149Z/`

## Forbidden Files

- `docs/`
- `crates/runtime-abi/` unless a strictly necessary ABI constant change is backed by a narrow test and explained in the report
- Any files owned by other active branches unless required for merge conflict resolution inside this worktree

## Expected Validation

Run the narrowest failing reproduction first, then at minimum:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-backend-wasm
cargo nextest run -p ts2wasm-compiler
cargo nextest run -p ts2wasm-cli module
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-233-semantic-entry.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-233-semantic-alias.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-shadow.ts -o /tmp/ts2wasm-233-semantic-shadow.wasm
scripts/manager check-issue-health
scripts/manager check-agent-state
```

If closing issue 233, also run:

```sh
cargo nextest run
scripts/manager check-repo-smoke
```

## Reporting

- Write `reports/runs/233-module-semantic-runtime-20260428T120149Z/cycle_report.md`.
- Write a schema-valid `reports/runs/233-module-semantic-runtime-20260428T120149Z/test_report.json`.
- Attempt `scripts/manager discord-report --run-id 233-module-semantic-runtime-20260428T120149Z`; if webhook configuration is absent or fails, commit deferred payload/error evidence and continue.
- Commit validated work on the assigned branch.

End with exactly one parent event line:

```text
PARENT_EVENT: DONE issue=233 branch=agent/233-module-semantic-runtime-20260428T120149Z commit=<hash> merge_request=yes
PARENT_EVENT: PROGRESS issue=233 branch=agent/233-module-semantic-runtime-20260428T120149Z commit=<hash> merge_request=yes
PARENT_EVENT: BLOCKED issue=233 branch=agent/233-module-semantic-runtime-20260428T120149Z commit=<hash-or-none> reason=<short-reason>
```
