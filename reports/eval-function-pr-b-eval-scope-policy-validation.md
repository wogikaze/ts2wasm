# eval/function PR B report: EvalFragmentPlan scope-mode validation

Run ID: `eval-function-pr-b-eval-scope-policy-validation`
Date: 2026-05-19

## Slice

Moved direct/indirect eval scope-mode consistency checking into `EvalFragmentPlan`.

## Change

- Added `EvalFragmentPlan::expected_scope_mode()` and `scope_mode_is_consistent()`.
- Static eval AOT expansion now validates scope mode and host policy for direct, indirect, and non-string static eval fragments before expansion.
- Runtime eval lowering now rejects plans where `EvalKind::Direct` is not caller-scope or `EvalKind::Indirect` is not global-scope.
- Added an IR unit test covering direct caller scope, indirect global scope, and an intentionally inconsistent plan.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-parent-eval-scope-target cargo test -p ts2wasm-ir eval_fragment_plan_derives_expected_scope_mode -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-parent-eval-scope-target cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`

## Remaining

This is another PR B validation slice. Scope ids and realm ids are still not first-class enough for the final global/caller environment model described in the plan.
