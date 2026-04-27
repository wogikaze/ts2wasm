---
id: 214
title: "Replace string method placeholders"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Replace placeholder implementations of `String.prototype.trim`, `toUpperCase`, and `toLowerCase` with semantic runtime behavior.

## Problem

Issue 042 records that several string methods currently return the original string as placeholders. Those methods need dedicated semantic implementation and differential fixtures.

## Desired final state

Supported string methods transform strings according to ECMAScript semantics for the project's supported string subset.

## Scope

In scope:

- [x] Implement `String.prototype.trim` for supported whitespace/code unit cases.
- [x] Implement `String.prototype.toUpperCase` and `String.prototype.toLowerCase` for the supported ASCII/Unicode subset chosen by the issue.
- [x] Add Node differential fixtures for changed and unchanged strings.
- [x] Track any Unicode parity gaps explicitly if the first implementation is ASCII-only.

Out of scope:

- Full `Intl` or locale-sensitive case mapping.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- none

## Acceptance criteria

- [x] The listed methods no longer return the input unchanged for cases that require transformation.
- [x] Node differential fixtures prove trim and case conversion for supported strings.
- [x] Any unsupported Unicode behavior is issue-linked and not counted as semantic pass.
- [x] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(string)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] update `docs/language-reference/javascript-features.md`

Current state:

- [x] update `current-state.md`

Follow-up issues:

- [x] none

## Notes

Created from issue 203 audit of `issues/done/042-implement-string-methods.md`.

## Completion evidence

Commits:

- `b41e7b6` issue-214: implement ascii string methods
- close commit for docs/issue sync

Validation result:

```text
command: cargo test -p ts2wasm-cli --test m2_node_diff string_method_fixtures_match_node_output_under_iwasm -- --nocapture
result: pass (trim, toUpperCase, and toLowerCase fixtures matched Node stdout under iwasm)
date: 2026-04-28

command: cargo nextest run -E 'test(string)'
result: pass (17 passed)
date: 2026-04-28

command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: scripts/manager check-agent-state
result: pass
date: 2026-04-28

command: scripts/manager update-issue-index --check
result: pass
date: 2026-04-28

command: scripts/manager check-issue-health
result: pass
date: 2026-04-28

command: scripts/manager check-repo-smoke
result: pass
date: 2026-04-28

command: cargo nextest run
result: pass (205 passed, 4 skipped)
date: 2026-04-28

command: cargo clippy --all-targets --all-features
result: pass with pre-existing warnings outside issue 214 scope
date: 2026-04-28
```

Remaining risks:

- Unicode whitespace beyond ASCII HT/LF/VT/FF/CR/space and Unicode case folding are outside the current byte-oriented runtime string subset and are not counted as semantic pass.
