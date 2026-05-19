# eval/function PR B report: EvalFragmentPlan completion-state validation

Run ID: `eval-function-pr-b-eval-completion-plan-validation`
Date: 2026-05-19

## Slice

Tightened the plan-owned contract between `EvalFragmentPlan` and its embedded declaration/completion plan.

## Change

- Added `EvalFragmentPlan::completion_state_is_consistent()`.
- The consistency check rejects missing declaration/completion counterparts, strictness drift, scope drift, and runtime-host fragments that carry AOT completion state.
- Static eval AOT expansion and runtime eval lowering now reject inconsistent embedded completion state before consuming an eval fragment plan.
- Added IR and compiler regression tests for intentionally malformed plans.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-completion-plan-target cargo test -p ts2wasm-ir eval_fragment_plan_validates_embedded_completion_state -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-completion-plan-target cargo test -p ts2wasm-compiler compiler_rejects_eval_fragment_with_inconsistent_completion_plan -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-completion-plan-target cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`
- PASS: `python scripts/issue-lint.py`
- NOTE: `git diff --check` is currently blocked by unrelated pre-existing trailing whitespace in `issues/I-20260518-55S922.md`, `issues/I-20260518-6ZVXAX.md`, and `issues/I-20260518-AZE6KY.md`.

## Remaining

This closes another PR B validation gap. It does not by itself add first-class scope ids, full eval declaration environments, or the PR C/PR D canonical caller/global environment model.
