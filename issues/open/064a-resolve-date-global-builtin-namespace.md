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
---

Problem: Date support is blocked by resolver failures for the global `Date` binding before deterministic Date lowering can be reached consistently.

## Summary

Teach name resolution to recognize the `Date` global builtin namespace for deterministic supported Date entry points while preserving diagnostics for live-time and unsupported Date APIs.

## Scope

In scope:

- [ ] Resolve `Date` as a builtin namespace for supported deterministic Date constructor/method patterns.
- [ ] Preserve issue-linked diagnostics for `Date.now()`, `new Date()`, and unsupported methods.
- [ ] Add focused resolver/lowering tests and one CLI fixture if needed.

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

- [ ] Deterministic Date fixtures reach the intended Date lowering path instead of unresolved-name/class-constructor fallback errors.
- [ ] Unsupported live-time Date fixtures still report issue-linked diagnostics.
- [ ] The resolver change does not treat arbitrary unknown globals as builtins.

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

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
