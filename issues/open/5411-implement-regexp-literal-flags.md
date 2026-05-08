---
id: 5411
title: "W2: Implement RegExp literal flags parser support"
type: feature
area: frontend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Implement parser support for all RegExp literal flags (g, i, m, s, u, y, d) to reduce UnsupportedSyntax from ~45 regexp-literal-related test262 failures at limit 500.

## Problem

test262 has ~45 RegExp literal cases failing with UnsupportedSyntax at limit 500 because only basic RegExp flag combinations are parsed. RegExp flags `u`, `y`, `s`, `d` in particular trigger parser diagnostic exits.

Problem: RegExp literal flags (g,i,m,s,u,y,d) not fully parsed.

## Current failure

```sh
mise run reference-coverage -- test262 --limit 500 --detail
# feature "regexp-literal" shows 45 unsupported cases
```

## Desired final state

All RegExp literal flags (g, i, m, s, u, y, d) in any combination are parsed without UnsupportedSyntax diagnostic. RegExp literal with any valid flag combination passes build_smoke.

## Scope

In scope:

- [ ] Add `u` (unicode), `y` (sticky), `s` (dotAll), `d` (indices) to RegExp flag token/parser handling in `crates/frontend/src/parser/expressions.rs`
- [ ] Ensure all combinations of g, i, m, s, u, y, d are accepted
- [ ] Add build_smoke fixture test for each flag combination
- [ ] Validate with `mise run reference-coverage -- test262 --limit 500` that regexp-literal unsupported count decreases

Out of scope:

- RegExp runtime semantics (test, exec flags) — already implemented in runtime_regexp.rs
- Non-literal RegExp (new RegExp(...)) constructor
- Unicode property escapes (\p{...})
- Lookbehind, named capture groups runtime

## Affected paths

Expected:

- `crates/frontend/src/parser/expressions.rs` — RegExp literal parsing
- `crates/frontend/src/parser/tokens.rs` — token type if new flags needed
- `fixtures/builtins-and-io/regexp-flag-*.ts` — new fixture files
- `crates/cli/tests/m6_regexp_flags.rs` — new test file for flag fixtures

Do not touch:

- `crates/backend-wasm/src/runtime_*.rs` — runtime is out of scope
- `crates/ir/src/lowered/` — IR is out of scope
- `crates/ir/src/name_resolver.rs` — name resolver out of scope

## Acceptance criteria

- [ ] RegExp literal `/foo/u`, `/foo/y`, `/foo/s`, `/foo/d` all parse without diagnostic
- [ ] Mixed flags `/foo/gimy`, `/foo/msy`, `/foo/dg` all parse without diagnostic
- [ ] Invalid flag combinations (e.g., duplicate) produce appropriate diagnostic
- [ ] At least 2 build_smoke fixtures added for flag combinations
- [ ] `mise run reference-coverage -- test262 --limit 500` shows "regexp-literal" feature unsupported count decreased from 45

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- test262 --limit 500
```

Impacted commands:

```sh
# Verify regexp-literal unsupported count decreased
mise run reference-coverage -- test262 --limit 500 --detail | grep regexp-literal
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/language-reference/javascript-features.md`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] created/updated: meta-issue 5000 (parser syntax coverage)

## Notes

- Reference: crates/frontend/src/parser/expressions.rs (~2000 lines), find the RegExp literal parsing section
- The existing parser already handles basic RegExp without flags or with `g`/`i`/`m`
- Flag enum values match the ASCII character — store as bitmask for easy combination checking
