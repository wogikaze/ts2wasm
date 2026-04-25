---
name: milestone
description: Use when implementing vertical slices, updating shared definitions in docs/11, or deciding the next compiler/runtime work bounded by gates (historical name "milestone"; prefer workstreams/gates in new text).
---

# Vertical slice workflow

Use `docs/11-shared-definitions.md` as the canonical source for project goal、workstreams、gates、test status schema、capability manifest、and benchmark policy.

## Mise: run before you finish a slice (required)

**Run and pass the commands that match the slice; do not mark the workstream step done on red.** Without `mise`, use `scripts/manager` with the same subcommand. First time: `mise trust` ([docs](https://mise.jdx.dev/cli/trust.html)).

```bash
mise run fmt
mise run nextest
mise run clippy
mise run check-repo-smoke
```

If the slice changes reference coverage or benchmark policy expectations, also use `mise run reference-coverage` / `mise run check-coverage-gate` (see `scripts/*` and `docs/15`).

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
