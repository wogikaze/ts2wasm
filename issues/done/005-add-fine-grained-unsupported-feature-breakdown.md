# Add fine-grained unsupported feature breakdown

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 005
**Type**: infra
**Area**: scripts/coverage
**Priority**: P0
**Depends on**: none
**Orchestration class**: implementation-ready

Problem: `UnsupportedSyntax:423` is not actionable. The project needs feature-level breakdown such as class, import, regexp literal, type annotation, destructuring, async, etc.

Scope:

- Add stable feature labels to diagnostics or derive them from structured diagnostics.
- Update reference coverage scripts to aggregate by diagnostic code and feature label.
- Include feature labels in tracking records.
- Align test262 runner and official corpus classification.

Acceptance Criteria:

- [x] Coverage artifacts show feature breakdown, not only `UnsupportedSyntax` count.
- [x] Top unsupported features directly suggest next implementation slices.
- [x] `unsupported`, `blocked`, and `fail` are not conflated.
- [x] Skip-with-reason is not counted as pass.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- test262 --limit 50
mise run update-coverage-matrix -- --check
```

## Completion evidence

Implemented stable feature-label classification for reference coverage and TestRecord tracking.
`artifacts/coverage/reference-coverage-matrix.md` now includes both diagnostic-code and feature breakdown columns.

Validation result:

```text
cargo fmt --all --check: pass
cargo nextest run: pass (185 passed, 4 skipped)
mise run reference-coverage -- test262 --limit 50: pass
  unsupported=50, blocked=0, fail=0, skip_with_reason=0
  unsupported_diagcodes=UnsupportedSyntax:37,UnresolvedName:8,UnresolvedFunction:5
  unsupported_features=regexp-literal:18,date:17,name-resolution:8,function-resolution:5,function:1,property-access:1
mise run update-coverage-matrix -- --check: pass
mise run check-fast-gate -- --skip-nextest: pass
```

Remaining risks:

- Feature labels are heuristic labels derived from diagnostics and reference paths; they are stable enough for triage but not a typed compiler diagnostic contract yet.
