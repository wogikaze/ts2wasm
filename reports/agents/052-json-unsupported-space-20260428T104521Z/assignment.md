# Child Assignment: 052 JSON unsupported space parity

Child run id: `052-json-unsupported-space-20260428T104521Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-052-json-unsupported-space-20260428T104521Z`
Branch: `agent/052-json-unsupported-space-20260428T104521Z`
Parent branch at assignment: `master` @ `ff514ed`

You are not alone in this repository. Other agents are active in separate worktrees; do not revert or overwrite changes made by others, do not touch parent `master`, and stay within this assignment.

## Assigned Issue List

1. `issues/open/052-implement-json.md`

## Objective

Make one narrow JSON continuation around the remaining "non-stringify `space` ignored-value parity requiring IR validation work" gap recorded in issue 052.

Preferred slice:

- Reproduce a small `JSON.stringify(value, null, unsupportedSpace)` case where Node ignores or coerces a non-number/non-string `space` form but ts2wasm currently rejects too broadly.
- If safe, implement the narrow accepted parity case with Node/iwasm evidence.
- If not safe, add a precise issue-052 diagnostic fixture and update issue evidence so the gap is explicit and regression-guarded.

Do not close issue 052. Do not touch module, logical-assignment, coverage, or Annex B files.

## Allowed Files

- `crates/ir/src/`
- `crates/backend-wasm/src/runtime_builtins_host.rs` only if needed
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/builtins-and-io/`
- `issues/open/052-implement-json.md`
- `reports/agents/052-json-unsupported-space-20260428T104521Z/`
- `reports/runs/052-json-unsupported-space-20260428T104521Z/`

## Forbidden Files

- `docs/`
- `current-state.md` unless behavior facts changed materially
- Module-system files or issue 233 files
- Logical-assignment files or issue 236/237 files
- Coverage artifacts or issue 060 files
- Parent branch or any other agent worktree

## Required Validation

```sh
cargo fmt --all --check
cargo nextest run -E 'test(json)'
cargo nextest run -p ts2wasm-cli json
node <new-json-fixture>
cargo run -q -p ts2wasm-cli -- build <new-json-fixture> -o /tmp/ts2wasm-052-json-space.wasm
iwasm /tmp/ts2wasm-052-json-space.wasm
scripts/manager check-issue-health
scripts/manager check-agent-state
```

Run full `cargo nextest run` only if backend runtime behavior changes broadly or if attempting to close issue 052.

## Reporting

- Write `reports/runs/052-json-unsupported-space-20260428T104521Z/cycle_report.md`.
- Write/validate `test_report.json` when practical.
- Attempt `scripts/manager discord-report --run-id 052-json-unsupported-space-20260428T104521Z`; if unavailable, commit deferred payload/error artifacts.
- Commit all validated useful work.
- Merge latest parent `master` before final event.
- End with exactly one parent event line:

```text
PARENT_EVENT: PROGRESS issue=052 branch=agent/052-json-unsupported-space-20260428T104521Z commit=<hash> validation="<summary>" report=reports/runs/052-json-unsupported-space-20260428T104521Z/cycle_report.md merge_request=yes
```
