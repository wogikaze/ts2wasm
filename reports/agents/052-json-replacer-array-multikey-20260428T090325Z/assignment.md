# Assignment: 052 JSON.stringify multi-key array replacer slice

- Run ID: `052-json-replacer-array-multikey-20260428T090325Z`
- Branch: `agent/052-json-replacer-array-multikey-20260428T090325Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-052-json-replacer-array-multikey-20260428T090325Z`
- Issue: `issues/open/052-implement-json.md`
- Slice: extend the narrow `JSON.stringify` array replacer support from one string key to multiple string-literal keys for object literals.

## Coordination

You are not alone in the codebase. Other child agents are working in separate worktrees, including issue 060 coverage and issue 232 module graph. Do not revert, overwrite, or depend on their unmerged edits. Stay within this worktree and this branch.

## Scope

- Reproduce current behavior for `JSON.stringify({ a: 1, b: 2, c: 3 }, ["c", "a"])`.
- Implement the smallest safe support for object-literal property filtering with multiple string-literal array entries, preserving the replacer array order where the current runtime/stringifier architecture can safely do so.
- If preserving replacer order would require broader runtime object-order changes, support only source-object order and record the gap explicitly; do not overreach.
- Keep unsupported diagnostics for function replacers and unsupported array replacer contents/forms.
- Add Node/iwasm differential coverage and issue 052 progress evidence.

## Allowed Files

- `crates/ir/src/lowered.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/builtins-and-io/json-stringify*.ts`
- `issues/open/052-implement-json.md`
- `reports/runs/052-json-replacer-array-multikey-20260428T090325Z/**`
- `reports/agents/052-json-replacer-array-multikey-20260428T090325Z/assignment.md`

## Forbidden Files

- `docs/**`
- `crates/frontend/**`
- `crates/backend-wasm/**` unless the current IR-only approach cannot express the slice safely
- Unrelated fixtures or issue files

## Required Validation

```sh
cargo fmt --all --check
cargo nextest run -E 'test(json)'
cargo nextest run -p ts2wasm-cli json
node fixtures/builtins-and-io/<new-json-stringify-replacer-array-multikey-fixture>.ts
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/<new-json-stringify-replacer-array-multikey-fixture>.ts -o /tmp/ts2wasm-json-replacer-array-multikey.wasm
iwasm /tmp/ts2wasm-json-replacer-array-multikey.wasm
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager discord-report --run-id 052-json-replacer-array-multikey-20260428T090325Z
```

Run full `cargo nextest run` if the change goes beyond `crates/ir/src/lowered.rs` plus fixture/test coverage. If Discord reporting fails because `DISCORD_WEBHOOK_URL` is absent, save the deferred payload/error under the run directory and continue.

## Completion Protocol

- Commit validated progress on this branch.
- Do not merge to parent.
- End with exactly one line:

```text
PARENT_EVENT: PROGRESS issue=052 branch=agent/052-json-replacer-array-multikey-20260428T090325Z commit=<hash> validation="<short evidence>" report=reports/runs/052-json-replacer-array-multikey-20260428T090325Z/cycle_report.md merge_request=yes
```
