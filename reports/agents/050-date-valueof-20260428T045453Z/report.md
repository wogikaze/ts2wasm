# Child Report: 050-date-valueof-20260428T045453Z

- Child id: `050-date-valueof-20260428T045453Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-050-date-valueof-20260428T045453Z`
- Branch: `agent/050-date-valueof-20260428T045453Z`
- Issue: `050`
- Outcome: PROGRESS
- Commit: `ea01ddfbf18dcc5acc9a9f969f046b36b7d9b0af`

## Summary

Implemented the deterministic Date-only slice for `Date.prototype.valueOf()`.
`valueOf()` now reuses the existing Date epoch representation and `DateGetTime`
runtime helper for deterministic `new Date(<epoch-ms integer>)` receivers.

Added Node/iwasm differential fixture coverage for `0`, positive, and negative
integer epochs:

- `fixtures/builtins-and-io/date-epoch-value-of.ts`
- `crates/cli/tests/m2_node_diff.rs`

Issue 050 remains open because broader Date work is still incomplete:
`Date.now()`, no-argument `new Date()`, `toString`, non-integer/non-literal Date
inputs, an auditable live-time capability policy, and full Date API behavior.
No live host time import was added.

## Validation

```text
command: node fixtures/builtins-and-io/date-epoch-value-of.ts
result: pass
stdout:
0
123456
-123456

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-epoch-value-of.ts -o /tmp/ts2wasm-date-epoch-value-of.wasm && iwasm /tmp/ts2wasm-date-epoch-value-of.wasm
result: pass
stdout:
0
123456
-123456

command: cargo fmt --all --check
result: pass

command: cargo nextest run -p ts2wasm-cli date_epoch_value_of_fixture_matches_node_output_under_iwasm date_epoch_get_time_fixture_matches_node_output_under_iwasm date_live_time_fixtures_report_capability_policy_diagnostic
result: pass, 3 tests run

command: cargo nextest run -p ts2wasm-cli date
result: pass, 5 tests run

command: scripts/manager fmt
result: pass

command: scripts/manager check-issue-health
result: pass

command: scripts/manager check-agent-state
result: pass
```
