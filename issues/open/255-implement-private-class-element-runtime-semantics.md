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

Problem: Issue 248 tokenizes `#name` and parses private fields, methods, getters, and setters. The first runtime slice now supports direct instance private field initialization plus `this.#field` read/write inside class constructors and instance methods, but private brand storage, private methods/getters/setters, static private elements, derived-class private initialization, and external/private brand-checking behavior remain incomplete.

## Remaining failure

```sh
tmp=/tmp/ts2wasm-255-private-runtime.ts
printf 'class C { #m() { return 1; } }\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-255-private-runtime.wasm
```

Expected remaining result:

```text
error: [UnsupportedSyntax] issue-255: private methods are not supported in this private field runtime slice
```

## Desired final state

Supported private class fields and methods behave like Node for construction, access, updates, and brand checks.

## Scope

In scope:

- [ ] Define private-name representation and class brand storage.
- [ ] Lower private fields, methods, getters, and setters for the supported class subset.
- [x] Reject unsupported private access forms with issue-linked diagnostics.
- [x] Add Node/iwasm differential fixtures for supported private element behavior.

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

## Progress evidence

2026-04-29 progress slice:

- Added parsing/lowering for direct private member syntax needed by runtime tests: `this.#field`.
- Added constructor injection for instance private field initializers on non-derived classes.
- Added `this.#field` read/write lowering for fields declared on the current class.
- Added issue-linked diagnostics for private methods and external/non-`this` private field access in the current slice.
- Added Node/iwasm differential coverage: `fixtures/core-semantics/private-class-field-read-write.ts`.
- Added unsupported diagnostics coverage: `fixtures/core-semantics/private-class-field-method-unsupported.ts` and `fixtures/core-semantics/private-class-field-external-unsupported.ts`.

Validation recorded in the child branch:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(private_class_field_read_write_fixture_matches_node_output_under_iwasm) or test(private_class_field_unsupported_forms_report_issue_255)'
cargo nextest run -E 'test(private) or test(class) or test(node_diff)'
cargo nextest run
mise run update-issue-index -- --check
mise run check issues
```
