# Agent Report: issue 049 Map/Set delete completion

Child id: `agent-049-map-set-delete-20260428T012931Z`
Branch: `agent/049-map-set-delete-20260428T012931Z`
Status: DONE

## Summary

Completed issue 049 by closing the remaining `.delete()` gap for Map and Set.

The parser now accepts `delete` as a narrow member property name after `.`, which allows `map.delete(...)` and `set.delete(...)` to lower to the existing `MapDelete` and `SetDelete` runtime helpers. The Map/Set fixture now includes delete behavior for present and missing keys and passes Node/iwasm differential validation.

## Commits

- `1b26026` - `Fix Map Set delete member parsing`

## Validation

- `cargo fmt --all --check`: pass
- `cargo nextest run -p ts2wasm-frontend parses_delete_keyword_after_dot_as_member_property_name`: pass
- `cargo nextest run -p ts2wasm-cli map_set_collection_fixture_matches_node_output_under_iwasm`: pass
- `node fixtures/builtins-and-io/map-set.ts`: pass
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/map-set.ts -o /tmp/issue049-map-set-delete.wasm && iwasm /tmp/issue049-map-set-delete.wasm`: pass
- `cargo nextest run -E 'test(map) or test(set)'`: pass
- `cargo nextest run -p ts2wasm-cli map`: pass
- `cargo nextest run -p ts2wasm-cli set`: pass
- `cargo nextest run`: pass, 268 tests passed and 4 skipped
- `scripts/manager update-issue-index --check`: pass
- `scripts/manager check-issue-index`: pass
- `scripts/manager check-issue-health`: pass
- `scripts/manager check-agent-state`: pass
- `scripts/manager check-repo-smoke`: pass

## Webhook

Webhook was not configured in the environment, so the payload was deferred to `reports/agents/agent-049-map-set-delete-20260428T012931Z/deferred-webhook.json`.
