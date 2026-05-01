# Parallel Development Workflow

Workflow for running independent tasks in isolated git worktrees.

## Prerequisites

- Worktree helpers: `scripts/dev/git-worktree.sh`
- `.config/nextest.toml` - 8 parallel test workers

## Workflow

1. **Plan**: `mise run update-issue-index`
2. **Create worktree**: `scripts/dev/git-worktree.sh create feature-name`
   `cd _worktrees/feature-name && mise run link-reference`
3. **Develop**: `cargo check -p <crate> && cargo nextest run -p <crate>`
4. **Gate**: `mise run check architecture` then `git add -A && git commit`
5. **Integrate**: `cd /path/to/main && git pull _worktrees/feature-name master`
6. **Clean up**: `scripts/dev/git-worktree.sh remove feature-name`

## File splits enable parallelism

- runtime_core.rs 2696 -> 3 sub-files (< 600 lines each)
- statements.rs 2238 -> 3 sub-files (< 1800 lines each)
- expressions.rs 2039 -> 2 sub-files (< 1800 lines each)
- expr_emit.rs 2110 -> dir with 3 sub-modules
- resolver_extra.rs 2400 -> dir with 4 sub-modules

## CI hooks

- Pre-commit: fmt + clippy + issue health + markdownlint
- Pre-push: fast gate + architecture rules + diff smoke + webhook
