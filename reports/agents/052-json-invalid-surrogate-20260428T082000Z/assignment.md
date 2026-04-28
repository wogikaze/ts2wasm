# Assignment: 052-json-invalid-surrogate-20260428T082000Z

- Run ID: `052-json-invalid-surrogate-20260428T082000Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-052-json-invalid-surrogate-20260428T082000Z`
- Branch: `agent/052-json-invalid-surrogate-20260428T082000Z`
- Issue: `issues/open/052-implement-json.md`
- Slice: narrow `JSON.parse` surrogate diagnostics/coverage.

## Scope

- Reproduce Node and iwasm behavior for invalid or unsupported surrogate forms:
  - lone low surrogate: `JSON.parse('"\\udc00"')`
  - surrogate pair: `JSON.parse('"\\ud83d\\ude00"')`
- If current runtime already rejects or handles safely, add regression coverage only and record PROGRESS.
- If it accepts incorrectly or miscompiles, implement the smallest backend/runtime diagnostic fix.

## Boundaries

Allowed files:

- `crates/backend-wasm/src/**` only if implementation is required
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/builtins-and-io/json-parse-*surrogate*.ts`
- `issues/open/052-implement-json.md`
- `reports/runs/052-json-invalid-surrogate-20260428T082000Z/**`
- `reports/agents/052-json-invalid-surrogate-20260428T082000Z/assignment.md`

Forbidden files:

- `docs/**`
- `crates/frontend/**`
- `crates/ir/**`
- coverage artifacts
- unrelated issue files

## Expected Validation

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(json)'`
- `cargo nextest run -p ts2wasm-cli json`
- direct Node and `cargo run -p ts2wasm-cli -- build ... && iwasm ...` evidence for new fixture(s)
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`
- full `cargo nextest run` if feasible before reporting merge

## Reporting

- Attempt `scripts/manager discord-report --run-id 052-json-invalid-surrogate-20260428T082000Z`
- If webhook env is absent or reporting fails, save a deferred payload/report under `reports/runs/052-json-invalid-surrogate-20260428T082000Z/`
- Commit validated progress on the assigned branch.
- Do not merge to parent.
