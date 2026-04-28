# Agent Outcome: issue 221

Status: DONE
Run ID: `221-gc-call-frame-roots-20260428T002534Z`
Branch: `agent/221-gc-call-frame-roots-20260428T095000Z`
Commit: `f7ad5b0`

## Summary

Issue 221 is complete. The backend now registers function activation frames in a fixed GC root-frame stack, mirrors function locals/temporaries into active frames, marks the active frame chain during collection, and unregisters frames on emitted returns.

## Evidence

- `cargo fmt --all --check`: PASS
- `cargo nextest run -p ts2wasm-backend-wasm`: PASS, 15 passed
- `cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm`: PASS
- `cargo nextest run -p ts2wasm-cli --test m2_node_diff arrow_function_fixtures_match_node_output_under_iwasm`: PASS
- `scripts/manager update-issue-index --check`: PASS
- `scripts/manager check-issue-health`: PASS
- `scripts/manager check-repo-smoke`: PASS
- `cargo nextest run`: PASS, 246 passed, 4 skipped
- `scripts/manager check-agent-state`: PASS

## Fixture Coverage

- Existing: `fixtures/core-semantics/gc-call-frame-root.ts`
- Added: `fixtures/core-semantics/closure-gc-call-frame-root.ts`
