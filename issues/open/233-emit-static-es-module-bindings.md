---
id: 233
title: "Emit static ES module bindings"
type: feature
area: ir/backend
class: implementation-ready
priority: P1
depends_on: [231, 232]
blocks: [234]
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Lower resolved ES module imports/exports into module initialization, export storage, and import reads in the existing WASM runtime model.

## Problem

The backend has CommonJS-oriented module cache helpers, but static ES module bindings are not represented through lowering and emission. Importing code needs to read exported bindings without weakening the existing runtime/capability boundaries.

## Desired final state

Named static exports are emitted into a module export object and named imports read those bindings through the resolved module graph. Modules initialize once in dependency order.

## Scope

In scope:

- [ ] Lower named exports from declarations and export lists into resolved IR
- [ ] Lower named imports to reads from resolved module exports
- [ ] Emit module initialization in dependency order using the module graph IDs
- [ ] Reuse or extend existing module cache/export runtime helpers without unconditional host imports
- [ ] Add backend/link-plan tests showing ES module helpers are included only when needed

Out of scope:

- [ ] Live binding updates beyond simple top-level values
- [ ] `export default`, namespace objects, package resolution, and dynamic import
- [ ] Broad module semantic parity claims without execution fixtures

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/compiler/src/`
- `crates/cli/tests/`
- `fixtures/module-system/`

Do not touch:

- `crates/runtime-abi/` unless a layout constant is strictly required and reviewed in this issue
- `docs/`

## Acceptance criteria

- [ ] Simple named export/import programs build to WASM
- [ ] Imported values are read from the resolved source module, not from lexical globals in the importer
- [ ] Module initialization runs once per module for repeated imports
- [ ] Runtime link plan includes module helpers only for programs that use modules
- [ ] Existing CommonJS module-cache fixtures still build

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-backend-wasm
cargo nextest run -p ts2wasm-cli module
```

Impacted commands:

```sh
cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-entry.ts -o /tmp/ts2wasm-esm.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

Follow `docs/12-coding-standard.md`: runtime helpers, imports, capabilities, and manifest behavior must be derived through the runtime/link-plan catalog, not hard-coded in the emitter.

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
