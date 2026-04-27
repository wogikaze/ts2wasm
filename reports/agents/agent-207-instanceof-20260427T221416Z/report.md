# agent-207-instanceof-20260427T221416Z Report

Issue: 207 - Complete instanceof prototype-chain semantics
Branch: `agent/207-instanceof-20260427T221416Z`
Status: DONE
Date: 2026-04-28

## Summary

Implemented runtime `instanceof` semantics for the supported class-constructor subset. Lowering now routes `obj instanceof ClassName` to a class prototype object, `new ClassName(...)` stores that prototype in the instance prototype slot, and `$instanceof` walks the runtime prototype chain instead of returning a fixed placeholder false value.

## Changes

- Added lowered IR support for class prototype references with parent constructor chains.
- Added backend class prototype globals initialized in `_start`.
- Wired `new` instances to their class prototype object.
- Replaced `$instanceof` placeholder with prototype-chain traversal.
- Added Node differential coverage for true, false, inherited, non-object-left, and manually linked prototype-chain cases.
- Added an unsupported RHS diagnostic fixture with an issue-207 diagnostic.
- Moved issue 207 to `issues/done/`, refreshed `issues/index.md`, and updated docs/current-state.
- Updated historical issue 030 links from the old open follow-up path to the completed issue 207 path.

## Validation

- PASS: `cargo test -p ts2wasm-cli --test m2_node_diff instanceof -- --nocapture`
- PASS: `cargo nextest run -E 'test(instanceof)'` (4 passed)
- PASS: `cargo test -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm -- --nocapture`
- PASS: `cargo fmt --all --check`
- PASS: `scripts/manager check-agent-state`
- PASS: `scripts/manager update-issue-index --check`
- PASS: `scripts/manager check-issue-health`
- PASS: `scripts/manager check-repo-smoke`
- PASS: `cargo nextest run` (206 passed, 4 skipped)
- FAIL, out of scope: `cargo clippy --workspace --all-targets -- -D warnings` reports pre-existing `clippy::assertions-on-constants` in `crates/runtime-abi/src/layout.rs`.

## Notes

- Custom `Symbol.hasInstance` remains explicitly out of scope for issue 207.
- Discord webhook delivery was attempted twice and deferred because `DISCORD_WEBHOOK_URL` is not configured.
