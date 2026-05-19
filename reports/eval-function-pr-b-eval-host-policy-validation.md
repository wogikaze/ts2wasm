# eval/function PR B report: EvalFragmentPlan host-policy validation

Run ID: `eval-function-pr-b-eval-host-policy-validation`
Date: 2026-05-19

## Slice

Moved eval host-lane consistency checking into `EvalFragmentPlan`.

## Change

- Added `EvalFragmentPlan::expected_host_policy()` and `host_policy_is_consistent()`.
- Lowering now rejects eval fragments whose stored `host_policy` does not match the plan's `kind` + `source` before selecting `host.eval.direct` / `host.eval.indirect`.
- Added an IR unit test covering static AOT, runtime direct-host, runtime indirect-host, and an intentionally inconsistent plan.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-parent-eval-policy-target cargo test -p ts2wasm-ir eval_fragment_plan_derives_expected_host_policy -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-parent-eval-policy-target cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`

## Remaining

This is a plan-validation slice. Full PR B still needs scope ids, declaration/completion plan consumption in lowering, and direct/indirect/global environment validation to be owned entirely by the eval fragment plan.
