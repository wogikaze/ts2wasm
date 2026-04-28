# Expand test262 differential coverage

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-28
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

- [x] test262 executed count >= 100 (Gate D).
- [ ] test262 build-pass count >= 50 (Gate E).
- [ ] test262 semantic-pass count >= 20 (Gate E).
- [x] Build-pass and semantic-pass are separately tracked.
- [x] reference-coverage-matrix.md is updated.

Progress evidence (2026-04-28):

- Ran `python scripts/manager.py reference-coverage test262 --limit 100 --detail`; result: `executed=100`, `build_pass=0`, `semantic_pass=0`, `unsupported=100`, `fail=0`, `blocked=0`, `semantic_enabled=1`.
- Updated `artifacts/coverage/results/test262.json` and regenerated `artifacts/coverage/reference-coverage-matrix.md`; the generated test262 row records denominator `53445`, executed `100`, build-pass `0`, semantic-pass `0`.
- Gate D executed-count evidence is satisfied. Gate E remains open because the first 100 sorted test262 files are all unsupported, dominated by `regexp-literal:47`, `name-resolution:37`, and `date:13`.
- Next safe ramp recommendation: keep issue 022 open and either add a harness selection mode for known runnable semantic-core test262 seeds or implement the dominant unsupported feature slices before expecting the sorted `--limit` ramp to produce build-pass and semantic-pass counts.

Validation:

```sh
cargo fmt --all --check
scripts/manager reference-coverage test262
scripts/manager update-coverage-matrix --check
```
