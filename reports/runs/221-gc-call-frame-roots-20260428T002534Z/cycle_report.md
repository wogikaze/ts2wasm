# Cycle Report: issue 221 GC call-frame roots

Run ID: `221-gc-call-frame-roots-20260428T002534Z`
Branch: `agent/221-gc-call-frame-roots-20260428T095000Z`
Outcome: DONE

## Scope

- Implemented activation-frame GC roots for function execution.
- Added closure/call-frame GC differential coverage.
- Moved issue 221 to done and regenerated `issues/index.md`.

## Implementation

- `_start` now allocates a fixed call-frame root stack as part of the GC root table allocation when heap allocation and functions are present.
- Function entry pushes an activation frame with previous-frame pointer, slot count, and local root slots.
- Function locals and backend temporaries mirror into the active frame.
- Function returns save the result, unregister the activation frame, then return the saved value.
- GC mark now scans the active call-frame chain in addition to the top-level root table.

## Acceptance Evidence

- Function/call-frame heap locals are marked across collection:
  `cargo nextest run -p ts2wasm-backend-wasm function_locals_are_mirrored_into_activation_gc_root_frames` passed.
- Closure/call-frame escape fixtures trigger collection and preserve semantics:
  `fixtures/core-semantics/gc-call-frame-root.ts` and `fixtures/core-semantics/closure-gc-call-frame-root.ts` pass Node differential in `m3_semantic_fixtures_match_node_output_under_iwasm`.
- Node differential tests pass for closure/call-frame GC fixtures:
  `cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm` passed.

## Validation

- `cargo fmt --all --check`: PASS
- `cargo nextest run -p ts2wasm-backend-wasm`: PASS, 15 passed
- `cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm`: PASS
- `cargo nextest run -p ts2wasm-cli --test m2_node_diff arrow_function_fixtures_match_node_output_under_iwasm`: PASS
- `scripts/manager update-issue-index --check`: PASS
- `scripts/manager check-issue-health`: PASS
- `scripts/manager check-repo-smoke`: PASS
- `cargo nextest run`: PASS, 246 passed, 4 skipped
- `scripts/manager check-agent-state`: PASS

## Notes

- The suggested broad filter `cargo nextest run -E 'test(gc|root|closure|arrow)'` selected zero tests in this repository, so exact backend and Node differential tests were used.
- Escaping function values remain outside the current devirtualized local-arrow model and are documented in `current-state.md`; this issue closes the activation frame and local-arrow capture root safety slice.
