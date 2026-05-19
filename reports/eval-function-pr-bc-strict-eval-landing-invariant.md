# eval/function PR B/C report: strict eval landing invariant

Run ID: `eval-function-pr-bc-strict-eval-landing-invariant`
Date: 2026-05-19

## Slice

Tightened the plan-owned eval completion contract for strict eval-code lexical isolation.

## Change

- `EvalCompletionPlan::landing_state_is_consistent()` now rejects strict eval plans that carry caller declaration metadata.
- The same invariant rejects strict eval completion steps that would land `var` / function declarations in the caller scope.
- Added an IR unit regression that constructs a malformed strict caller/eval plan and verifies the plan validator rejects it.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-strict-landing-target cargo test -p ts2wasm-ir eval_completion_plan_validates_scope_landing_state -- --nocapture`

## Remaining

This is a plan-invariant slice. It does not replace the broader PR C/G work for full eval lexical environments, TDZ modeling, or mutation-ledger-backed dynamic direct eval.
