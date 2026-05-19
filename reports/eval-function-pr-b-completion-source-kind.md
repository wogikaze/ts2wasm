# eval/function PR B report: completion source kind

Run ID: `eval-function-pr-b-completion-source-kind`
Date: 2026-05-19

## Slice

Tightened `EvalFragmentPlan` completion ownership so completion plans only attach to static string eval sources.

## Change

- `EvalFragmentPlan::completion_state_is_consistent()` now requires embedded completion/declaration plans to belong to `EvalSource::StaticLiteral`.
- Forged `EvalSource::NonStringStatic` plans with completion steps are rejected instead of letting lowering ignore the embedded completion plan and return the non-string value.
- Updated the embedded completion-state unit test so the valid base case uses sloppy eval declaration landing, while strict lexical isolation remains covered separately.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-completion-source-target cargo test -p ts2wasm-ir eval_fragment_plan_validates_embedded_completion_state -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-completion-source-target cargo test -p ts2wasm-ir eval_completion_plan_validates_scope_landing_state -- --nocapture`

## Remaining

This is a PR B plan-invariant slice. It does not replace the remaining canonical eval-code environment work for full direct eval TDZ, global eval environments, or dynamic descriptor mutation ledgers.
