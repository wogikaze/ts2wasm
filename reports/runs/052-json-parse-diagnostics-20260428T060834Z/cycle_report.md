# Issue 052 JSON Parse Diagnostics Cycle

- Agent: `052-json-parse-diagnostics-20260428T060834Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-052-json-parse-diagnostics-20260428T060834Z`
- Branch: `agent/052-json-parse-diagnostics-20260428T060834Z`
- Status: PROGRESS

## Slice

Implemented exact keyword validation for JSON `true`, `false`, and `null` literals in the runtime parser. The previous top-level path accepted invalid same-length literals such as `turd` as `true`; nested object/array literal parsing now uses the same exact checks.

## Evidence

Pre-change reproduction:

```text
node /tmp/ts2wasm-json-invalid-literal.ts
result: rejected with JSON SyntaxError, status 1

cargo run -q -p ts2wasm-cli -- build /tmp/ts2wasm-json-invalid-literal.ts -o /tmp/ts2wasm-json-invalid-literal.wasm && iwasm /tmp/ts2wasm-json-invalid-literal.wasm
result: accepted, printed "accepted", status 0
```

Post-change direct fixture:

```text
node fixtures/builtins-and-io/json-parse-invalid-literal.ts
result: rejected with JSON SyntaxError, status 1

cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-invalid-literal.ts -o /tmp/ts2wasm-json-parse-invalid-literal.wasm && iwasm /tmp/ts2wasm-json-parse-invalid-literal.wasm
result: rejected with Exception: unreachable, status 1
```

Validation:

```text
cargo fmt --all --check
result: pass

cargo nextest run -E 'test(json)'
result: pass, 14 passed

cargo nextest run -p ts2wasm-cli json
result: pass, 11 passed

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass

cargo nextest run
result: pass, 341 passed, 4 skipped
```

## Remaining

Issue 052 remains open. Remaining gaps include arbitrary non-integer JSON number representation, non-ASCII `\uXXXX`/surrogate handling, full replacer semantics, and broader throw-compatible parse diagnostics.
