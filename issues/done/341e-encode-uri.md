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

- [x] `encodeURI(str)` — percent-encode special characters (host import shim via Node)
- [x] `decodeURI(str)` — percent-decode (host import shim via Node)
- [x] `escape(str)` — legacy %-encoding (host import shim via Node)
- [x] `unescape(str)` — legacy %-decoding (host import shim via Node)

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
- `issues/open/341e-encode-uri.md` (moved to done/ per close evidence)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Completion evidence

Implementation completed in commit `715deb74` ("Close issue 341e: implement encodeURI, decodeURI, escape, unescape as Node host shim imports"). Full pipeline:

- **`crates/ir/src/builtin.rs`**: `BuiltinId::EncodeURI`, `DecodeURI`, `Escape`, `Unescape` variants
- **`crates/ir/src/lowered/resolver_expr.rs`**: `resolve_global_identifier_call()` routes identifier names to BuiltinId
- **`crates/backend-wasm/src/runtime_fn.rs`**: 4 `RuntimeFn` variants with `HostImport` enum entries
- **`crates/backend-wasm/src/runtime_fn_impl.rs`**: `from_builtin` mapping and `RuntimeSpec` entries with host ABI
- **`crates/backend-wasm/src/runtime_builtins_host.rs`**: WAT emit functions calling host imports
- **`crates/backend-wasm/src/runtime_builder.rs`**: Builder dispatch arms
- **`fixtures/builtins-and-io/`**: `global-encode-uri.ts`, `global-decode-uri.ts`, `global-escape.ts`, `global-unescape.ts`
- **`crates/cli/tests/m6_builtin_methods.rs`**: Build-smoke tests for all four functions

### Validation
```sh
cargo nextest run --package ts2wasm-cli --test m6_builtin_methods
```
Result: 105 tests passed (including 4 encodeURI/decodeURI/escape/unescape build-smoke tests).

### Runtime note
The host-side JS shims (`$host_encode_uri`, etc.) are expected in the external Node.js runner, not in the WASM binary. Build-smoke tests verify compilation; runtime execution requires Node host shim support.
