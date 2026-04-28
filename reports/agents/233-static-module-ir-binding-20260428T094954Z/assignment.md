# Child Assignment: 233 static module IR binding

Child run id: `233-static-module-ir-binding-20260428T094954Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-233-static-module-ir-binding-20260428T094954Z`
Branch: `agent/233-static-module-ir-binding-20260428T094954Z`
Parent branch at assignment: `master` @ `0b4cfb1`

You are not alone in this repository. Other agents may be editing other worktrees; do not revert, overwrite, or depend on changes outside this branch. Do not merge to `master`.

## Assigned Issue List

1. `issues/open/233-emit-static-es-module-bindings.md`

## Objective

Make one safe forward step from the current temporary compiler rewrite toward explicit module binding representation. Prefer a narrow, validated slice over broad module semantics. Do not claim issue 233 done unless every acceptance criterion in the issue is actually satisfied.

Suggested slice:

- Introduce explicit resolved/lowered representation for simple static named import/export bindings, or move the current graph-backed named import rewrite behind a clearly named module-binding lowering helper with tests proving imported values are resolved from the source module export rather than importer lexical globals.
- Add or keep regression coverage for a case where the importer has a same-named local/global and the import must use the source module binding.
- Preserve the current `static-entry.ts` and `static-entry-alias.ts` build behavior.

If the explicit IR step is too large after investigation, commit a smaller progress slice that hardens the current named import/export lowering with source-backed shadowing tests and issue evidence.

## Allowed Files

- `crates/compiler/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/m9_modules.rs`
- `fixtures/module-system/`
- `issues/open/233-emit-static-es-module-bindings.md`
- `current-state.md` only if implementation facts changed
- `reports/agents/233-static-module-ir-binding-20260428T094954Z/`
- `reports/runs/233-static-module-ir-binding-20260428T094954Z/`

## Forbidden Files

- `docs/`
- `crates/runtime-abi/` unless you first record a blocker explaining why the ABI change is unavoidable
- Any issue other than 233
- Parent branch or any other agent worktree

## Required Validation

Run narrow commands first, then the issue gates relevant to your touched paths:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-backend-wasm
cargo nextest run -p ts2wasm-cli module
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-esm-233-ir.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry-alias.ts -o /tmp/ts2wasm-esm-233-alias-ir.wasm
scripts/manager check-issue-health
scripts/manager check-agent-state
```

If you change runtime execution semantics, add direct Node/iwasm evidence, but do not move issue 234 unless explicitly completing its dependency gate.

## Reporting

- Write `reports/runs/233-static-module-ir-binding-20260428T094954Z/cycle_report.md`.
- Write a machine-readable `test_report.json` when practical and validate it with the repo schema.
- Attempt `scripts/manager discord-report --run-id 233-static-module-ir-binding-20260428T094954Z`; if the webhook is unavailable, save the deferred payload/error and continue.
- Commit all validated useful work.
- End with exactly one parent event line, for example:

```text
PARENT_EVENT: PROGRESS issue=233 branch=agent/233-static-module-ir-binding-20260428T094954Z commit=<hash> validation="<summary>" report=reports/runs/233-static-module-ir-binding-20260428T094954Z/cycle_report.md merge_request=yes
```

Use `DONE` only if the issue file is moved to `issues/done/`, index regenerated, full close requirements met, and all acceptance criteria are verified.
