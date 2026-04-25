# `.agents/`

- **`skills/*/`** — each skill is a directory with a `SKILL.md` (and optional `references/`, `references/ja`, …).
- **`state/`** — machine-readable autonomous-dev state (see `workflows/compiler_dev_fsm.md`); not a replacement for `issues/`.
- **`workflows/`** — long-form operational contracts (e.g. compiler FSM) that are too large for a single `SKILL.md` front matter.

**Entry (autonomy loop):** `workflows/compiler_dev_fsm.md` then `state/current_task.json`.

**Skill bundle:** `skills/compiler-autonomy/SKILL.md` + `skills/compiler-autonomy/references/`.
