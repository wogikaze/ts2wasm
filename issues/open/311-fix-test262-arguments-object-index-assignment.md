---
id: 311
title: "Fix test262 arguments object index assignment semantics"
type: bug
area: runtime/semantics
class: blocked
priority: P0
depends_on: ["274"]
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

The original test262 run had one P0 semantic failure in the executed window.
The local differential fixture for admitted out-of-range `arguments` index
assignment now passes. The issue-247 parser/classification blocker
(`undefined` keyword not accepted as binding identifier in WASM globals shim)
is fixed. The test now reaches issue-274 (spread operator IIFE with `this`
limitation).

## Problem

`arguments[7] = 12` should create or update the indexed property and a
subsequent `arguments[7]` read should observe `12`. The current wasm output
returns the test262 assertion sentinel instead.

The test262 runner prepends `var undefined = void 0;` (WASM globals shim) which
was blocked by the parser not accepting `undefined` keyword tokens as binding
identifiers. That parser blocker is now fixed. The IR resolver then rejects the
`fnGlobalObject()` IIFE `(function() { return this; })()` used in the WASM
harness shim (tracked by issue 274).

Problem: arguments object out-of-range index assignment fails semantic comparison
in `reference/test262/test/language/arguments-object/10.5-7-b-2-s.js`.

## Current status

Test262 runs consistently produce the following sequence:

1. Parser: `var undefined = void 0;` no longer fails (fixed by making
   `Token::Undefined` a valid binding identifier in `parse_binding_pattern`).
2. IR resolver: `(function() { return this; })()` in `fnGlobalObject()` is
   rejected by issue-274's spread-IIFE-with-`this` guard.

Focused test262 result on 2026-04-30 after parser fix:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference \
  python3 scripts/run/test262.py \
  --path-filter language/arguments-object/10.5-7-b-2-s.js \
  --jobs 1
```

```text
Pass: 0
Fail: 0
Unsupported: 1
Blocked: 0
Total: 1
reason: UnsupportedSyntax/feature-unsupported: [UnsupportedSyntax]
stderr: error: [UnsupportedSyntax] issue-274: direct function-expression
  spread calls with `this` or `arguments` require broader call-expression
  runtime support at 1088..1118
```

The span 1088..1118 points to `(function() { return this; })()` in the
`fnGlobalObject()` function of the WASM harness shim:

```js
function fnGlobalObject() {
  return (function() { return this; })();
}
```

This IIFE pattern is used to obtain the global object. The issue-274 spread
lowering check (`block_contains_this`) is overly broad: it rejects any
function-expression call whose body contains `this`, even when no spread
syntax is present. Fixing this requires narrowing the issue-274 guard or
rewriting the harness shim to avoid IIFE-with-`this`.

## Desired final state

The representative test262 case passes under the test262 runner. Supported
`arguments` object indexed writes and reads match Node/iwasm differential
behavior for out-of-range indexes.

## Scope

In scope:

- [x] Implement the smallest arguments-object indexed write/read semantics
      needed for `arguments[7] = 12; arguments[7]`.
- [x] Preserve existing supported `arguments.length` and indexed read behavior.
- [x] Add or update a focused fixture for out-of-range `arguments` index
      assignment.
- [x] Fix issue-247 parser/classification blocker (`Token::Undefined` as
      binding identifier).
- [ ] Resolve issue-274 blocker (fnGlobalObject IIFE with `this` rejected by
      spread operator guard) — tracked by issue 274, see notes below.

Out of scope:

- Full exotic arguments object aliasing semantics.
- Iterator support for arguments objects.
- Unrelated Object descriptor or callee/caller attributes.
- Broad test262 harness changes.

## Affected paths

Expected:

- `crates/frontend/src/parser/binding_patterns.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/ir/src/lowered/resolver_extra.rs`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `artifacts/coverage/results/test262.json` only if the validation run is
  intentionally refreshed

Do not touch:

- `scripts/check/architecture-rules.py`
- `.githooks/pre-push`
- unrelated web-ui UI code

## Acceptance criteria

- [x] Add an equivalent fixture for arguments-object out-of-range index writes
      and verify it matches Node output under iwasm.
- [x] Fix the issue-247 parser/classification blocker: `undefined` keyword
      token accepted as a binding identifier in `parse_binding_pattern`.
- [x] `arguments.length` is not incorrectly extended unless the selected
      fixture proves the ECMAScript behavior requires it for this slice.
- [x] Existing `function_arguments_fixture_matches_node_output_under_iwasm`
      coverage still passes.
- [ ] The representative test262 runner command reports `Pass: 1`, `Fail: 0`.
      Blocked by issue-274 (spread operator IIFE limitation).

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(arguments) or test(node_diff) or test(parser)'
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference python3 scripts/run/test262.py --path-filter language/arguments-object/10.5-7-b-2-s.js --jobs 1
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 18000
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] issue-247 parser/classification blocker fixed in this issue
- [x] narrowed blocker: issue-274 (spread operator IIFE guard is overly broad)
      records the exact failure; track issue-274 resolution separately.

## Notes

The issue-274 check at `resolver_extra.rs:725` is overly broad: it rejects
`lower_function_expr_call` when the function body contains `this` or
`arguments`, even without any spread syntax. The WASM harness
`fnGlobalObject()` uses `(function() { return this; })()` which triggers this
guard. The fix should narrow the check to only reject spread calls with
`this`/`arguments`, not all function-expression calls.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
