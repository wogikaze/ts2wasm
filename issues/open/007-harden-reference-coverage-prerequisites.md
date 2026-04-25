# Harden reference coverage prerequisites

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
**ID**: 007
**Type**: infra
**Area**: scripts/reference
**Priority**: P1
**Depends on**: 005
**Orchestration class**: implementation-ready

Problem: Reference coverage scripts depend on external reference repositories. If those repositories are missing, failures can look like coverage failures instead of environment setup failures.

Scope:
- Detect missing reference repos early.
- Print exact clone/init command hints.
- Prevent denominator-zero matrix updates.
- Clarify check mode vs ramp mode in README/AGENTS.
- Add script syntax checks where useful.

Acceptance Criteria:
- [ ] Missing references fail with clear action text.
- [ ] Coverage matrix is not updated from invalid inputs.
- [ ] Check/ramp behavior is documented.

Validation:
```sh
scripts/check_scripts.sh
scripts/reference_coverage.sh test262 --limit 1
scripts/update_coverage_matrix.sh --check
```

