# Coding standard (agent-bound)

**Authority:** repository root `AGENTS.md` and `docs/` (especially compatibility, semantics, and testing sections).

## Non-negotiables

- `cargo nextest run` (not raw `cargo test` unless a task’s `commands` explicitly use it) for project-standard verification when running the full matrix.
- `cargo fmt --all --check` before calling work complete.
- Scope in `.agents/state/current_task.json` is the file-level contract: do not touch `forbidden_files` without a new task or a formal split.

## Edits

- Every line in a change should map to a stated acceptance item or a recorded bug fix; avoid “drive-by” refactors.
- Prefer extending existing abstractions over parallel implementations.

## Documentation

- Intentional product/source truth lives in the numbered `docs/*` that match the topic; do not rewrite `architecture` / roadmap files to “match the code you just wrote” in the same change set. Track drift in `issues/` or a dedicated docs issue.
