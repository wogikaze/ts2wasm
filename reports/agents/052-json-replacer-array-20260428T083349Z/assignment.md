# Assignment: 052 JSON.stringify array replacer slice

- Run ID: `052-json-replacer-array-20260428T083349Z`
- Branch: `agent/052-json-replacer-array-20260428T083349Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-052-json-replacer-array-20260428T083349Z`
- Issue: `issues/open/052-implement-json.md`
- Slice: implement a narrow `JSON.stringify` array replacer property-list subset currently guarded as unsupported.

## Coordination

You are not alone in the codebase. Other child agents are working in separate worktrees on issue 060 coverage and issue 231 module parser diagnostics. Do not revert, overwrite, or depend on their unmerged edits. Stay within this worktree and this branch.

## Scope

- Reproduce `fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts`.
- Implement the smallest safe `JSON.stringify(value, ["key"])` subset for object property filtering with string-literal array entries.
- Add Node/iwasm differential coverage for at least one object property-list case.
- Keep unsupported diagnostics for function replacers and unsupported array replacer contents/forms that are outside this slice.
- Update issue 052 progress evidence with remaining gaps.

## Allowed Files

- `crates/ir/src/lowered.rs`
- `crates/backend-wasm/src/runtime_builtins_host.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/builtins-and-io/json-stringify*.ts`
- `issues/open/052-implement-json.md`
- `reports/runs/052-json-replacer-array-20260428T083349Z/**`
- `reports/agents/052-json-replacer-array-20260428T083349Z/assignment.md`

## Forbidden Files

- `docs/**`
- `crates/frontend/**` unless a diagnostic span-only adjustment is strictly required
- Unrelated fixtures or issue files

## Required Validation

```sh
cargo fmt --all --check
cargo nextest run -E 'test(json)'
cargo nextest run -p ts2wasm-cli json
node fixtures/builtins-and-io/<new-json-stringify-replacer-array-fixture>.ts
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/<new-json-stringify-replacer-array-fixture>.ts -o /tmp/ts2wasm-json-replacer-array.wasm
iwasm /tmp/ts2wasm-json-replacer-array.wasm
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager discord-report --run-id 052-json-replacer-array-20260428T083349Z
```

Run full `cargo nextest run` if the runtime change is broader than the narrow replacer-array subset. If Discord reporting fails because `DISCORD_WEBHOOK_URL` is absent, save the deferred payload/error under the run directory and continue.

## Completion Protocol

- Commit validated progress on this branch.
- Do not merge to parent.
- End with exactly one line:

```text
PARENT_EVENT: PROGRESS issue=052 branch=agent/052-json-replacer-array-20260428T083349Z commit=<hash> validation="<short evidence>" report=reports/runs/052-json-replacer-array-20260428T083349Z/cycle_report.md merge_request=yes
```
