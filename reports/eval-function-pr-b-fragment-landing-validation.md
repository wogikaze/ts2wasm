# eval/function PR B report: EvalFragmentPlan landing validation

Run ID: `eval-function-pr-b-fragment-landing-validation`
Date: 2026-05-19

## Slice

Connected embedded completion landing validation back into `EvalFragmentPlan`.

## Change

- `EvalFragmentPlan::completion_state_is_consistent()` now requires the embedded `EvalCompletionPlan` landing state to match its caller/global scope mode.
- Added an IR regression that intentionally embeds a global landing step in a caller-scope fragment and verifies the fragment-level consistency check rejects it.
- This makes static expansion and runtime lowering inherit the same caller/global landing guard through their existing fragment validation.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-fragment-landing-target cargo test -p ts2wasm-ir eval_fragment_plan_validates_embedded_completion_state -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-eval-fragment-landing-target cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`

## Remaining

This is a PR B contract-tightening slice. It does not replace the larger PR C/PR D environment-model work for full eval lexical/global semantics.
