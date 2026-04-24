---
name: ts2wasm-milestone
description: Use when implementing ts2wasm milestones M0-M10, updating shared definitions, or deciding the next vertical slice for the TypeScript/JavaScript to WASM compiler/runtime.
---

# ts2wasm Milestone Workflow

Use `docs/11-shared-definitions.md` as the canonical milestone source. Do not use older milestone tables except for historical comparison.

## Workflow

1. Identify the current milestone and success condition.
2. Make the smallest implementation change that turns the condition into executable code, schema, or tests.
3. Keep docs and implementation aligned in the same change.
4. Prefer vertical slices over broad abstractions.
5. Run the narrowest verification command first, then the full relevant command.

## M0 Rules

M0 owns runtime ABI definitions, capability manifest definitions, and test status definitions.

- Rust definitions live in `crates/shared/`.
- Documentation source is `docs/11-shared-definitions.md`.
- Add tests for validation rules, not just constructors.
- Do not add parser, lowering, or wasm emission work to M0.

## Runner Policy

`iwasm` is installed and should be treated as a required milestone gate. Development may add faster local checks later, but milestone success should include the iwasm path when wasm execution exists.
