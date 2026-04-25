# Issues Index

This file is the human entrypoint for the issue queue.

Issue files are the source of truth for work items. The generated section below may be replaced by a script or pasted manually from a generated report.

## Reading rules

- Start with `Ready queue`.
- Check `Blocked queue` only after ready work is exhausted.
- Do not use `done/` as current project truth.
- For docs work, verify whether the issue updates final-state docs, `docs/current-state.md`, or follow-up issues.
- For implementation work, verify acceptance criteria and validation commands before starting.

## Ready queue

<!-- generated:ready:start -->
| ID | Title | Type | Area | Class | Priority | Depends on | Summary |
|---:|---|---|---|---|---|---|---|
| — | No ready issues yet | — | — | — | — | — | Create issues from `issues/templates/issue.md` |
<!-- generated:ready:end -->

## Blocked queue

<!-- generated:blocked:start -->
| ID | Title | Type | Area | Blocker | Summary |
|---:|---|---|---|---|---|
| — | No blocked issues yet | — | — | — | — |
<!-- generated:blocked:end -->

## Done queue

<!-- generated:done:start -->
| ID | Title | Type | Area | Completed evidence |
|---:|---|---|---|---|
| — | No completed issues yet | — | — | — |
<!-- generated:done:end -->

## Index generation contract

A future generator may replace only the regions between these markers:

```text
<!-- generated:ready:start -->
<!-- generated:ready:end -->

<!-- generated:blocked:start -->
<!-- generated:blocked:end -->

<!-- generated:done:start -->
<!-- generated:done:end -->
```

Do not put hand-written policy text inside generated regions.

## Manual update checklist

When adding, completing, or blocking an issue:

- [ ] issue file is in the correct directory
- [ ] frontmatter is updated
- [ ] dependencies are reflected in the ready/blocked queue
- [ ] done issue has completion evidence
- [ ] follow-up work is represented as a separate open issue
- [ ] final-state docs do not contain future TODOs
- [ ] current implementation gaps are in `docs/current-state.md`
