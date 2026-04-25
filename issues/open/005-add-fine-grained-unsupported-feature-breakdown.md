# Add fine-grained unsupported feature breakdown

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 005
**Type**: infra
**Area**: scripts/coverage
**Priority**: P0
**Depends on**: 001
**Orchestration class**: implementation-ready

Problem: `UnsupportedSyntax:423` is not actionable. The project needs feature-level breakdown such as class, import, regexp literal, type annotation, destructuring, async, etc.

Scope:
- Add stable feature labels to diagnostics or derive them from structured diagnostics.
- Update reference coverage scripts to aggregate by diagnostic code and feature label.
- Include feature labels in tracking records.
- Align test262 runner and official corpus classification.

Acceptance Criteria:
- [ ] Coverage artifacts show feature breakdown, not only `UnsupportedSyntax` count.
- [ ] Top unsupported features directly suggest next implementation slices.
- [ ] `unsupported`, `blocked`, and `fail` are not conflated.
- [ ] Skip-with-reason is not counted as pass.

Validation:
```sh
cargo fmt --all --check
cargo nextest run
scripts/reference_coverage.sh test262 --limit 50
scripts/update_coverage_matrix.sh --check
```

