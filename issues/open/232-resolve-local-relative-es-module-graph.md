---
id: 232
title: "Resolve local relative ES module graph"
type: feature
area: compiler/frontend
class: implementation-ready
priority: P1
depends_on: [231]
blocks: [233, 234]
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Build a deterministic module graph for static local relative ES module specifiers after the parser can represent module declarations.

## Problem

The compiler currently builds a single entry file and does not resolve static ES module specifiers into source files, module IDs, or diagnostics. Import/export lowering needs a checked module graph before it can bind names safely.

## Desired final state

Given an entry file, static local relative imports and re-exports resolve to canonical source files with stable module IDs, cycle detection, and source diagnostics for missing or unsupported specifiers.

## Scope

In scope:

- [ ] Resolve `./` and `../` module specifiers relative to the importing file
- [ ] Support `.ts` and `.js` source files with deterministic extension handling documented in tests
- [ ] Build a module graph from the entrypoint without executing modules
- [ ] Detect missing files and non-local/bare specifiers with issue-linked diagnostics
- [ ] Preserve module IDs and source paths for lowering

Out of scope:

- [ ] Package resolution, `node_modules`, import maps, and TypeScript path mapping
- [ ] Dynamic import and CommonJS `require()`
- [ ] Export binding lowering or backend execution

## Affected paths

Expected:

- `crates/compiler/src/`
- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/module-system/`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`
- `docs/`

## Acceptance criteria

- [ ] Entry builds collect all reachable local relative ES modules exactly once
- [ ] Module graph ordering is deterministic and covered by tests
- [ ] Missing relative modules produce source diagnostics with the importing file/span
- [ ] Bare specifiers are rejected with an unsupported diagnostic, not silently treated as paths
- [ ] Cycles are detected and either represented safely or diagnosed explicitly

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler
cargo nextest run -p ts2wasm-cli module
```

Impacted commands:

```sh
cargo run -q -p ts2wasm-cli -- dump fixtures/module-system/static-entry.ts --ast --resolved
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

Keep path resolution local and deterministic. Do not implement Node package resolution in this issue.

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
