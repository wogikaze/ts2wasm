# Fix issue infrastructure and current-state path references

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 001
**Type**: infra
**Area**: issues/docs
**Priority**: P0
**Depends on**: none
**Orchestration class**: implementation-ready

Problem: `issues/index.md` assumes `issues/open/`, but the directory is absent. `issues/README.md` and `issues/templates/issue.md` refer to `docs/current-state.md`, while the real file appears to be root `current-state.md`.

Scope:

- Add `issues/open/.gitkeep`.
- Fix `docs/current-state.md` references to `current-state.md`.
- Update issue reading rules and template references.
- Keep manually maintained policy outside generated regions.

Acceptance Criteria:

- [ ] `issues/open/` exists.
- [ ] `grep -R "docs/current-state.md" issues` returns no stale references.
- [ ] A new issue can be created from the template without path ambiguity.
- [ ] `issues/index.md` still has a human-maintained policy section outside generated output.

Validation:

```sh
grep -R "docs/current-state.md" issues
find issues -maxdepth 2 -type d | sort
```
