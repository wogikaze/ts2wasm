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

Problem: Issue 248 tokenizes `#name` and parses private fields, methods, getters, and setters. The runtime slices now support direct instance private field initialization plus `this.#field` read/write inside class constructors and instance methods, direct same-class instance private methods called as `this.#m()`, direct same-class static private methods called as `this.#m()` from static methods or `Class.#m()` inside the declaring class, direct same-class private getters read as `this.#x`, and direct same-class private setters assigned as `this.#x = value` for non-derived classes. Full private brand storage, static private fields/accessors, derived-class private initialization, external/extracted private element access, and complete brand-checking behavior remain incomplete.

## Remaining failure

```sh
tmp=/tmp/ts2wasm-255-private-runtime.ts
printf 'class C { static set #x(value) {} }\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-255-private-runtime.wasm
```

Expected remaining result:

```text
error: [UnsupportedSyntax] issue-255: static private accessors are not supported in this private accessor runtime slice
```

## Desired final state

Supported private class fields and methods behave like Node for construction, access, updates, and brand checks.

## Scope

In scope:

- [x] Define an internal slot representation for the supported non-derived instance private field subset.
- [ ] Define full private-name representation and class brand storage.
- [ ] Lower private methods, getters, setters, static private elements, and derived private initialization.
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
- Reserved the temporary private backing-key namespace from ordinary property access and rejects key enumeration on private-field progress objects, so `c["__ts2wasm_private::Class::field"]` and `Object.keys(c)` do not silently expose private storage.
- Added Node/iwasm differential coverage: `fixtures/core-semantics/private-class-field-read-write.ts`.
- Added unsupported diagnostics coverage: `fixtures/core-semantics/private-class-field-method-unsupported.ts`, `fixtures/core-semantics/private-class-field-external-unsupported.ts`, `fixtures/core-semantics/private-class-field-backing-key-unsupported.ts`, and `fixtures/core-semantics/private-class-field-object-keys-unsupported.ts`.

2026-04-29 internal-slot progress slice:

- Replaced supported `this.#field` read/write storage with backend-internal private slots instead of ordinary string-keyed properties.
- Class instance allocation records the private slot count and appends private slot payload after the existing public-property capacity.
- GC object marking now scans private slots so heap values retained only by supported private fields survive allocation pressure.
- Kept conservative diagnostics for observable ordinary access/enumeration patterns that could otherwise mask private-storage leaks while full brand semantics remain open.
- Added IR regression coverage for `PrivateFieldGet`/`PrivateFieldSet` lowering and private slot count allocation metadata.
- Added Node/iwasm differential allocation-pressure coverage: `fixtures/core-semantics/private-class-field-internal-slot-gc.ts`.
- Verified the parent review repro `c["__ts2wasm_private::Counter::value"]` fails with the issue-255 diagnostic instead of compiling to a wrong observable value.

Validation recorded in the child branch:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(lowering_represents_private_field_access_as_internal_slot_calls)'
cargo nextest run -E 'test(private_class_field_read_write_fixture_matches_node_output_under_iwasm) or test(private_class_field_unsupported_forms_report_issue_255)'
cargo nextest run -E 'test(private) or test(class) or test(node_diff)'
cargo nextest run
mise run update-issue-index -- --check
mise run check issues
```

Manual leak repro recorded:

```sh
tmp=/tmp/ts2wasm-255-private-leak.ts
printf 'class Counter { #value = 7; read(){ return this.#value; } }\nlet c = new Counter();\nconsole.log(c["__ts2wasm_private::Counter::value"]);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-255-private-leak.wasm
```

Result:

```text
error: [UnsupportedSyntax] issue-255: private field backing storage is not accessible through ordinary property access in this private field runtime slice
```

2026-04-29 private method/getter/setter progress slices:

- Added direct non-derived instance private method support for same-class `this.#m(...)` calls.
- Added direct non-derived instance private getter support for same-class `this.#x` reads.
- Added direct non-derived instance private setter support for same-class `this.#x = value` assignments by lowering setters to internal same-class methods that return the assigned value for assignment-expression compatibility.
- Kept static private fields/accessors, derived private elements, extracted/private external method/accessor use, and full brand semantics on issue-255 diagnostics.
- Added IR lowering regression coverage for method/getter/setter same-class calls and Node/iwasm differential coverage for `fixtures/core-semantics/private-class-method-call.ts`, `fixtures/core-semantics/private-class-getter-direct.ts`, and `fixtures/core-semantics/private-class-setter-direct.ts`.
- Added unsupported diagnostics coverage for external/extracted method access, external getter access, static setter declaration, and external setter assignment.

Validation recorded in child branches:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(private) or test(class) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

2026-04-29 static private method progress slice:

- Added direct same-class static private method support for `this.#m(...)` inside static methods and `Class.#m(...)` inside the declaring class.
- Represented supported static private methods as internal `static::#m` class methods so existing static method lowering can call them without an instance receiver.
- Kept static private fields/accessors, derived private elements, external/extracted private access, and full brand semantics on issue-255 diagnostics.
- Added Node/iwasm differential coverage: `fixtures/core-semantics/private-class-static-method-call.ts`.
- Added unsupported diagnostics coverage: `fixtures/core-semantics/private-class-static-method-external-unsupported.ts`, and kept derived static private method declaration covered by `fixtures/core-semantics/private-class-field-method-unsupported.ts`.

Validation recorded in child branch:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(private) or test(class) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

2026-04-29 static private accessor diagnostic slice:

- Kept static private getters/setters on issue-255 diagnostics because they need class-level private accessor storage/brand semantics rather than the current instance getter/setter lowering.
- Added unsupported diagnostics coverage for `static get #value()`, `static set #value(next)`, same-class static `this.#value` read, `this.#value = next` write, and `Class.#value` read attempt: `fixtures/core-semantics/private-class-static-accessor-unsupported.ts`.
- Preserved the existing narrower static setter declaration fixture while adding explicit getter/access attempt coverage.

Validation recorded in child branch:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(private) or test(class) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

2026-04-29 static private field diagnostic slice:

- Kept static private fields on issue-255 diagnostics because mutable static private field support needs class-level private storage, not the existing instance private slot model or static method call lowering.
- Added unsupported diagnostics coverage for a same-class static private field initializer plus `this.#value` read, `this.#value = value` write, and `Class.#value` read attempt: `fixtures/core-semantics/private-class-static-field-unsupported.ts`.
- Verified the diagnostic remains source-spanned at the static private field declaration instead of compiling into an unsafe partial static storage model.

Validation recorded in child branch:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(private) or test(class) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

2026-04-29 private delete diagnostic slice:

- Hardened delete lowering so `delete this.#value` reports issue-255 instead of compiling as an ordinary property delete.
- Hardened the guarded private backing-key path so `delete c["__ts2wasm_private::C::value"]` reports the existing private storage leak diagnostic instead of bypassing the computed-access guard.
- Added unsupported diagnostics coverage: `fixtures/core-semantics/private-class-delete-unsupported.ts` and `fixtures/core-semantics/private-class-delete-backing-key-unsupported.ts`.

Validation recorded in child branch:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(private) or test(class) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```
