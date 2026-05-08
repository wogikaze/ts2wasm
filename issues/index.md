# Issues Index

This file is the human entrypoint for the issue queue.

Issue files are the source of truth for work items. The generated section below may be replaced by a script or pasted manually from a generated report.

## Summary

<!-- generated:summary:start -->
| Area | Total | Open | Resolved |
|---|---:|---:|---:|
| total | 0 | 0 | 0 |
<!-- generated:summary:end -->

## Reading rules

- Start with `Ready queue`.
- Check `Dependency graph` for implementation order and parent-child relationships between meta issues.
- Check `Blocked queue` only after ready work is exhausted.
- Do not use `done/` as current project truth.
- For docs work, verify whether the issue updates final-state docs, `current-state.md` (repo root), or follow-up issues.
- For implementation work, verify acceptance criteria and validation commands before starting.

## Dependency graph

<!-- generated:dep-graph:start -->
No meta issues found.
<!-- generated:dep-graph:end -->

## Ready queue

<!-- generated:ready:start -->
| ID | Title | Type | Area | Class | Priority | Depends on | Summary |
|---:|---|---|---|---|---|---|---|
| — | No open issues | — | — | — | — | — | Create issues from `issues/templates/issue.md` |
<!-- generated:ready:end -->

## Blocked queue

<!-- generated:blocked:start -->
| ID | Title | Type | Area | Blocker | Summary |
|---:|---|---|---|---|---|
| — | No blocked issues | — | — | — | — |
<!-- generated:blocked:end -->

## Done queue

<!-- generated:done:start -->
| ID | Title | Type | Area | Completed evidence |
|---:|---|---|---|---|
| — | No completed issues | — | — | — |
<!-- generated:done:end -->

## Index generation contract

Run `mise run update-issue-index` after adding, closing, or moving issues. CI and agents should run `mise run update-issue-index -- --check` and `mise run check-issue-health`. `mise run check-issue-index` remains a compatibility alias.

A future generator replaces only the regions between the `<!-- generated:*:start -->` / `<!-- generated:*:end -->` markers.

Do not put hand-written policy text inside generated regions.

## Manual update checklist

When adding, completing, or blocking an issue:

- [ ] issue file is in the correct directory
- [ ] frontmatter is updated
- [ ] dependencies are reflected by re-running `mise run update-issue-index`
- [ ] done issue has completion evidence
- [ ] follow-up work is represented as a separate open issue
- [ ] final-state docs do not contain future TODOs
- [ ] current implementation gaps are in `current-state.md` (repo root)
