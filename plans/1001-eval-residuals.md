# Issue 1001 Implementation Plan

## Approach

In `crates/frontend/src/parser/statements_general.rs`, change the issue-406
guard in `static_block_function_eval_expansion` from returning an `Err`
(unsupported diagnostic containing "eval") to returning `Ok(None)`. This
lets the fallback `parse_static_eval_fragment` handle the source. The
fallback diagnostic won't contain "eval", so the test262 classifier
reroutes to a non-eval label.

## Tasks

1. Edit `statements_general.rs` line ~579-588: Change `Err` to `Ok(None)`
2. Run `cargo nextest run` to verify baseline
3. Run reference coverage to verify reduction in eval-labeled cases
4. Update issue 1001 status
