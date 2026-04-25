---
name: issues-workflow
description: Use when adding/closing/moving/splitting/reclassifying issues under issues/. Regenerates index after any issue lifecycle change.
---

# Issues workflow

Use when adding, closing, moving, splitting, or reclassifying issues under `issues/`, or when fixing the issue index generator.

## Mise: run before you close the task (required)

**Do not** claim the issue work is done until the relevant commands below have been run and pass. If `mise` is not available, use the same subcommands via `scripts/manager` (see root `mise.toml`). On first use: `mise trust` ([docs](https://mise.jdx.dev/cli/trust.html)).

- After **any** change under `issues/open/`, `issues/done/`, or the index generator: `mise run update-issue-index` then `mise run check-issue-index` and `mise run check-issue-queue`
- If you only need a quick mechanical gate: `mise run check-repo-smoke` (fmt + `check-scripts` + `check-issue-queue`)

## Rules

- Issue files in `issues/open/` and `issues/done/` are the source of truth; `issues/index.md` queue tables are generated.
- After any issue lifecycle change, run `scripts/update_issue_index.sh` and commit the updated `issues/index.md`.
- Do not hand-edit HTML comment regions between `<!-- generated:*:start -->` and `<!-- generated:*:end -->` in `issues/index.md`.
- Prefer the template at `issues/templates/issue.md`. Use `**ID**`, `**Depends on**`, `**Orchestration class**`, and a one-line `Problem:` so the index generator can summarize issues.
- **Depends on** lists open-issue IDs that block this issue, or `none`. Use comma-separated IDs (e.g. `003,004`). The generator treats an issue as blocked if any listed dependency is still open, or if **Orchestration class** is exactly `blocked` (case-insensitive).
- Closing an issue: fill completion evidence, set **Status** to `done`, move the file to `issues/done/`, then regenerate the index.
- Validation for the queue: `mise run update-issue-index --check` and `mise run check-issue-index` (human status on stderr; exit code is the contract).

## Anti-patterns

- Claiming the queue is empty while `issues/open/*.md` still has work items.
- Referring to `docs/current-state.md`; use `current-state.md` at the repository root (see `issues/README.md`).

## Related skills

- **False-done / done-queue audit:** `.agents/skills/false-done-audit/SKILL.md`（監査本体）· `issue-state-sync/` · `checklist-to-issue/` · `post-wave-orchestration/` — **明示の監査依頼時のみ**。通常の issue 編集では使わない。

## Example Usage

### Before: Creating a new issue manually

```markdown
---
id: 025
title: Fix memory leak
type: bug
---
```

### After: Use template and run sync

```bash
# Copy from issues/templates/issue.md
cp issues/templates/issue.md issues/open/025-fix-memory-leak.md
# Fill in required fields
# Run sync commands
mise run update-issue-index
mise run check-issue-queue
```

### Commands run

```bash
mise run update-issue-index
mise run check-issue-index
mise run check-issue-queue
```
