---
name: issues-workflow
description: Use when adding/closing/moving/splitting/reclassifying issues under issues/. Regenerates index after any issue lifecycle change.
---

# Issues workflow

Use when adding, closing, moving, splitting, or reclassifying issues under `issues/`, or when fixing the issue index generator.

## Mise: run before you close the task (required)

**Do not** claim the issue work is done until the relevant commands below have been run and pass. If `mise` is not available, use the same subcommands via `mise` (see root `mise.toml`). On first use: `mise trust` ([docs](https://mise.jdx.dev/cli/trust.html)).

- After **any** change under `issues/open/`, `issues/done/`, or the index generator: `mise run update-issue-index` then `mise run check issue-index` and `mise run check issues`
- If you only need a quick mechanical gate: `mise run check` (fmt + `check scripts` + `check issues`)

## Rules

- Issue files in `issues/open/` and `issues/done/` are the source of truth; `issues/index.md` queue tables are generated.
- After any issue lifecycle change, run `mise run update-issue-index` and commit the updated `issues/index.md`.
- Do not hand-edit HTML comment regions between `<!-- generated:*:start -->` and `<!-- generated:*:end -->` in `issues/index.md`.
- Prefer the template at `issues/templates/issue.md`. Use `**ID**`, `**Depends on**`, `**Orchestration class**`, and a one-line `Problem:` so the index generator can summarize issues.
- **Depends on** lists open-issue IDs that block this issue, or `none`. Use comma-separated IDs (e.g. `003,004`). The generator treats an issue as blocked if any listed dependency is still open, or if **Orchestration class** is exactly `blocked` (case-insensitive).
- Closing an issue: fill completion evidence, set **Status** to `done`, move the file to `issues/done/`, then regenerate the index.
- Validation for the queue: `mise run update-issue-index -- --check` and `mise run check issue-index` (human status on stderr; exit code is the contract).

## Blocked to ready flow

`blocked` and `triage-needed` issues are not executable work orders. `blocked` is for epics, duplicates, and work that lacks a required design/policy decision. `triage-needed` is for generated reference buckets that need smart-runner evidence and duplicate review before child issues are created. Do not move an issue out of either class just because it is important.

Before changing `class: blocked` or `class: triage-needed` to another class:

1. Run `mise run check issue-readiness -- --format markdown --limit 20` and inspect the issue's missing dimensions.
2. Decide whether the blocked issue itself can become one executable slice. If not, create child issues and leave the parent blocked.
3. Make the candidate issue score at least 80 with `mise run check issue-readiness -- --fail-ready-below 80`.
4. Choose the narrowest correct non-blocked class:
   - `implementation-ready`: code changes can start immediately.
   - `verification-ready`: implementation exists; validation/review is the remaining work.
   - `docs-ready`: documentation, current-state, or issue cleanup only.
   - `design-ready`: the output is a concrete design decision or contract, not implementation.
5. Regenerate and verify the queue:

```bash
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issue-index
mise run check issues
mise run check issue-readiness -- --fail-ready-below 80
```

Do not unblock generated reference bucket issues directly. Run `mise run reference-triage -- <suite> <path>` for representative failures, record duplicate candidates, then split them first into one feature family, one observable behavior, or one fixed reference window. Do not unblock duplicate parent issues; merge or supersede them through a cleanup child.

## Anti-patterns

- Claiming the queue is empty while `issues/open/*.md` still has work items.
- Referring to `docs/current-state.md`; use `current-state.md` at the repository root (see `issues/README.md`).
- Moving a broad epic from `blocked` to `implementation-ready` without creating a small child issue.
- Treating a readiness score as priority. Priority decides order; readiness decides whether the issue is pick-up-able.

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
mise run check issues
```

### Commands run

```bash
mise run update-issue-index
mise run check issue-index
mise run check issues
```
