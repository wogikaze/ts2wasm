# Assignment: 052-json-array-object-20260428T074900Z

- Run ID: `052-json-array-object-20260428T074900Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-052-json-array-object-20260428T074900Z`
- Branch: `agent/052-json-array-object-20260428T074900Z`
- Issue: `issues/open/052-implement-json.md`
- Slice: narrow `JSON.parse` coverage/implementation for object elements inside parsed arrays, including `JSON.parse('[{"n":1},{"n":2}]')` and reading object properties from those array elements.

## Scope

First reproduce against Node and iwasm. If current runtime already handles it, add regression coverage only and record PROGRESS. If it fails, implement the smallest backend/runtime change needed for this form only.

## Allowed files

- `crates/backend-wasm/src/**` only if implementation is required
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/builtins-and-io/json-parse-array-object*.ts`
- `issues/open/052-implement-json.md`
- `reports/runs/052-json-array-object-20260428T074900Z/**`
- `reports/agents/052-json-array-object-20260428T074900Z/assignment.md`

## Forbidden files

- `docs/**`
- `crates/frontend/**`
- `crates/ir/**`
- coverage artifacts
- unrelated issue files

## Exclusions

- arbitrary non-integer numbers
- unicode/surrogate support
- full replacer semantics
- broad throw-compatible diagnostics

## Expected validation

- `cargo fmt --all --check`
- `cargo nextest run -E 'test(json)'`
- `cargo nextest run -p ts2wasm-cli json`
- direct `node` and `cargo run -p ts2wasm-cli -- build ... && iwasm ...` evidence for new fixture(s)
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`
- full `cargo nextest run` if feasible before reporting merge
- `scripts/manager discord-report --run-id 052-json-array-object-20260428T074900Z` with deferred payload/report on webhook failure
