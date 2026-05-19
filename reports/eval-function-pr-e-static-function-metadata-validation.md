# eval/function PR E report: FunctionConstructorPlan static metadata validation

Run ID: `eval-function-pr-e-static-function-metadata-validation`
Date: 2026-05-19

## Slice

Tightened first-class `FunctionConstructorPlan` validation before static AOT expansion.

## Change

- Added `FunctionConstructorPlan::static_source_is_consistent()`.
- Added `StaticFunctionConstructorSource::is_consistent()` and `FunctionConstructorGeneratedFunction::is_anonymous_constructor_base()`.
- Static Function constructor AOT expansion now rejects plan metadata drift before building the synthetic `FunctionExpr`.
- Added IR and compiler regressions for malformed generated function metadata.

## Evidence

- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-function-metadata-plan-target cargo test -p ts2wasm-ir function_constructor_plan_validates_static_source_metadata -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-function-metadata-plan-clean-target cargo test -p ts2wasm-compiler compiler_rejects_function_constructor_with_inconsistent_static_metadata -- --nocapture`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-function-metadata-plan-clean-target cargo check -p ts2wasm-ir -p ts2wasm-compiler -p ts2wasm-cli`
- PASS: `CARGO_TARGET_DIR=/tmp/ts2wasm-function-metadata-plan-clean-target cargo fmt --all --check`
- PASS: scoped `git diff --check`
- NOTE: compiler/check/fmt were run in clean worktree `/tmp/ts2wasm-function-metadata-plan` because the parent worktree has unrelated dirty backend changes that currently fail compilation.

## Remaining

This is a PR E plan-contract slice. The broader migration away from synthetic `FunctionExpr` fallback and into a generated function object directly owned by `FunctionConstructorPlan` remains open.
