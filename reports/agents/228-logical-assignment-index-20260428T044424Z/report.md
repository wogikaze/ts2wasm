# Child Report: issue 228 logical assignment index continuation

- Status: PROGRESS
- Branch: `agent/228-logical-assignment-index-20260428T044424Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-228-logical-assignment-index-20260428T044424Z`
- Issue: `228`

## Scope completed

- Added parser support for computed string-literal logical assignment on identifier receivers, for example `target["value"] ||= rhs()`.
- Routed that form to the existing static property logical-assignment path to preserve short-circuit RHS behavior without introducing unsafe dynamic receiver/key temporaries.
- Added `fixtures/core-semantics/logical-assignment-index.ts` for Node/iwasm differential coverage across `||=`, `??=`, and `&&=`.
- Updated the unsupported-target fixture to keep dynamic computed keys (`target[key] &&= 1`) issue-linked and unsupported.

## Validation

```text
cargo fmt --all --check
result: pass

cargo nextest run -E 'test(logical_assignment)'
result: pass; 5 tests passed

node fixtures/core-semantics/logical-assignment-index.ts
result: pass; stdout:
kept
kept
rhs
filled
filled
rhs
fallback
fallback
rhs
again
again

cargo run -p ts2wasm-cli -- build fixtures/core-semantics/logical-assignment-index.ts -o /tmp/ts2wasm-228-logical-assignment-index.wasm && iwasm /tmp/ts2wasm-228-logical-assignment-index.wasm
result: pass; stdout matched Node output

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass
```

## Remaining work

- Dynamic computed logical assignment targets still need a safe temporary-key design.
- Non-identifier receivers remain unsupported.
- Annex B `[[IsHTMLDDA]]` compatibility remains open.
