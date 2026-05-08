---
id: 5414
title: "W4: Implement String.replace with RegExp callback semantics"
type: feature
area: runtime
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Implement full String.prototype.replace semantics with RegExp match + callback function to achieve Node/iwasm semantic parity for replace with RegExp arguments.

## Problem

String.prototype.replace currently works with string arguments but lacks full RegExp match semantics (capture groups, global flag iteration) and callback function replacement (`"str".replace(/pattern/g, (match, ...) => ...)`). This blocks ~22 string-builtin test262 cases plus many more indirect RegExp-dependent cases.

Problem: String.replace/replaceAll with RegExp callback is not semantically equivalent to Node.js.

## Current failure

```sh
ts2wasm build fixtures/builtins-and-io/string-replace-regexp-callback.ts
# May build_smoke but fails semantic_diff — Node output differs from iwasm
```

## Desired final state

String.prototype.replace and replaceAll with RegExp pattern + callback function produce identical stdout to Node.js for these cases:
- Simple match + callback replacement
- Global flag + callback (multiple matches)
- Capture group callback arguments (match, g1, g2, ..., offset, string)
- Named capture groups in callback
- Replacement string patterns with `$&`, `$1`, `$` etc.

## Scope

In scope:

- [ ] Implement RegExp match iteration (global flag loop over matches) in WAT runtime
- [ ] Implement callback invocation for each match with correct arguments (match, captures, offset, string)
- [ ] Implement replacement string pattern expansion (`$&`, `$``, `$'`, `$n`, `$<name>`)
- [ ] Handle zero-length match edge cases (advance by 1 char per ECMA-262)
- [ ] Add fixture `fixtures/builtins-and-io/string-replace-regexp-callback.ts`
- [ ] Add Node/iwasm semantic_diff test for these fixtures

Out of scope:

- String.prototype.matchAll (separate issue)
- RegExp.prototype [@@replace] symbol method
- Full Unicode case folding in replacement
- String.prototype.split with RegExp callback

## Affected paths

Expected:

- `crates/backend-wasm/src/runtime_strings.rs` — enhance replace/replaceAll WAT
- `crates/backend-wasm/src/runtime_regexp.rs` — add match-iteration helpers if needed
- `fixtures/builtins-and-io/string-replace-regexp-callback.ts` — new fixture

Do not touch:

- `crates/frontend/` — parser out of scope
- `crates/ir/` — IR out of scope
- `crates/backend-wasm/src/runtime_fn.rs` — no new RuntimeFn variants needed
- `crates/backend-wasm/src/runtime_fn_impl.rs` — no catalog changes
- `crates/backend-wasm/src/runtime_objects.rs`, `runtime_arrays.rs`, etc.

## Acceptance criteria

- [ ] `"abc".replace(/b/, "*")` returns `"a*c"` matching Node
- [ ] `"aba".replace(/a/g, x => x.toUpperCase())` returns `"AbA"` matching Node
- [ ] `"a1b2c3".replace(/(\d)/g, (m, d) => String(Number(d)*2))` returns `"a2b4c6"` matching Node
- [ ] `$&`, `$``, `$'`, `$1` replacement patterns match Node output
- [ ] Zero-length match `"".replace(/(.?)/g, "*")` matches Node behavior

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -- m6_string_regexp
cargo nextest run -- m2_node_diff
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/language-reference/javascript-features.md`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

- Existing `runtime_strings.rs` (~1425 lines) has replace and replaceAll WAT implementations — find them and extend
- RegExp match data is already stored as match result arrays in runtime_regexp.rs
- The callback signature is `fn(match: string, ...captures: string[], offset: number, string: string) -> string`
- For global flag: call exec() repeatedly until null, collect results, call callback for each
