---
id: 052d
title: "Implement broader JSON.stringify replacer semantics"
type: feature
area: runtime/builtins
class: blocked
priority: P1
depends_on: [052g]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: `JSON.stringify` currently supports a narrow object-literal array replacer subset and diagnoses function replacers and unsupported property-list contents.

## Summary

Implement replacer behavior beyond the validated string/numeric-literal object-literal subset, including function callbacks and broader property-list forms when the required call/property semantics are available.

## Current failure

Existing issue 052 evidence records issue-linked diagnostics for function replacer callbacks and unsupported array replacer contents/forms.

Progress note (2026-04-29): the static array property-list slice now supports broader ignored entries and keeps precise diagnostics for dynamic/object-coercion entries. Function replacer callback execution remains split to `052g` because it requires JSON traversal-time callback invocation with root/property key-value arguments and callback return filtering.

## Desired final state

`JSON.stringify(value, replacer)` follows ECMAScript replacer behavior for supported runtime values or emits precise issue-linked diagnostics only for explicitly unsupported forms.

## Scope

In scope:

- [x] Explicitly gate function replacer callbacks with a narrower follow-up (`052g`).
- [x] Expand static array replacer property lists beyond literal string/number entries.
- [x] Preserve property-list ordering and duplicate suppression for supported static keys.
- [x] Add Node differential or diagnostic coverage for every newly supported or intentionally unsupported form in the progress slice.

Out of scope:

- JSON number representation changes.
- UTF-16/surrogate string representation changes.
- General function-call semantics outside the replacer callback surface.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/`
- `issues/open/052-implement-json.md`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] A function replacer fixture either matches Node or reports a precise issue-linked unsupported diagnostic with a narrower follow-up.
- [x] Array replacer contents beyond string/numeric literals are implemented or diagnosed with precise coverage.
- [ ] Existing string/numeric-literal replacer fixtures still match Node.
- [x] Unsupported diagnostics do not mask forms that this issue claims to support.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(json)'
cargo nextest run -p ts2wasm-cli json
cargo nextest run
mise run check issues
```

Impacted commands:

```sh
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-function-unsupported.ts -o /tmp/ts2wasm-json-replacer-function.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts -o /tmp/ts2wasm-json-replacer-array-unsupported.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] update `current-state.md` if the supported JSON subset changes

Follow-up issues:

- [x] created: `issues/open/052g-implement-json-stringify-function-replacer-callbacks.md`
- [x] update `issues/open/052-implement-json.md`

## Progress evidence

2026-04-29:

- Implemented a narrow static `JSON.stringify` array replacer continuation slice for ignored property-list entries. The property-list now accepts and ignores static entries that Node does not add to the replacer property list: boolean/null/undefined literals, declared function identifiers, global `Number`/`String`/`Boolean`/`Object`/`Symbol` identifiers, inline arrow function literals, `Symbol(...)` calls with primitive arguments, side-effect-free object literals, `new Boolean(...)`, and `new Object()`.
- Preserved selected String/Number property-name entries, property-list order, and duplicate suppression.
- Added Node differential fixture `fixtures/builtins-and-io/json-stringify-replacer-array-ignored.ts`; Node and iwasm both print:

```text
{"b":2}
{"a":1}
```

- Updated unsupported diagnostics to cover remaining dynamic/static-coercion gaps:
  - `fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts` covers a dynamic identifier property-list entry (`[key]`), which Node would evaluate as `"a"` but the current compile-time property-list lowering cannot safely fold.
  - `fixtures/builtins-and-io/json-stringify-replacer-array-boxed-unsupported.ts` covers `new Object(2)`, which requires broader object coercion to become the property name `"2"`.
  - `fixtures/builtins-and-io/json-stringify-replacer-function-unsupported.ts` still reports the precise issue-052 function callback diagnostic and is split to `052g`.

Validation result:

```text
command: node fixtures/builtins-and-io/json-stringify-replacer-array-ignored.ts
result: pass; printed {"b":2} and {"a":1}
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-ignored.ts -o /tmp/ts2wasm-json-replacer-array-ignored.wasm && iwasm /tmp/ts2wasm-json-replacer-array-ignored.wasm
result: pass; printed {"b":2} and {"a":1}
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-function-unsupported.ts -o /tmp/ts2wasm-json-replacer-function.wasm
result: expected failure; reports issue-052 function replacer callbacks diagnostic at 59..89
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts -o /tmp/ts2wasm-json-replacer-array-unsupported.wasm
result: expected failure; reports issue-052 static String/Number/ignored-entry subset diagnostic at 27..64
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-boxed-unsupported.ts -o /tmp/ts2wasm-json-replacer-array-boxed-unsupported.wasm
result: expected failure; reports issue-052 static String/Number/ignored-entry subset diagnostic at 12..61
date: 2026-04-29
```

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
