# Expand test262 differential coverage

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 022
**Type**: feature
**Area**: tests/coverage
**Priority**: P1
**Depends on**: 005
**Orchestration class**: implementation-ready

Problem: test262 full differential operation is incomplete. Current coverage uses sample/ramp approach. docs/11 Gate D requires test262 executed count >= 100 and Gate E requires build-pass >= 50 and semantic-pass >= 20.

Scope:

- Expand test262 execution beyond sample to full coverage.
- Improve executed count to meet Gate D (>= 100).
- Improve build-pass count to meet Gate E (>= 50).
- Improve semantic-pass count to meet Gate E (>= 20).
- Update reference-coverage-matrix.md continuously.

Acceptance Criteria:

- [ ] test262 executed count >= 100 (Gate D).
- [ ] test262 build-pass count >= 50 (Gate E).
- [ ] test262 semantic-pass count >= 20 (Gate E).
- [ ] Build-pass and semantic-pass are separately tracked.
- [ ] reference-coverage-matrix.md is updated.

Validation:

```sh
cargo fmt --all --check
scripts/run/reference-coverage.sh test262
scripts/gen/coverage-matrix.sh --check
```
