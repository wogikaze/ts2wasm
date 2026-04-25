# ts2wasm Issues Workflow

Use when adding, closing, moving, splitting, or reclassifying issues under `issues/`, or when fixing the issue index generator.

## Rules

- Issue files in `issues/open/` and `issues/done/` are the source of truth; `issues/index.md` queue tables are generated.
- After any issue lifecycle change, run `scripts/update_issue_index.sh` and commit the updated `issues/index.md`.
- Do not hand-edit HTML comment regions between `<!-- generated:*:start -->` and `<!-- generated:*:end -->` in `issues/index.md`.
- Prefer the template at `issues/templates/issue.md`. Use `**ID**`, `**Depends on**`, `**Orchestration class**`, and a one-line `Problem:` so the index generator can summarize issues.
- **Depends on** lists open-issue IDs that block this issue, or `none`. Use comma-separated IDs (e.g. `003,004`). The generator treats an issue as blocked if any listed dependency is still open, or if **Orchestration class** is exactly `blocked` (case-insensitive).
- Closing an issue: fill completion evidence, set **Status** to `done`, move the file to `issues/done/`, then regenerate the index.
- Validation for the queue: `scripts/update_issue_index.sh --check` and `scripts/check_issue_index.sh` (human status on stderr; exit code is the contract).

## Anti-patterns

- Claiming the queue is empty while `issues/open/*.md` still has work items.
- Referring to `docs/current-state.md`; use `current-state.md` at the repository root (see `issues/README.md`).
