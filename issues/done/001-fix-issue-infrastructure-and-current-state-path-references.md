# Fix issue infrastructure and current-state path references

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 001
**Type**: infra
**Area**: issues/docs
**Priority**: P0
**Depends on**: none
**Orchestration class**: implementation-ready

Problem: Issue queue docs pointed at a non-existent `issues/open/` layout (now fixed), and at a `current-state` path under `docs/` even though the live status file is `current-state.md` at the repository root.

Scope:

- Add `issues/open/.gitkeep`.
- Fix stale `current-state` path references under `issues/`.
- Update issue reading rules and template references.
- Keep manually maintained policy outside generated regions.

Acceptance Criteria:

- [x] `issues/open/` exists.
- [x] `rg 'docs/current-state\\.md' issues` returns no matches (no stale path claims under `issues/`).
- [x] A new issue can be created from the template without path ambiguity.
- [x] `issues/index.md` still has a human-maintained policy section outside generated output.

Validation:

```sh
rg 'docs/current-state\.md' issues || true
find issues -maxdepth 2 -type d | sort
scripts/manager update-issue-index -- --check
scripts/manager check-issue-index
```

## Completion evidence

Commits: (record on merge)

Validation result:

```text
command: rg 'docs/current-state\.md' issues; mise run update-issue-index --check; mise run check-issue-index
result: no matches under issues/; index generator --check pass; check_issue_index pass
date: 2026-04-26
```

Remaining risks:

- none
