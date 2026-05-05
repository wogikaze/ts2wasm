# `.agents/`

- **`skills/<name>/SKILL.md`** — each skill is a directory with `SKILL.md` (and optional `references/`, …). Directory names are short role slugs (for example `scripts-workflow`, `docs-workflow`, `issues-workflow`, `false-done-audit`).
- **`prompts/`** — prompt templates for workflows that are not stateful skills.
  - `autonomous-parent-orchestrator.md` — parent prompt for worktree assignment, child supervision, merge review, and Discord reporting.
  - `autonomous-child-worker.md` — child prompt for scoped work inside one assigned worktree.

**Audit-only orchestrator (split):** `skills/false-done-audit/SKILL.md` plus `issue-state-sync/`, `checklist-to-issue/`, `post-wave-orchestration/` — use **only** on explicit false-done / done-queue audit triggers; not for normal implementation.
