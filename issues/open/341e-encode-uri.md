---
id: 341e
title: "Implement encodeURI, decodeURI, escape, unescape (audit reopened #341e)"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-05status: open
---

## Summary

Implement `encodeURI`, `decodeURI`, `escape`, and `unescape` global functions.

## Problem

Legacy URI encoding/decoding functions are used in some test262 harness and test cases.

## Desired final state

`encodeURI(str)` and `decodeURI(str)` perform basic URI encoding/decoding. `escape(str)` and `unescape(str)` provide legacy escape/unescape behavior.

## Scope

- [ ] `encodeURI(str)` — percent-encode special characters
- [ ] `decodeURI(str)` — percent-decode
- [ ] `escape(str)` — legacy %-encoding
- [ ] `unescape(str)` — legacy %-decoding

## Affected paths

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/builtin_resolver.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_fn_impl.rs`
- `crates/backend-wasm/src/runtime_builtins_host.rs`

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/341e-encode-uri.md` before this move
- `issues/open/341e-encode-uri.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
