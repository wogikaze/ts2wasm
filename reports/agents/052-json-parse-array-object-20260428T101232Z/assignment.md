# Child Assignment: 052 JSON parse array object continuation

Child run id: `052-json-parse-array-object-20260428T101232Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-052-json-parse-array-object-20260428T101232Z`
Branch: `agent/052-json-parse-array-object-20260428T101232Z`
Parent branch at assignment: `master` @ `1760164`

You are not alone in this repository. Other agents are active in separate worktrees; do not revert or overwrite changes made by others, do not touch parent `master`, and stay within this assignment.

## Assigned Issue List

1. `issues/open/052-implement-json.md`

## Objective

Complete one narrow issue-052 JSON.parse continuation around object elements inside arrays. The issue history still lists explicit object-elements-inside-arrays coverage as a remaining gap in earlier slices; close that gap with direct Node/iwasm evidence, implementing only if the current runtime does not already handle it.

## Expected Slice

- Add a fixture such as `fixtures/builtins-and-io/json-parse-array-object.ts` covering `JSON.parse('[{"a":1},{"b":[2]}]')` or a similarly small object-inside-array case.
- Add the fixture to the existing JSON Node differential test list.
- If pre-change runtime already matches Node, commit the regression coverage and issue evidence as PROGRESS.
- If it fails, implement the smallest runtime fix in `crates/backend-wasm/src/runtime_builtins_host.rs`, then add coverage.

Do not claim issue 052 done. Do not touch module-system, logical-assignment, or coverage-ramp files.

## Allowed Files

- `crates/backend-wasm/src/runtime_builtins_host.rs` only if required by the fixture
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/builtins-and-io/`
- `issues/open/052-implement-json.md`
- `reports/agents/052-json-parse-array-object-20260428T101232Z/`
- `reports/runs/052-json-parse-array-object-20260428T101232Z/`

## Forbidden Files

- `docs/`
- `current-state.md` unless implementation facts changed materially
- Module-system files or issue 233 files
- Logical-assignment files or issue 228 files
- Coverage artifacts or issue 060 files
- Parent branch or any other agent worktree

## Required Validation

```sh
cargo fmt --all --check
cargo nextest run -E 'test(json)'
cargo nextest run -p ts2wasm-cli json
node fixtures/builtins-and-io/json-parse-array-object.ts
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-array-object.ts -o /tmp/ts2wasm-052-json-array-object.wasm
iwasm /tmp/ts2wasm-052-json-array-object.wasm
scripts/manager check-issue-health
scripts/manager check-agent-state
```

Run full `cargo nextest run` if backend runtime code changes are broad or if attempting to close issue 052.

## Reporting

- Write `reports/runs/052-json-parse-array-object-20260428T101232Z/cycle_report.md`.
- Write/validate `test_report.json` when practical.
- Attempt `scripts/manager discord-report --run-id 052-json-parse-array-object-20260428T101232Z`; if unavailable, commit deferred payload/error artifacts.
- Commit all validated useful work.
- Merge latest parent `master` before final event.
- End with exactly one parent event line:

```text
PARENT_EVENT: PROGRESS issue=052 branch=agent/052-json-parse-array-object-20260428T101232Z commit=<hash> validation="<summary>" report=reports/runs/052-json-parse-array-object-20260428T101232Z/cycle_report.md merge_request=yes
```
