---
id: 054
title: "Implement Error types"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-28
status: done
completed: 2026-04-28
---

## Summary

Implement Error, TypeError, and other error types.

## Problem

Error types are not implemented. They are essential for error handling.

## Desired final state

`new Error()`, `new TypeError()`, etc. work correctly.

## Scope

In scope:

- [x] Implement Error constructor
- [x] Implement TypeError constructor
- [x] Implement ReferenceError constructor
- [x] Implement SyntaxError constructor
- [x] Implement Error.prototype.message
- [x] Implement Error.prototype.stack
- [x] Add fixtures for Error behavior

Out of scope:

- Full Error spec compliance (start with basic error types)

## Affected paths

Expected:

- `crates/backend-wasm/src/` (runtime builtins)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Error constructors work correctly
- [x] Error properties work correctly
- [x] Fixtures cover Error behavior
- [x] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/error-test.ts -o /tmp/test.wasm
iwasm /tmp/test.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Progress 2026-04-28:

- Reproduced current behavior for `new Error("msg")`, `new TypeError("msg")`, `new ReferenceError("msg")`, and `new SyntaxError("msg")`: each failed at build time with `issue-207: instanceof right-hand side must be a supported class constructor`.
- Implemented a first runtime slice that lowers `new Error(...)`, `new TypeError(...)`, `new ReferenceError(...)`, and `new SyntaxError(...)` to heap objects with a Node-differential `.message` property.
- Added `fixtures/builtins-and-io/error-message.ts` and `error_message_fixture_matches_node_output_under_iwasm`.
- Remaining criteria before close: `.stack` is still not implemented; non-string message coercion and Error prototype identity are not yet covered.

Progress 2026-04-28 continuation:

- Implemented Error constructor message coercion for non-string messages through `$error_message`, which returns `""` for `undefined` and otherwise materializes the existing `ToString` result as a heap string.
- Extended `fixtures/builtins-and-io/error-message.ts` with Node differential coverage for number, boolean, null, and explicit undefined message arguments across Error subclasses.
- Validation passed: `cargo fmt --all --check`; `cargo nextest run -E 'test(error)'`; `cargo nextest run -p ts2wasm-cli error`; `cargo run -p ts2wasm-cli -- build fixtures/builtins-and-io/error-message.ts -o /tmp/ts2wasm-054-error-message.wasm && iwasm /tmp/ts2wasm-054-error-message.wasm`; `cargo nextest run`; `mise run check-issue-health`; `mise run check-agent-state`.
- Remaining criteria before close: `.stack` is still not implemented; Error prototype identity / `instanceof Error` remains uncovered.

Progress 2026-04-28 continuation 2:

- Reproduced the remaining prototype gap with a temporary fixture: Node printed `true` for `new Error("generic") instanceof Error`, `new TypeError("type") instanceof TypeError`, and `new TypeError("type") instanceof Error`, while the compiler failed at build time with `issue-207: instanceof right-hand side must be a supported class constructor \`Error\``.
- Added built-in Error prototype lowering for Error, TypeError, ReferenceError, and SyntaxError. Error instances now carry built-in prototype payloads, subclass prototypes chain through Error.prototype, and the existing `$instanceof` runtime helper observes those chains.
- Added `fixtures/builtins-and-io/error-instanceof.ts` and `error_instanceof_fixture_matches_node_output_under_iwasm`, covering self `instanceof`, subclass-to-Error `instanceof`, cross-subclass negatives, plain object negative, and primitive left-hand negative.
- Direct Node vs iwasm evidence for `fixtures/builtins-and-io/error-instanceof.ts` matched:
  `true true true true true true true false false false false` on separate stdout lines.
- Validation passed: `cargo check -p ts2wasm-backend-wasm -p ts2wasm-ir -p ts2wasm-cli`; `cargo fmt --all --check`; `cargo nextest run -E 'test(error)'`; `cargo nextest run -p ts2wasm-cli error`; `node fixtures/builtins-and-io/error-instanceof.ts`; `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/error-instanceof.ts -o /tmp/ts2wasm-054-error-instanceof.wasm && iwasm /tmp/ts2wasm-054-error-instanceof.wasm`; `mise run check-issue-health`; `mise run check-agent-state`.
- Remaining criteria before close: `.stack` is still not implemented; full Error spec compliance and full `cargo nextest run` close validation remain outstanding.

