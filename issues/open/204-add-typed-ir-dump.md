---
id: 204
title: "Add typed IR dump command"
type: feature
area: cli
class: implementation-ready
priority: P2
depends_on: [020]
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Add `ts2wasm dump --tir` once the typed semantic IR exists. The command should expose the typed IR as a first-class diagnostic/debugging phase rather than aliasing the current `LoweredProgram`.

## Problem

`ts2wasm dump` can show AST, resolved AST, lowered IR, and WAT, but there is no typed IR in the compiler pipeline yet. Exposing `--tir` today would mislabel the current lowered runtime IR and make debugging output semantically misleading.

## Desired final state

`ts2wasm dump --tir <input.ts>` prints the typed IR produced after type-aware semantic analysis and before runtime/WASM lowering. `ts2wasm dump --tir --unparse <input.ts>` prints a pseudo TypeScript/Wado-like source view of that typed IR.

## Scope

In scope:

- [ ] Define the concrete typed IR phase to expose as `--tir`.
- [ ] Add `DumpPhase::TypedIr` or equivalent CLI plumbing.
- [ ] Print the typed IR in a readable structural format.
- [ ] Add `--tir --unparse` pseudo-source output.
- [ ] Add CLI regression tests for `--tir` and `--tir --unparse`.

Out of scope:

- Designing the whole semantic IR from scratch if issue 020 is not complete.
- Optimizer output; track that separately.

## Affected paths

Expected:

- `crates/cli/src/dump.rs`
- `crates/cli/src/main.rs`
- `crates/cli/tests/dump_cli.rs`
- `crates/ir/src/...`

Do not touch:

- `crates/runtime-abi/`
- `fixtures/` unless a focused dump fixture is needed

## Acceptance criteria

- [ ] `ts2wasm dump --tir fixtures/.../example.ts` succeeds for a supported fixture.
- [ ] `ts2wasm dump --tir --unparse fixtures/.../example.ts` emits readable pseudo source.
- [ ] The output is not an alias for `LoweredProgram`.
- [ ] Regression tests cover both structural and unparse modes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli --test dump_cli
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/13-ir-contracts.md`

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

`--tir` should remain unavailable until a typed IR phase exists. The current lowered IR remains available through `ts2wasm dump --lowered` and `ts2wasm dump --ir`.

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
