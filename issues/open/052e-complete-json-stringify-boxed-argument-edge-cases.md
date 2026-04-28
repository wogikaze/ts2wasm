---
id: 052e
title: "Complete JSON.stringify boxed argument edge cases"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: `JSON.stringify` has validated narrow boxed `space` handling, but broader boxed and object-coercion edge cases are not closed.

## Summary

Audit and implement the remaining boxed/object argument cases for `JSON.stringify` `replacer` and `space` semantics, or split any large runtime-coercion dependency into narrower child issues.

## Current failure

Existing issue 052 evidence closes only narrow boxed `Number`, `String`, and `Boolean` `space` forms and leaves boxed forms beyond those fixtures as remaining gaps.

## Desired final state

Boxed values passed to `JSON.stringify` arguments either match Node for the supported runtime object model or produce precise unsupported diagnostics with a smaller tracking issue.

## Scope

In scope:

- [ ] Audit boxed `Number`, `String`, `Boolean`, and object coercion paths for the `space` argument.
- [ ] Audit boxed entries in array replacer property lists.
- [ ] Add Node differential or diagnostic fixtures for each selected edge case.

Out of scope:

- Function replacer callback execution.
- General object model features not required to classify the selected boxed cases.
- Non-ASCII JSON string support.

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

- [ ] Boxed `space` behavior beyond the currently covered literals is either Node-matching or explicitly diagnosed.
- [ ] Boxed array replacer entries are either Node-matching or explicitly diagnosed.
- [ ] Existing `json-stringify-space-boxed-symbol` coverage still passes.
- [ ] Any deferred object-model dependency has a separate issue reference.

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
node fixtures/builtins-and-io/json-stringify-space-boxed-symbol.ts
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-space-boxed-symbol.ts -o /tmp/ts2wasm-json-space-boxed-symbol.wasm && iwasm /tmp/ts2wasm-json-space-boxed-symbol.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] update `current-state.md` if the supported JSON subset changes

Follow-up issues:

- [ ] update `issues/open/052-implement-json.md`

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
