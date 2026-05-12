# Parallel Parent/Child Development

This workflow runs child agents in isolated git worktrees while a parent agent supervises assignment, review, merge, and Discord reporting.

It intentionally does not use `.agents/state`, `current_task.json`, `project_state.json`, or `dev-loop`.

## Layout

- Worktrees live in sibling directories: `../ts2wasm-<prefix>-<id>-<timestamp>/`
- Parent/child prompts live under `.agents/prompts/`
- Local assignment files live under ignored `reports/agents/<agent_id>/assignment.md`
- Discord run reports live under ignored `reports/runs/<run_id>/`
- Reference corpus is symlinked with `mise run link-reference`

## Parent Loop

The parent follows:

```text
SYNC -> QUEUE_SCAN -> SPLIT_OR_SELECT -> WORKTREE_ASSIGN
-> CHILD_SUPERVISE -> MERGE_REVIEW -> REPORT -> QUEUE_REFILL
```

The tracked source of truth remains `issues/`, `docs/`, and git history. Parent queue notes and child assignments are local report artifacts, not tracked state.

## Batch Worktree Creation

Create one worktree per issue file:

```bash
mise run spawn-worktrees -- \
  --base master \
  issues/I-20260512-BTAP7K.md \
  issues/I-20260512-CA5S2K.md
```

The command accepts one or more `issues/*.md` files or glob patterns. It extracts `Id:` and `Title:` headers, outputs a JSON manifest, and creates local assignment files under `reports/agents/`.

Each worktree gets:

- isolated branch from `--base`
- reference corpus symlink when available
- shared cargo `target/` through `.cargo/config.toml`
- no `.agents/state` files

## Child Launch

For each manifest entry, start a child with:

- prompt: `.agents/prompts/autonomous-child-worker.md`
- assignment: `reports/agents/<agent_id>/assignment.md`
- worktree path from the manifest

The child reports back with a `PARENT_EVENT:` line.

## Status Collection

Collect all worktree status:

```bash
mise run worktree-status -- --format json
```

Useful variants:

```bash
mise run worktree-status -- --dirty-only
mise run worktree-status -- --ahead-only --base origin/master
```

## Merge Review

The parent reviews each child branch before merge:

1. Inspect diff scope and commits.
2. Confirm validation evidence.
3. Run relevant narrow validation.
4. Run `mise run check`.
5. Merge or cherry-pick only after review passes.
6. Run `mise run issue-index` and `mise run check issues` when issues changed.

## Discord Reporting

Discord reporting is required after each parent cycle and issue-close wave:

```bash
mise run discord-report -- reports/runs/<run_id>/cycle_report.md --run-id <run_id>
```

If sending fails or no webhook is configured, save the markdown/payload under `reports/runs/<run_id>/` and report that it is deferred.

## Coverage expansion wave

The 2026-05-12 coverage expansion wave is prepared as six independent issues:

| Epic | Issue | Focus |
|---|---|---|
| 1 | `I-20260512-BTAP7K` | Builtin API coverage |
| 2 | `I-20260512-CA5S2K` | Class completion |
| 3 | `I-20260512-ASYNC3` | Async/await and Promise integration |
| 4 | `I-20260512-MD7EX4` | Import/export module system |
| 5 | `I-20260512-TSG6R2` | TypeScript erased features and tsc/tsgo ramp |
| 6 | `I-20260512-NAM3R5` | Name resolution improvements |

Spawn all six child worktrees:

```bash
mise run issue-lint
mise run issue-index
mise run spawn-worktrees -- \
  --base master \
  --prefix covexp \
  issues/I-20260512-BTAP7K.md \
  issues/I-20260512-CA5S2K.md \
  issues/I-20260512-ASYNC3.md \
  issues/I-20260512-MD7EX4.md \
  issues/I-20260512-TSG6R2.md \
  issues/I-20260512-NAM3R5.md
```

Each child reads `docs/27-coverage-expansion-epics.md` plus its issue file before implementation.

## Prerequisites

- `mise run issue-lint`
- `mise run issue-index`
- `mise run spawn-worktrees`
- `mise run worktree-status`
- `mise run discord-report`
- `mise run check`
