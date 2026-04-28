# Cycle Report: Issue 049 Map/Set Delete Completion

Run: `20260428T013510Z-049-map-set-delete`
Branch: `agent/049-map-set-delete-20260428T012931Z`
Implementation commit: `1b26026`

## Outcome

DONE. Issue 049 was moved to `issues/done/049-implement-map-set.md`.

## Changes

- Added narrow parser support for `delete` as a member property name after `.`, preserving keyword behavior elsewhere.
- Expanded `fixtures/builtins-and-io/map-set.ts` to cover `Map.prototype.delete` and `Set.prototype.delete` for existing and missing keys.
- Added a frontend parser regression test for `map.delete(...)`.
- Regenerated `issues/index.md`.

## Evidence

- `cargo fmt --all --check`: pass
- `cargo nextest run -p ts2wasm-frontend parses_delete_keyword_after_dot_as_member_property_name`: pass
- `cargo nextest run -p ts2wasm-cli map_set_collection_fixture_matches_node_output_under_iwasm`: pass
- `node fixtures/builtins-and-io/map-set.ts`: pass
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/map-set.ts -o /tmp/issue049-map-set-delete.wasm && iwasm /tmp/issue049-map-set-delete.wasm`: pass
- `cargo nextest run -E 'test(map) or test(set)'`: pass, 4 tests
- `cargo nextest run -p ts2wasm-cli map`: pass, 1 test
- `cargo nextest run -p ts2wasm-cli set`: pass, 2 tests
- `cargo nextest run`: pass, 268 tests passed and 4 skipped
- `scripts/manager update-issue-index --check`: pass
- `scripts/manager check-issue-index`: pass
- `scripts/manager check-issue-health`: pass
- `scripts/manager check-agent-state`: pass
- `scripts/manager check-repo-smoke`: pass

## Remaining Risk

Map/Set key identity remains limited by the current runtime string-key normalization from the previous progress note. This slice completed the assigned basic-operation delete coverage and did not broaden key semantics.
