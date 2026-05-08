---
id: 5216
title: "Allow extra arguments for functions reading arguments object"
type: bug
area: ir/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Allow supported functions that read the `arguments` object to accept extra call-site arguments so `fixtures/core-semantics/function-arguments.ts` can run under Node/iwasm differential testing.

## Problem

Problem: `fixtures/core-semantics/function-arguments.ts` now fails at `first(7)` with TS2554 even though that fixture relies on the function-local `arguments` object.

The full `cargo nextest run` gate progressed past issue 5215 and then stopped because arity validation rejected an extra argument to a zero-parameter function that reads `arguments.length` and `arguments[0]`.

## Current failure

Reproduction:

```sh
cargo nextest run -p ts2wasm-cli function_arguments_fixture_matches_node_output_under_iwasm
```

Failure excerpt:

```text
fixtures/core-semantics/function-arguments.ts
error: [ArityMismatch] TS2554: Expected 0 arguments, but got 1. at 330..338
```

## Desired final state

Supported ordinary functions that need the `arguments` object keep the real call arity available at runtime without weakening TS2554 diagnostics for ordinary user calls that do not read `arguments`.

## Scope

In scope:

- [x] Preserve strict arity diagnostics for ordinary user calls while allowing the supported `arguments` object fixture to receive extra arguments.

Out of scope:

- Broad test262 `arguments-object` triage.
- Nested closure `arguments` semantics.

## Affected paths

Expected:

- `crates/ir/src/`
- `fixtures/core-semantics/function-arguments.ts`

Do not touch:

- `crates/backend-wasm/src/runtime_arrays.rs`
- `fixtures/builtins-and-io/array-includes.ts`

## Acceptance criteria

- [x] `cargo nextest run -p ts2wasm-cli function_arguments_fixture_matches_node_output_under_iwasm` passes.
- [x] Existing TS2554 diagnostics for issue 5188 representatives do not regress.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli function_arguments_fixture_matches_node_output_under_iwasm
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
git diff --check
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli validate_rejects_arity_mismatch
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] 5217 tracks the next unrelated full-gate failure found after the function arguments fixture passed.

## Notes

Discovered while validating issue 5215.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- This commit.

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-cli function_arguments_fixture_matches_node_output_under_iwasm
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-cli validate_rejects_arity_mismatch
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-cli typescript_semantics_
result: pass
date: 2026-05-06

command: cargo nextest run
result: fail after the 5216 function-arguments failure was cleared; stopped at issue 5217 (`resolves_ambient*` count assertions)
date: 2026-05-06
```

Remaining risks:

- Full `cargo nextest run` still fails on unrelated issue 5217.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

