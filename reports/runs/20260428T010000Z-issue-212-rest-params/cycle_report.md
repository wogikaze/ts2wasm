# Cycle Report: issue 212 rest parameters

## Outcome

DONE. Rest parameter argument collection is implemented without adding the `arguments` object.

## Changes

- Lowered IR now records `rest_param_index` on `LoweredFunction`.
- Rest parameters are no longer overwritten with an empty array in function prologues.
- User-call emission builds a dense array for arguments at and after the rest index, then passes it as the final wasm parameter.
- Constructor emission handles rest constructors with the existing implicit `this` parameter convention.
- Runtime link planning includes `AllocHeap` when rest parameters are present so call-site rest arrays can be allocated.
- Node differential fixtures cover zero, one, and multiple extra rest arguments.
- `docs/language-reference/javascript-features.md`, `current-state.md`, issue 038 cross-links, issue 212 completion evidence, and `issues/index.md` are synchronized.

## Validation

- `cargo fmt --all --check`: PASS
- `cargo nextest run -E 'test(rest)'`: PASS, 1 passed
- `cargo nextest run`: PASS, 240 passed / 4 skipped
- `scripts/manager update-issue-index --check`: PASS
- `scripts/manager check-issue-index`: PASS
- `scripts/manager check-issue-health`: PASS
- `scripts/manager check-repo-smoke`: PASS
- `scripts/manager check-agent-state`: PASS

## Notes

- `cargo clippy --all-targets --all-features -- -D warnings` failed on pre-existing `clippy::assertions-on-constants` diagnostics in `crates/runtime-abi/src/layout.rs`. That path is outside the issue-212 assignment scope and was not edited.
- The parent-provided `reports/agents/agent-212-rest-params-20260428T010000Z/assignment.md` remains untracked and unstaged.
