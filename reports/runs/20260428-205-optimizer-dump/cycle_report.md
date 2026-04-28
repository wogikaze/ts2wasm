# Cycle report: issue 205

Run ID: `20260428-205-optimizer-dump`
Branch: `agent/205-optimizer-dump-20260428T000000Z`
Outcome: DONE

## Scope

- Added `ts2wasm dump --optimize` with `-O0`, `-O1`, `-O2`, and `-O3`.
- Added an optimized HIR diagnostic phase backed by the same `optimize_hir` pipeline validated by build for supported HIR slices.
- Kept optimizer behavior conservative: `-O1` and above currently fold only literal numeric `JsAdd` when `checked_add` proves the result is representable.
- Added structural and unparse tests proving optimized output is not the unoptimized `LoweredProgram` path.
- Updated final-state docs for optimizer diagnostics and optimized HIR contracts.

## Acceptance evidence

- `ts2wasm dump --optimize -O0 <input.ts>`: covered by `dump_optimize_o0_emits_optimized_hir_without_folding`.
- `ts2wasm dump --optimize -O2 <input.ts>`: covered by `dump_optimize_o2_uses_real_optimizer_passes`.
- `ts2wasm dump --optimize --unparse -O2 <input.ts>`: covered by `dump_optimize_unparse_emits_optimized_pseudo_source`.
- Tests assert `OptimizedHirProgram`, `LiteralNumericAddFold`, and absence of `LoweredProgram` / `top_level_statements` for optimized structural output.

## Validation

- `cargo fmt --all --check`: PASS
- `cargo nextest run -p ts2wasm-cli --test dump_cli`: PASS (9 passed)
- `scripts/manager check-agent-state`: PASS
- `cargo nextest run`: PASS (234 passed, 4 skipped)
- `scripts/manager update-issue-index`: PASS
- `scripts/manager check-issue-index`: PASS
- `scripts/manager check-issue-health`: PASS
- `scripts/manager check-repo-smoke`: PASS
- `scripts/manager update-issue-index --check`: PASS

## Risks

- The backend still emits from `LoweredProgram`; optimized HIR is a truthful diagnostic/compiler validation surface and does not claim wasm codegen optimization.
- No semantic-changing optimizer pass was added.
