# Assignment: 052 JSON.stringify nested value preservation

Run ID: `052-json-stringify-nested-20260428T080100Z`

Branch: `agent/052-json-stringify-nested-20260428T080100Z`

Issue: `issues/open/052-implement-json.md`

Slice: narrow `JSON.stringify` nested object literal value preservation. Reproduce with Node and iwasm using a tiny fixture such as `JSON.stringify({a:{b:1}, c:[2]})` or the smallest failing nested object/array case. If current runtime already matches Node, add regression coverage only and record PROGRESS. If it fails, implement only the smallest backend/runtime change for nested object literal/array value preservation.

Allowed files:

- `crates/backend-wasm/src/**` if implementation is required
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/builtins-and-io/json-stringify-nested*.ts`
- `issues/open/052-implement-json.md`
- `reports/runs/052-json-stringify-nested-20260428T080100Z/**`
- `reports/agents/052-json-stringify-nested-20260428T080100Z/assignment.md`

Forbidden files:

- `docs/**`
- `crates/frontend/**`
- `crates/ir/**`
- coverage artifacts
- unrelated issue files

Out of scope:

- non-integer numbers
- UTF-16/non-ASCII/surrogate support
- full replacer semantics
- broad throw-compatible parse diagnostics

Expected validation:

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(json)'`
- `cargo nextest run -p ts2wasm-cli json`
- direct `node` and `cargo run -p ts2wasm-cli -- build ... && iwasm ...` evidence for new fixture(s)
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`
- full `cargo nextest run` if feasible before reporting merge

Reporting:

- Attempt `scripts/manager discord-report --run-id 052-json-stringify-nested-20260428T080100Z`.
- If webhook env is absent or reporting fails, save deferred payload/report under `reports/runs/052-json-stringify-nested-20260428T080100Z/` and continue.
