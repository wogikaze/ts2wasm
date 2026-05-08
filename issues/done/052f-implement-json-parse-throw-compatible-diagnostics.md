---
id: 052f
title: "Implement JSON.parse throw-compatible diagnostics"
type: feature
area: runtime/builtins
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
---

Problem: Invalid `JSON.parse` cases are now rejected in many paths, but iwasm usually traps with `Exception: unreachable` instead of producing throw-compatible JavaScript diagnostics.

## Summary

Replace broad trap-only invalid JSON handling with a throw-compatible runtime diagnostic path for parse errors, preserving existing rejection coverage and distinguishing unsupported representation gaps from malformed JSON.

## Current failure

Existing issue 052 evidence records multiple invalid JSON fixtures where Node rejects with a JSON `SyntaxError` and iwasm rejects with `Exception: unreachable`.

## Desired final state

Malformed JSON inputs reject through the project's JavaScript error/diagnostic model in a way that can be compared against Node's `SyntaxError` behavior for supported execution paths.

## Scope

In scope:

- [x] Define the runtime behavior for `JSON.parse` syntax errors in the current compiler/runtime error model.
- [x] Convert selected invalid JSON traps to throw-compatible behavior.
- [x] Preserve unsupported traps or diagnostics for representation gaps such as non-ASCII strings until their own issues close.
- [x] Add regression coverage for invalid literals, invalid numbers, invalid strings, trailing tokens, and incomplete input.

Out of scope:

- Implementing non-integer number representation.
- Implementing UTF-16/surrogate string support.
- Full ECMAScript Error object metadata beyond the selected throw-compatible contract.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/ir/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/`
- `issues/done/052-implement-json.md`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] A representative malformed JSON fixture reports throw-compatible parse failure rather than an unclassified `unreachable` trap.
- [x] Invalid literal, leading-zero number, incomplete-number, control-character, and trailing-token cases remain rejected.
- [x] Unsupported representation gaps stay separately tracked by 052b and 052c.
- [x] Node differential or expected-rejection tests document the selected compatibility contract.

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
cargo nextest run -E 'test(json_parse_invalid)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] update `current-state.md` if the supported JSON subset changes

Follow-up issues:

- [x] update `issues/done/052-implement-json.md`

## Completion evidence

Commits:

- `f02e3ef`

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -E 'test(json_parse_invalid)'
result: pass, 5 tests run / 5 passed
date: 2026-04-29

command: cargo nextest run -E 'test(json)'
result: pass, 18 tests run / 18 passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli json
result: pass, 15 tests run / 15 passed
date: 2026-04-29

command: cargo nextest run
result: pass, 418 tests run / 418 passed / 4 skipped
date: 2026-04-29

command: mise run check issues
result: pass after restoring local gitignored reports/runs evidence paths referenced by existing issue history
date: 2026-04-29
```

Manual runtime evidence:

```text
command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-invalid-literal.ts -o /tmp/ts2wasm-052f-json-invalid-literal-after.wasm && iwasm /tmp/ts2wasm-052f-json-invalid-literal-after.wasm
result: rejects with `SyntaxError: JSON.parse invalid JSON` before `Exception: unreachable`
date: 2026-04-29
```

Remaining risks:

- Full ECMAScript Error object throw/catch metadata is still outside this selected runtime diagnostic contract.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/052f-implement-json-parse-throw-compatible-diagnostics.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
