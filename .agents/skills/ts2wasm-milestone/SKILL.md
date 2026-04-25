---
name: ts2wasm-milestone
description: Use when implementing ts2wasm vertical slices, updating shared definitions in docs/11, or deciding the next compiler/runtime work bounded by gates (historical name "milestone"; prefer workstreams/gates in new text).
---

# ts2wasm Vertical Slice Workflow

Use `docs/11-shared-definitions.md` as the canonical source for project goal、workstreams、gates、test status schema、capability manifest、and benchmark policy.

## Workflow

1. Identify the current gate / workstream slice from `docs/11` and `current-state.md`.
2. Make the smallest implementation change that turns the condition into executable code, schema, or tests.
3. Keep docs and implementation aligned in the same change.
4. Prefer vertical slices over broad abstractions.
5. Run the narrowest verification command first, then the full relevant command.

## Shared definitions (`crates/shared`) rules

Shared schema and ABI definitions live next to documentation in `docs/11`.

- Rust definitions live in `crates/shared/`.
- Documentation source is `docs/11-shared-definitions.md`.
- Add tests for validation rules, not just constructors.
- Do not fold unrelated parser/lowering/emission work into shared-definition-only changes.

## Runner policy

`iwasm` is installed and should be treated as a required execution gate when wasm output is in scope. Development may add faster local checks later, but merge readiness should include the `iwasm` path when runtime behavior is claimed.
