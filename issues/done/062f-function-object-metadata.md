---
id: 062f
title: "Implement function object metadata"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: ["062c"]
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
---

Problem: Function object metadata such as `name`, `length`, and prototype
properties is observable but should not be mixed into callable execution support.

## Summary

Implement the smallest supported function object metadata slice after ordinary
function declarations and direct calls exist.

## Scope

In scope:

- [x] `fn.name` for supported named functions.
- [x] `fn.length` for supported fixed-arity functions.
- [x] Diagnostics or explicit follow-up issues for unsupported prototype and descriptor behavior.
- [x] Node/iwasm differential fixtures for the supported metadata subset.

Out of scope:

- Dynamic Function constructor metadata.
- `bind`, `call`, `apply`, and prototype mutation behavior.
- Property descriptor compatibility.
- Async/generator/class constructor metadata.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/` unless ABI changes are explicitly required and reviewed.

## Acceptance criteria

- [x] Supported named function `name` reads match Node.
- [x] Supported fixed-arity function `length` reads match Node.
- [x] Unsupported metadata/prototype forms remain issue-linked diagnostics or follow-up issues.
- [x] Callable execution fixtures from issue 062c continue to pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(function) or test(node_diff)'
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 94 --detail
```

Not run:

- none

## Completion evidence

Commits:

- `6448031` issue-062f: implement function object metadata

Validation result:

```text
command: cargo fmt --all --check
result: passed
date: 2026-04-29

command: cargo nextest run -E 'test(function) or test(node_diff)'
result: passed (27 tests, 403 skipped) after adding source spans to issue-062f unsupported diagnostics
date: 2026-04-29

command: cargo nextest run
result: passed (426 tests, 4 skipped)
date: 2026-04-29
```

Remaining risks:

- none

Evidence notes:

- `fixtures/core-semantics/function-object-metadata.ts` verifies `score.name`, `score.length`, `empty.name`, and `empty.length` against Node/iwasm output while preserving ordinary direct calls.
- `fixtures/core-semantics/function-prototype-metadata-unsupported.ts` verifies function prototype metadata remains out of scope with an issue-linked `issue-062f` diagnostic at a source span.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/062f-function-object-metadata.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