Progress 2026-04-28 continuation 3:

- Reproduced the `.stack` gap with a temporary fixture: Node printed `string` and `true` for `typeof new Error("stack message").stack` and first-line `indexOf("Error: stack message") === 0`; pre-change iwasm printed `undefined` and `false`.
- Implemented a minimal stack slice for Error, TypeError, ReferenceError, and SyntaxError: constructors now store an own `.stack` string initialized to the Node-compatible first line `ConstructorName: message`.
- Fixed the runtime catalog dependency for `$concat` so direct `RuntimeFn::Concat` users include `$is_string`.
- Added `fixtures/builtins-and-io/error-stack.ts` and `error_stack_fixture_matches_node_output_under_iwasm`, covering first-line stack prefixes for all supported Error constructors.
- Direct Node vs iwasm evidence for `fixtures/builtins-and-io/error-stack.ts` matched:
  `true true true true` on separate stdout lines.
- Validation passed: `cargo check -p ts2wasm-backend-wasm -p ts2wasm-cli`; `cargo fmt --all --check`; `cargo nextest run -E 'test(error)'`; `cargo nextest run -p ts2wasm-cli error`; `node fixtures/builtins-and-io/error-stack.ts`; `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/error-stack.ts -o /tmp/ts2wasm-054-error-stack.wasm && iwasm /tmp/ts2wasm-054-error-stack.wasm`; `mise run check-issue-health`; `mise run check-agent-state`; `mise run check-repo-smoke`.
- Out-of-scope validation note: `cargo clippy -p ts2wasm-backend-wasm -p ts2wasm-cli --all-targets -- -D warnings` failed in pre-existing frontend parser warnings (`clippy::needless_bool`, `clippy::collapsible_if`) outside this assignment's allowed files.
- Remaining criteria before close: full stack trace frames and full `cargo nextest run` close validation remain outstanding.

Closure audit 2026-04-28:

- Verified Error constructors for `Error`, `TypeError`, `ReferenceError`, and `SyntaxError` through `fixtures/builtins-and-io/error-message.ts`, including no-argument and non-string message coercion cases.
- Verified Error properties through `fixtures/builtins-and-io/error-message.ts` for `.message`, `fixtures/builtins-and-io/error-stack.ts` for minimal Node-compatible `.stack` first-line prefixes, and `fixtures/builtins-and-io/error-instanceof.ts` for built-in prototype identity and subclass-to-`Error` prototype chaining.
- Direct Node/iwasm evidence matched for all Error fixtures: `error-message.ts`, `error-instanceof.ts`, and `error-stack.ts`.
- Full stack trace frames remain out of scope for this issue's stated "start with basic error types" scope; the implemented `.stack` property satisfies the basic property acceptance criterion with fixture coverage.

## Completion evidence

Commits:

- `6258e84` (`issue-054: add error message constructor slice`)
- `3c843a5` (`issue-054: coerce error message arguments`)
- `8ab8f62` (`issue-054: implement error instanceof prototypes`)
- `37482ef` (`issue-054: add minimal error stack property`)
- `893369a` (`issue-054: close error types`)

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: cargo nextest run -E 'test(error)'
result: pass; 5 tests
date: 2026-04-28

command: node fixtures/builtins-and-io/error-message.ts && cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/error-message.ts -o /tmp/ts2wasm-054-error-message.wasm && iwasm /tmp/ts2wasm-054-error-message.wasm
result: pass; Node and iwasm stdout matched for Error message fixture
date: 2026-04-28

command: node fixtures/builtins-and-io/error-instanceof.ts && cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/error-instanceof.ts -o /tmp/ts2wasm-054-error-instanceof.wasm && iwasm /tmp/ts2wasm-054-error-instanceof.wasm
result: pass; Node and iwasm stdout matched for Error instanceof fixture
date: 2026-04-28

command: node fixtures/builtins-and-io/error-stack.ts && cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/error-stack.ts -o /tmp/ts2wasm-054-error-stack.wasm && iwasm /tmp/ts2wasm-054-error-stack.wasm
result: pass; Node and iwasm stdout matched for Error stack fixture
date: 2026-04-28

command: cargo nextest run
result: pass; 303 tests passed, 4 skipped
date: 2026-04-28
```

Remaining risks:

- none
