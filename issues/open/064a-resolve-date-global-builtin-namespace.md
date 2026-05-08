---
id: 064a
title: "Resolve Date global builtin namespace"
type: feature
area: frontend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
---

Problem: Date support is blocked by resolver failures for the global `Date` binding before deterministic Date lowering can be reached consistently.

## Summary

Teach name resolution to recognize the `Date` global builtin namespace for deterministic supported Date entry points while preserving diagnostics for live-time and unsupported Date APIs.

## Scope

In scope:

- [x] Resolve `Date` as a builtin namespace for supported deterministic Date constructor/method patterns.
- [x] Preserve issue-linked diagnostics for `Date.now()`, `new Date()`, and unsupported methods.
- [x] Add focused resolver/lowering tests and one CLI fixture if needed.

Out of scope:

- Live host time imports.
- Timezone formatting.
- Annex B Date legacy method implementation.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/lowered.rs`
- `crates/cli/tests/`
- `fixtures/builtins-and-io/`

Do not touch:

- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`

## Acceptance criteria

- [x] Deterministic Date fixtures reach the intended Date lowering path instead of unresolved-name/class-constructor fallback errors.
- [x] Unsupported live-time Date fixtures still report issue-linked diagnostics.
- [x] The resolver change does not treat arbitrary unknown globals as builtins.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(date)'
cargo nextest run -p ts2wasm-ir
```

Impacted commands:

```sh
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-epoch-get-time.ts -o /tmp/ts2wasm-064a-date.wasm
iwasm /tmp/ts2wasm-064a-date.wasm
```

Not run:

- none

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `bd87c9c` issue-064a: cover Date namespace resolution

Validation result:

```text
command: cargo fmt --all --check
result: passed
date: 2026-04-29

command: cargo nextest run -E 'test(date)'
result: passed; 16 tests passed, 396 skipped
date: 2026-04-29

command: cargo nextest run -p ts2wasm-ir
result: passed; 21 tests passed
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-epoch-get-time.ts -o /tmp/ts2wasm-064a-date.wasm
result: passed
date: 2026-04-29

command: iwasm /tmp/ts2wasm-064a-date.wasm
result: passed; stdout was 0, 1, -1
date: 2026-04-29
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/064a-resolve-date-global-builtin-namespace.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
