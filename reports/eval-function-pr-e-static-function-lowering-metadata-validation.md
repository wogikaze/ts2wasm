# eval/function PR E report: FunctionConstructorPlan lowering metadata validation

Run ID: `eval-function-pr-e-static-function-lowering-metadata-validation`
Date: 2026-05-19

## Slice

Extended static Function-constructor metadata validation to the lowering boundary.

## Change

- Runtime lowering now checks `FunctionConstructorPlan::static_source_is_consistent()` before deciding whether a static Function constructor missed AOT expansion.
- Malformed static AOT plans with generated-function metadata drift now get the same plan-owned diagnostic in lowering as in static expansion.
- Added an IR regression for a malformed static Function constructor plan that bypasses expansion and reaches lowering.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-function-lowering-metadata-target cargo test -p ts2wasm-ir lowering_rejects_function_constructor_static_metadata_drift -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-function-lowering-metadata-clean-target cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-function-lowering-metadata-clean-target cargo fmt --all --check`
- PASS: `python scripts/issue-lint.py`
- PASS: scoped `git diff --check`

## Remaining

This is a PR E boundary-hardening slice. The larger first-class generated function object migration remains open.
