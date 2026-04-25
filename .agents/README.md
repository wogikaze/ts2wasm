# `.agents/`

- **`skills/*/`** — each skill is a directory with a `SKILL.md` (and optional `references/`, `references/ja`, …). Directory names are short role slugs (for example `scripts-workflow`, `docs-workflow`, `issues-workflow`), not repeated product prefixes.
- **`state/`** — machine-readable autonomous-dev state (see `workflows/compiler_dev_fsm.md`); not a replacement for `issues/`.
- **`workflows/`** — long-form operational contracts (e.g. compiler FSM) that are too large for a single `SKILL.md` front matter.

**Entry (autonomy loop):** `workflows/compiler_dev_fsm.md` then `state/current_task.json`.

**Skill bundle:** `skills/compiler-autonomy/SKILL.md` + `skills/compiler-autonomy/references/`.

**Audit-only orchestrator:** `skills/false-done-audit/SKILL.md` — use **only** on explicit false-done / done-queue audit triggers; not for normal implementation.
