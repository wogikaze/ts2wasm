# eval/function PR D report: optional eval declaration global

Run ID: `eval-function-pr-d-optional-eval-declaration-global`
Date: 2026-05-19

## Slice

Added focused coverage for optional eval static declaration landing.

## Change

- Added `optional-eval-static-declaration-global.ts`.
- Registered it in `fixtures/catalog.yaml`.
- Added Node differential and host-deny standalone test entries.
- The fixture checks that unshadowed `eval?.("var ...; function ...")` uses indirect/global eval semantics, landing `var` and function declarations on `globalThis` while leaving a caller local untouched.

## Evidence

- PASS: `node fixtures/core-semantics/optional-eval-static-declaration-global.ts`
  - observed stdout: `8`, `7`, `8`, `number`, `caller`
- PASS: `TS2WASM_RUN_M2_NODE_DIFF=1 CARGO_TARGET_DIR=/tmp/ts2wasm-eval-clean-verify-target cargo test -p ts2wasm-cli --test m2_node_diff static_optional_eval_declaration_global_fixture_matches_node_output -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-clean-verify-target cargo test -p ts2wasm-cli --test m11_host_deny static_optional_eval_declaration_global_declares_no_node_host_eval_capability -- --nocapture`
  - run from clean worktree `/tmp/ts2wasm-eval-clean-verify` because the main worktree had unrelated dirty backend/array changes.
- PASS: scoped `git diff --check`

## Remaining

Re-run the host-deny standalone/manifest check once the unrelated dirty backend/array worktree changes are fixed or isolated. This slice is coverage for optional eval global declaration behavior, not the full canonical global eval environment model.
