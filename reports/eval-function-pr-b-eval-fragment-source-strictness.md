# eval/function PR B report: EvalFragmentPlan source strictness

Run ID: `eval-function-pr-b-eval-fragment-source-strictness`
Date: 2026-05-19

## Slice

Moved one more static eval expansion fact into `EvalFragmentPlan`.

## Change

- Added `eval_source_is_strict: Option<bool>` to `EvalFragmentPlan`.
- `EvalFragmentPlan::new` leaves the field unset for unparsed/runtime fragments.
- `EvalFragmentPlan::with_completion_plan` records the parsed eval-source strictness at the same time it attaches declaration and completion plans.
- Added an IR unit test for the new plan-owned strictness fact.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-parent-eval-plan-target cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-parent-eval-plan-target cargo test -p ts2wasm-ir eval_fragment_plan_records_eval_source_strictness -- --nocapture`

## Remaining

The larger PR B migration remains open: scope ids, host policy details, and declaration/completion lowering still need to consume the canonical plan directly instead of keeping expansion-stage context in parallel.
