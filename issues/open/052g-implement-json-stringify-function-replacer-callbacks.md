---
id: 052g
title: "Implement JSON.stringify function replacer callbacks"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: [052d]
created: 2026-04-29
updated: 2026-04-29
---

Problem: `JSON.stringify` still reports issue-052 diagnostics for function replacer callbacks instead of invoking the callback for the root and visited property values.

## Summary

Implement the callback branch of `JSON.stringify(value, replacer)` for the currently supported runtime value subset, or split any missing call/object traversal dependency into a smaller child issue with fixture evidence.

## Current failure

```sh
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-function-unsupported.ts -o /tmp/ts2wasm-json-replacer-function.wasm
```

Current result: rejects with `[UnsupportedSyntax] issue-052: JSON.stringify function replacer callbacks are not supported yet ... at 59..89`.

Node executes the fixture callback and prints `{"a":1}`.

## Desired final state

Function replacer callbacks are invoked with the ECMAScript `JSON.stringify` callback surface for the supported value subset, including root key `""`, property keys, callback return filtering/transformation, and the selected receiver/holder semantics.

## Scope

In scope:

- [ ] Invoke declared function replacers and inline arrow replacers for supported object/array/primitive values.
- [ ] Pass the correct key and value arguments for the root and visited properties.
- [ ] Implement callback return handling for supported return values and `undefined` filtering.
- [ ] Add Node differential fixtures for keep, drop, and transform behavior.

Out of scope:

- Full JSON number representation changes.
- UTF-16/surrogate string representation changes.
- General callback semantics unrelated to this JSON traversal surface.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/`
- `issues/open/052-implement-json.md`
- `issues/open/052d-implement-json-stringify-broader-replacer-semantics.md`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] `fixtures/builtins-and-io/json-stringify-replacer-function-unsupported.ts` is replaced by Node differential coverage or a narrower diagnostic fixture for a specifically unsupported callback subcase.
- [ ] A callback that returns `undefined` for one object property matches Node by omitting that property.
- [ ] A callback that transforms one supported primitive value matches Node.
- [ ] Existing array replacer fixtures still match Node.

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
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] update `current-state.md` if the supported JSON subset changes

Follow-up issues:

- [ ] none

## Notes

This issue was split from 052d after the 052d progress slice broadened static array property-list semantics but did not implement callback execution.

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
