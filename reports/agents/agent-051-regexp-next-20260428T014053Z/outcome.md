# Agent Outcome

Agent: `agent-051-regexp-next-20260428T014053Z`
Issue: `051`
Branch: `agent/051-regexp-next-20260428T014053Z`
Status: `PROGRESS`

Implemented the assigned continuation slice for constructor-backed plain RegExp `.test`:

- `new RegExp("abc")` lowers to `/abc/` in the existing constrained representation.
- `let r = new RegExp("abc"); r.test("...")` dispatches to `RegExpTest`.
- Unsupported constructor patterns keep an `issue-051` diagnostic.
- `fixtures/core-semantics/regexp-test.ts` now covers constructor-backed true/false cases with Node differential evidence.

Validation:

```text
cargo nextest run -E 'test(regexp)' -> pass; 11 tests
cargo nextest run -p ts2wasm-cli regexp -> pass; 8 tests
node fixtures/core-semantics/regexp-test.ts -> true / false / true / true / false
cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-regexp-test.wasm && iwasm /tmp/ts2wasm-issue051-regexp-test.wasm -> true / false / true / true / false
cargo fmt --all --check -> pass
scripts/manager check-agent-state -> pass
cargo nextest run -> pass; 270 tests, 4 skipped
scripts/manager check-issue-health -> pass
```

Remaining issue 051 work:

- `RegExp.prototype.exec`
- `String.prototype.match`
- broader constructor flags/state
- full RegExp syntax
