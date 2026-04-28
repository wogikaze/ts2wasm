---
name: milestone
description: Use for vertical slice implementation and shared definition updates. Uses docs/11 as canonical source for goals, workstreams, gates.
---

# Vertical slice workflow

Use `docs/11-shared-definitions.md` as the canonical source for project goal、workstreams、gates、test status schema、capability manifest、and benchmark policy.

## Success Criteria

A vertical slice is considered complete when:
- The smallest implementation change turns the gate condition into executable code, schema, or tests
- Docs and implementation are aligned in the same change
- All required gates (fmt, nextest, clippy, check) pass
- If reference coverage or benchmark policy changed, reference-coverage check passes
- Shared definitions in crates/shared are updated with tests
- The slice is vertical (not broad abstraction)
- current-state.md is updated if implementation state changed

## Mise: run before you finish a slice (required)

**Run and pass the commands that match the slice; do not mark the workstream step done on red.** Without `mise`, use `mise` with the same subcommand. First time: `mise trust` ([docs](https://mise.jdx.dev/cli/trust.html)).

```bash
mise run fmt
mise run nextest
mise run clippy
mise run check
```

If the slice changes reference coverage or benchmark policy expectations, also use `mise run reference-coverage` / `mise run update-coverage-matrix -- --check-gate` (see `scripts/*` and `docs/15`).

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

## Related Skills

- compatibility: for semantic compatibility changes
- gatekeeper-review: for merge gate verification
- docs-workflow: for updating shared definitions docs

## Example Usage

### Before: Implementing a vertical slice without docs alignment

```rust
// Implement feature in crates/cli
fn new_feature() { ... }
// No docs update, no shared definition change
```

### After: Vertical slice with docs alignment

```rust
// Implement feature in crates/cli
fn new_feature() { ... }
// Update docs/11-shared-definitions.md with new gate condition
// Add tests to crates/shared/
// Run all gates
mise run fmt
mise run nextest
mise run clippy
mise run check
```

### Commands run

```bash
mise run fmt
mise run nextest
mise run clippy
mise run check
```
