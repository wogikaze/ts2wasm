---
id: 254
title: "Implement class static block runtime semantics"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: ["249"]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement runtime execution semantics for class static initialization blocks after parser classification.

Problem: Issue 249 parses `static { ... }` as `ClassStaticBlock`, but builtin resolution currently rejects static blocks with an issue-linked diagnostic because execution ordering and class environment semantics are not lowered.

## Current failure

```sh
tmp=/tmp/ts2wasm-254-static-block-runtime.ts
printf 'class C { static { console.log(1); } }\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-254-static-block-runtime.wasm
```

Expected current result:

```text
error: [UnsupportedSyntax] issue-249: class static blocks parse, but runtime execution semantics are not implemented
```

## Desired final state

Supported static blocks execute in ECMAScript class element order with Node-compatible observable behavior for the selected class subset.

## Scope

In scope:

- [ ] Define lowering for static block statement lists.
- [ ] Preserve ordering relative to static fields/methods supported by the project.
- [ ] Add Node/iwasm differential fixtures for supported execution order.
- [ ] Keep unsupported forms issue-linked rather than silently ignored.

Out of scope:

- Private names unless coordinated with issue 255.
- Decorators.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run check issues
```

## Notes

Split from issue 249 so parser support can close independently from runtime execution.
