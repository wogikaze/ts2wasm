---
id: 052d
title: "Implement broader JSON.stringify replacer semantics"
type: feature
area: runtime/builtins
class: blocked
priority: P1
depends_on: [5004]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: `JSON.stringify` currently supports function replacer callbacks and a static object-literal array replacer subset, but broader dynamic property-list and object-coercion replacer forms remain unsupported.

## Summary

Implement replacer behavior beyond the validated function-callback and static object-literal property-list subset, including broader property-list forms when the required object-coercion semantics are available.

## Current failure

Existing issue 052 evidence records issue-linked diagnostics for unsupported array replacer contents/forms.

Progress note (2026-04-29): the static array property-list slice now supports broader ignored entries and keeps precise diagnostics for dynamic/object-coercion entries. Function replacer callback execution was split to `052g` and is now closed for the currently supported runtime value subset.

## Desired final state

`JSON.stringify(value, replacer)` follows ECMAScript replacer behavior for supported runtime values or emits precise issue-linked diagnostics only for explicitly unsupported forms.

## Scope

In scope:

- [x] Function replacer callbacks are covered by closed child issue `052g`.
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

- [x] Function replacer fixtures match Node for the currently supported runtime subset.
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
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-function-keep.ts -o /tmp/ts2wasm-json-replacer-function.wasm
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

- [x] created and closed: `issues/done/052g-implement-json-stringify-function-replacer-callbacks.md`
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
  - Function replacer callbacks remained split to `052g` in this historical slice; `052g` later replaced that diagnostic fixture with Node differential coverage.

Validation result:

```text
command: node fixtures/builtins-and-io/json-stringify-replacer-array-ignored.ts
result: pass; printed {"b":2} and {"a":1}
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-ignored.ts -o /tmp/ts2wasm-json-replacer-array-ignored.wasm && iwasm /tmp/ts2wasm-json-replacer-array-ignored.wasm
result: pass; printed {"b":2} and {"a":1}
date: 2026-04-29

command: historical function-replacer unsupported fixture build
result: historical expected failure superseded by issue 052g; replacement fixture is `fixtures/builtins-and-io/json-stringify-replacer-function-keep.ts`
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
