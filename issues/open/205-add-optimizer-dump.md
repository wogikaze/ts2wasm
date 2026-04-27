---
id: 205
title: "Add optimizer dump command"
type: feature
area: cli
class: implementation-ready
priority: P2
depends_on: [020, 204]
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Add `ts2wasm dump --optimize` and `-O0..-O3` aware dump output after optimizer passes are available.

## Problem

The project documents optimization levels, but the CLI does not yet have a reusable optimizer pipeline that produces an optimized IR artifact. `ts2wasm dump --optimize` therefore cannot truthfully show optimized TIR/MIR today.

## Desired final state

`ts2wasm dump --optimize -O2 <input.ts>` prints the optimized IR for the requested optimization level. `ts2wasm dump --optimize --unparse -O2 <input.ts>` prints pseudo source for the optimized IR.

## Scope

In scope:

- [ ] Add dump option parsing for `--optimize` with `-O0`, `-O1`, `-O2`, and `-O3`.
- [ ] Route dump through the same optimizer pipeline used by `build`.
- [ ] Emit structural optimized IR output.
- [ ] Emit pseudo-source output for `--optimize --unparse`.
- [ ] Add tests that prove `-O` changes are reflected when an optimization is observable in IR.

Out of scope:

- Defining the typed IR phase; track that in issue 204.
- Adding unsafe semantic-changing optimizations.

## Affected paths

Expected:

- `crates/compiler/src/dump.rs`
- `crates/cli/src/main.rs`
- `crates/cli/tests/dump_cli.rs`
- `crates/ir/src/...`
- `docs/07-performance-and-optimization.md`

Do not touch:

- `crates/runtime-abi/` unless optimizer output changes ABI requirements

## Acceptance criteria

- [ ] `ts2wasm dump --optimize -O0 <input.ts>` succeeds.
- [ ] `ts2wasm dump --optimize -O2 <input.ts>` succeeds.
- [ ] `ts2wasm dump --optimize --unparse -O2 <input.ts>` succeeds.
- [ ] Tests assert optimized dump output is produced by real optimizer passes, not by the unoptimized lowered IR path.

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

- [ ] updated: `docs/07-performance-and-optimization.md`
- [ ] updated: `docs/13-ir-contracts.md`

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Until this issue is complete, `ts2wasm dump --optimize` and `-O0..-O3` in dump mode should fail with a clear unsupported message.

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
