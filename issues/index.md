# Issues Index

This file is the human entrypoint for the issue queue.

Issue files are the source of truth for work items. The generated section below may be replaced by a script or pasted manually from a generated report.

## Reading rules

- Start with `Ready queue`.
- Check `Blocked queue` only after ready work is exhausted.
- Do not use `done/` as current project truth.
- For docs work, verify whether the issue updates final-state docs, `current-state.md` (repo root), or follow-up issues.
- For implementation work, verify acceptance criteria and validation commands before starting.

## Ready queue

<!-- generated:ready:start -->
| ID | Title | Type | Area | Class | Priority | Depends on | Summary |
|---:|---|---|---|---|---|---|---|
| 002 | Emit canonical capability manifest schema | feature | abi/wasi | implementation-ready | P0 | none | docs define a canonical manifest schema with `schema_version`, `target`, `standalone`, `wasi`, `node_host`, and `capa... |
| 004 | Reclassify compile-only compatibility tests | test | tests/coverage | implementation-ready | P0 | none | Tests such as class/module/Node API compile-only checks can make compatibility look more advanced than it is. Build s... |
| 005 | Add fine-grained unsupported feature breakdown | infra | scripts/coverage | implementation-ready | P0 | none | `UnsupportedSyntax:423` is not actionable. The project needs feature-level breakdown such as class, import, regexp li... |
<!-- generated:ready:end -->

## Blocked queue

<!-- generated:blocked:start -->
| ID | Title | Type | Area | Blocker | Summary |
|---:|---|---|---|---|---|
| 003 | Verify manifest against emitted WAT imports | test | wasi/tests | 002 | A manifest is only useful as a gate if it matches emitted WAT/wasm imports. The current project n... |
| 006 | Remove stale milestone and transitional docs | cleanup | docs | 002,003 | Several docs appear to mix stale milestone notes, transitional manifest schema, and current imple... |
| 007 | Harden reference coverage prerequisites | infra | scripts/reference | 005 | Reference coverage scripts depend on external reference repositories. If those repositories are m... |
| 008 | Introduce typed WAT writer skeleton | refactor | backend | 003 | Large raw WAT string generation in runtime builder code is a major maintainability risk. A full r... |
| 009 | Select first coverage-improvement feature slice | spike | frontend/ir/runtime | 005 | After coverage breakdown exists, the next implementation should be chosen by data. The goal is to... |
| 010 | Extract frontend module from crates/cli | refactor | frontend | 003,004 | `crates/cli/src/lib.rs` appears to mix lexer, parser, AST, span, validation, and build pipeline c... |
<!-- generated:blocked:end -->

## Done queue

<!-- generated:done:start -->
| ID | Title | Type | Area | Completed evidence |
|---:|---|---|---|---|
| 000 | Short imperative title | feature | frontend | see `issues/done/000-sample-issue.md` |
| 001 | Fix issue infrastructure and current-state path references | infra | issues/docs | see `issues/done/001-fix-issue-infrastructure-and-current-state-path-references.md` |
<!-- generated:done:end -->

## Index generation contract

Run `scripts/update_issue_index.sh` after adding, closing, or moving issues. CI and agents should run `scripts/update_issue_index.sh --check` and `scripts/check_issue_index.sh`.

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
- [ ] dependencies are reflected by re-running `scripts/update_issue_index.sh`
- [ ] done issue has completion evidence
- [ ] follow-up work is represented as a separate open issue
- [ ] final-state docs do not contain future TODOs
- [ ] current implementation gaps are in `current-state.md` (repo root)
