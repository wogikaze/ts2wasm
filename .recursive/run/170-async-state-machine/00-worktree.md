# Phase 0: Worktree Isolation

## Worktree Selection

- **Location**: `.worktrees/async-state-machine` (pre-existing)
- **Branch**: `recursive/async-state-machine`
- **Base commit**: `a7b85139e` (tracking: close item 168 slice 1)

## Git-Ignore Status

`.worktrees/` is NOT in `.gitignore` (only `_worktrees/`). This is a pre-existing worktree.

## Setup Verification

- `cargo build` — passes
- `cargo nextest run -p ts2wasm-cli --test m12_async_await` — all 6 tests PASS (items 170, 171, 172, 192)
- `cargo nextest run -p ts2wasm-cli --test m2_node_diff` — 11 pass, 1 FAIL (`bun_stdin_text_fixture_matches_node_baseline_under_iwasm`)

## Baseline Test State

| Test Suite | Result |
|---|---|
| m12_async_await (6 tests) | ALL PASS |
| m2_node_diff (12/306 run) | 11 PASS, 1 FAIL (pre-existing bun_stdin_text failure) |

## Active Items Status

- **170** (state-machine): acceptance test passes — can close
- **171** (polling ABI): acceptance test passes — can close
- **172** (Promise wrapper): acceptance test passes — can close
- **192** (CR + async): acceptance test passes — can close (same test as 172)
- **168** (UTF-8 string): acceptance = all m2_node_diff pass — 1 pre-existing failure needs investigation

## Normalized Diff Basis

- Baseline type: git commit
- Baseline reference: `a7b85139e`
- Comparison reference: `HEAD`

## TODO

- [ ] Close items 170, 171, 172, 192 in TRACKING.yaml
- [ ] Investigate item 168 — slice into smaller TDD steps
- [ ] Proceed with Phases 1-3

All subsequent work runs from this worktree (`/home/wogikaze/wgkz/ts2wasm/.worktrees/async-state-machine`).
