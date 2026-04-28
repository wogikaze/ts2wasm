---
id: 052c
title: "Implement JSON UTF-16 and surrogate string handling"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: JSON string parsing currently supports ASCII and ASCII-valued `\uXXXX` escapes, but rejects non-ASCII code points and surrogate forms that Node accepts.

## Summary

Extend JSON string parse/stringify behavior beyond the current byte-oriented ASCII subset so UTF-16 escape decoding, non-ASCII strings, and surrogate-pair handling match the selected JavaScript string representation.

## Current failure

Existing issue 052 evidence records Node accepting unsupported unicode and surrogate fixtures while iwasm rejects them with `Exception: unreachable`.

## Desired final state

`JSON.parse` and `JSON.stringify` preserve JSON string values across non-ASCII code points and surrogate-pair cases according to the project's string representation contract.

## Scope

In scope:

- [ ] Decode `\uXXXX` escapes for non-ASCII code points.
- [ ] Handle surrogate pairs consistently with Node-observable behavior.
- [ ] Preserve or explicitly define behavior for lone surrogates.
- [ ] Add Node differential coverage for top-level, array-value, and object-value string paths.

Out of scope:

- General-purpose UTF-16 refactors unrelated to JSON.
- Number representation work.
- Replacer callback semantics.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/ir/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/`
- `issues/open/052-implement-json.md`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] `JSON.parse('"\\u00e9"')` matches Node for observable output.
- [ ] `JSON.parse('"\\ud83d\\ude00"')` matches Node for observable output.
- [ ] Object and array string-value paths match the top-level behavior.
- [ ] Existing ASCII escape and invalid unicode escape fixtures still pass.

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
node fixtures/builtins-and-io/json-parse-unsupported-unicode-array.ts
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-unicode-array.ts -o /tmp/ts2wasm-json-unicode-array.wasm && iwasm /tmp/ts2wasm-json-unicode-array.wasm
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
