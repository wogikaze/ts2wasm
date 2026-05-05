---
id: 275
title: "Implement Set size and clear"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
status: done
completed: 2026-04-29
---

## Summary

Implement the remaining basic Set state APIs that were outside issue 049's constructor/add/has/delete slice.

Problem: `new Set()` currently has validated constructor/add/has/delete coverage, but `Set.prototype.size` and `Set.prototype.clear` are not covered by the closed basic Set fixture.

## Current failure

No current fixture covers this behavior. The expected narrow reproduction is:

```typescript
let s = new Set();
console.log(s.size);
s.add("a");
s.add("b");
s.add("a");
console.log(s.size);
s.clear();
console.log(s.size);
console.log(s.has("a"));
```

Expected Node stdout:

```text
0
2
0
false
```

## Desired final state

`Set.prototype.size` reflects the number of unique elements in the current supported Set representation, and `Set.prototype.clear` removes all entries.

## Scope

In scope:

- [x] Add lowering/runtime support for `Set.prototype.size`.
- [x] Add lowering/runtime support for `Set.prototype.clear`.
- [x] Add Node/iwasm differential fixture coverage for size, duplicate add, clear, and post-clear `has`.

Out of scope:

- Iterable constructor arguments.
- SameValueZero parity beyond the current collection key representation.
- Set iteration protocol.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/m2_node_diff.rs`

Do not touch:

- `web-ui/`

## Acceptance criteria

- [x] A fixture proves `new Set().size` starts at `0`.
- [x] The fixture proves duplicate `add` calls do not increase `size`.
- [x] The fixture proves `clear()` empties the Set and subsequent `has` returns `false`.
- [x] Node and iwasm stdout match for the fixture.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli set
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli map_set_collection_fixture_matches_node_output_under_iwasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from stale broad issue 272 after issue 049 closed the constructor/add/has/delete subset.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `00a603f` (`Implement Set size and clear`)

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli set_size_clear_fixture_matches_node_output_under_iwasm map_set_collection_fixture_matches_node_output_under_iwasm
result: pass; 2 tests run, 2 passed, 332 skipped
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli set
result: pass; 4 tests run, 4 passed, 330 skipped
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli map_set_collection_fixture_matches_node_output_under_iwasm
result: pass; 1 test run, 1 passed, 333 skipped
date: 2026-04-29

command: mise run update-issue-index
result: pass; issues/index.md regenerated
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass; issues/index.md OK (up to date)
date: 2026-04-29

command: mise run check issues
result: pass; issues/index.md queue OK; check_issue_health: OK
date: 2026-04-29

command: cargo nextest run
result: pass; 521 tests run, 521 passed, 4 skipped
date: 2026-04-29
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/275-implement-set-size-clear.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
