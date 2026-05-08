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
mise run update-issue-index -- --check
mise run check-issue-health
```

## Completion evidence

Commits: (record on merge)

Validation result:

```text
command: rg 'docs/current-state\.md' issues; mise run update-issue-index -- --check; mise run check-issue-health
result: no matches under issues/; index generator --check pass; check_issue_health pass
date: 2026-04-26
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/001-fix-issue-infrastructure-and-current-state-path-references.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
