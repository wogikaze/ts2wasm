# `.agents/`

- **`skills/<name>/SKILL.md`** — each skill is a directory with `SKILL.md` (and optional `references/`, …). Directory names are short role slugs (for example `scripts-workflow`, `docs-workflow`, `issues-workflow`, `false-done-audit`).
- **`state/`** — machine-readable autonomous-dev state (see `workflows/compiler_dev_fsm.md`); not a replacement for `issues/`.
- **`workflows/`** — long-form operational contracts (e.g. compiler FSM) that are too large for a single `SKILL.md` front matter.
- **`prompts/`** — prompt templates for invoking specific workflows or skills.
  - `start-autonomous-loop.md` — short launcher for the multi-worktree compiler loop.
  - `autonomous-parent-orchestrator.md` — parent prompt for queue, worktree, child, merge, and refill orchestration.
  - `autonomous-child-worker.md` — child prompt for assigned issue lists inside one worktree.

**Entry (autonomy loop):** `workflows/compiler_dev_fsm.md` then `state/current_task.json`.

**Skill bundle:** `skills/compiler-autonomy/SKILL.md` + `skills/compiler-autonomy/references/`.

**Audit-only orchestrator (split):** `skills/false-done-audit/SKILL.md` plus `issue-state-sync/`, `checklist-to-issue/`, `post-wave-orchestration/` — use **only** on explicit false-done / done-queue audit triggers; not for normal implementation.
