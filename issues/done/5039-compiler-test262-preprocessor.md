---
id: 5039
title: "[compiler] Stabilize test262 preprocessor feature handling (audit reopened #5039)"
type: feature
area: cli
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05
---

## Summary

`features/includes/negative` の対応範囲を明文化し、unsupported feature を tracking ID 付きで分類する。

## Problem

test262 preprocessor の feature/include/negative ハンドリングが暗黙的で、未対応 feature の tracking が不十分。

## Current failure

未対応 feature が tracking ID なしで `unsupported` 扱いになり、improvement の優先順位が不明確。

## Desired final state

全 feature/include/negative が tracking ID 付きで分類され、対応状況が可視化される。

## Scope

In scope:
- [x] features リストの明文化
- [x] unsupported feature の tracking ID 分類
- [x] include/negative ハンドリングの安定化

Out of scope:
- [ ] test262 suite の完全パス

## Affected paths

Expected:
- `crates/cli/`
- `scripts/`

## Acceptance criteria

- [x] 全 feature が tracking ID 付きで分類される
- [x] include/negative ハンドリングが安定する

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Docs / current-state / issue sync

Final-state docs:
- [ ] not affected

Current state:
- [ ] not affected

Follow-up issues:
- [ ] none

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5039-compiler-test262-preprocessor.md` before this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Completion evidence

### Fix applied

Removed 4 duplicate entries from `KNOWN_FEATURES` in `test262_preprocessor.rs`:

1. `("IsHTMLDDA", "issue-5022")` — duplicate of line 17
2. `("Object.fromEntries", "issue-5024")` — duplicate of line 139; kept `issue-5004` (runtime builtins)
3. `("Symbol.species", "issue-5024")` — duplicate of line 84; kept `issue-5000` (commonly used standard)
4. `("Symbol.unscopables", "issue-5025")` — duplicate of line 85; kept `issue-5000` (commonly used standard)

All 119 KNOWN_FEATURES entries are now unique with consistent tracking IDs.

### Files changed

- `crates/compiler/src/test262_preprocessor.rs`: 4 deletions in KNOWN_FEATURES

### Validation

- `cargo fmt --all --check`: pass
- `cargo test -p ts2wasm-compiler test262_preprocessor`: 13/13 pass
- `cargo test -p ts2wasm-compiler`: 55/55 pass
- Duplicate detection confirmed: 0 duplicate features

### Remaining

- `cargo nextest run` fails on pre-existing `check_valid_ts_exits_success` (missing `typescript` npm module, unrelated)
