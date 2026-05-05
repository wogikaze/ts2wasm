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
completed: 2026-04-28
---

## Summary

Add `ts2wasm dump --tir` once the typed semantic IR exists. The command should expose the typed IR as a first-class diagnostic/debugging phase rather than aliasing the current `LoweredProgram`.

## Problem

`ts2wasm dump` can show AST, resolved AST, lowered IR, and WAT, but there is no typed IR in the compiler pipeline yet. Exposing `--tir` today would mislabel the current lowered runtime IR and make debugging output semantically misleading.

## Desired final state

`ts2wasm dump --tir <input.ts>` prints the typed IR produced after type-aware semantic analysis and before runtime/WASM lowering. `ts2wasm dump --tir --unparse <input.ts>` prints a pseudo TypeScript/Wado-like source view of that typed IR.

## Scope

In scope:

- [x] Define the concrete typed IR phase to expose as `--tir`.
- [x] Add `DumpPhase::TypedIr` or equivalent CLI plumbing.
- [x] Print the typed IR in a readable structural format.
- [x] Add `--tir --unparse` pseudo-source output.
- [x] Add CLI regression tests for `--tir` and `--tir --unparse`.

Out of scope:

- Designing the whole semantic IR from scratch if issue 020 is not complete.
- Optimizer output; track that separately.

## Affected paths

Expected:

- `crates/compiler/src/dump.rs`
- `crates/cli/src/main.rs`
- `crates/cli/tests/dump_cli.rs`
- `crates/ir/src/...`

Do not touch:

- `crates/runtime-abi/`
- `fixtures/` unless a focused dump fixture is needed

## Acceptance criteria

- [x] `ts2wasm dump --tir fixtures/.../example.ts` succeeds for a supported fixture.
- [x] `ts2wasm dump --tir --unparse fixtures/.../example.ts` emits readable pseudo source.
- [x] The output is not an alias for `LoweredProgram`.
- [x] Regression tests cover both structural and unparse modes.

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

- [x] updated: `docs/13-ir-contracts.md`

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

`--tir` should remain unavailable until a typed IR phase exists. The current lowered IR remains available through `ts2wasm dump --lowered` and `ts2wasm dump --ir`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `5abe97b` issue-204: add typed IR dump

Validation result:

```text
command: cargo nextest run -p ts2wasm-cli --test dump_cli
result: PASS (5 passed)
date: 2026-04-28

command: cargo fmt --all --check
result: PASS
date: 2026-04-28

command: mise run check-agent-state
result: PASS
date: 2026-04-28

command: mise run check-issue-health
result: PASS
date: 2026-04-28

command: mise run check-repo-smoke
result: PASS
date: 2026-04-28

command: cargo nextest run
result: PASS (218 passed, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/204-add-typed-ir-dump.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
