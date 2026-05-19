# eval/function PR B report: EvalCompletionPlan landing validation

Run ID: `eval-function-pr-b-eval-completion-landing-validation`
Date: 2026-05-19

## Slice

Moved eval completion caller/global landing consistency into the plan model.

## Change

- Added recursive `EvalCompletionStep::has_caller_landing()` and `has_global_landing()` helpers.
- Added `EvalCompletionPlan::landing_state_is_consistent()` to reject caller/global landing-step drift from the plan itself.
- Runtime lowering now consumes that plan API instead of carrying a local caller-landing scanner.
- Added an IR unit test covering valid caller/global plans and malformed mixed landing plans.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-landing-plan-target cargo test -p ts2wasm-ir eval_completion_plan_validates_scope_landing_state -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-landing-plan-target cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`

## Remaining

This is a PR B validation slice. It does not replace the remaining PR C/PR D work to model full eval lexical environments and canonical global environment semantics.
