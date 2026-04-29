---
id: 255
title: "Implement private class element runtime semantics"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: ["248"]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement runtime storage and access semantics for private class elements after lexer/parser classification.

Problem: Issue 248 tokenizes `#name` and parses private fields, methods, getters, and setters, but builtin resolution rejects them because private-name storage, brand checks, and access semantics are not implemented.

## Current failure

```sh
tmp=/tmp/ts2wasm-255-private-runtime.ts
printf 'class C { #x = 1; }\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-255-private-runtime.wasm
```

Expected current result:

```text
error: [UnsupportedSyntax] issue-248: private class elements parse, but runtime storage/access semantics are not implemented
```

## Desired final state

Supported private class fields and methods behave like Node for construction, access, updates, and brand checks.

## Scope

In scope:

- [ ] Define private-name representation and class brand storage.
- [ ] Lower private fields, methods, getters, and setters for the supported class subset.
- [ ] Reject unsupported private access forms with issue-linked diagnostics.
- [ ] Add Node/iwasm differential fixtures for supported private element behavior.

Out of scope:

- Decorators.
- Optional chaining of private fields unless coordinated with issue 253.

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

Split from issue 248 so parser support can close independently from runtime semantics.
