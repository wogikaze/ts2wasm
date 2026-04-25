# Introduce typed WAT writer skeleton

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 008
**Type**: refactor
**Area**: backend
**Priority**: P1
**Depends on**: 003
**Orchestration class**: design-ready

Problem: Large raw WAT string generation in runtime builder code is a major maintainability risk. A full rewrite is too large, but new WAT generation should stop adding unstructured string concatenation.

Scope:
- Design minimal typed WAT writer API.
- Cover imports, functions, globals, and data segments.
- Keep existing output behavior unchanged.
- Convert one small helper or import path first.
- Add structural or snapshot tests.

Acceptance Criteria:
- [ ] There is at least one non-string-concatenation WAT generation path.
- [ ] Existing behavior remains unchanged.
- [ ] Coding standard says new WAT should prefer typed writer APIs.
- [ ] No broad runtime_builder rewrite is attempted in this issue.

Validation:
```sh
cargo fmt --all --check
cargo nextest run
```

