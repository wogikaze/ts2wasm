# Harden reference coverage prerequisites

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Completed**: 2026-04-26
**ID**: 007
**Type**: infra
**Area**: scripts/reference
**Priority**: P1
**Depends on**: 005
**Orchestration class**: implementation-ready

Problem: Reference coverage scripts depend on external reference repositories. If those repositories are missing, failures can look like coverage failures instead of environment setup failures.

Scope:

- [x] Detect missing reference repos early.
- [x] Print exact clone/init command hints.
- [x] Prevent denominator-zero matrix updates.
- [x] Clarify check mode vs ramp mode in README/AGENTS.
- [x] Add script syntax checks where useful.

Acceptance Criteria:

- [x] Missing references fail with clear action text.
- [x] Coverage matrix is not updated from invalid inputs.
- [x] Check/ramp behavior is documented.

Validation:

```sh
python scripts/check/shell-syntax.py
python scripts/manager.py reference-coverage test262 --limit 1
python scripts/gen/coverage-matrix.py --check
```

## Completion evidence

**Validation results:**

```text
command: python scripts/check/shell-syntax.py
result: All shell syntax checks passed
date: 2026-04-26

command: python scripts/manager.py reference-coverage test262 --limit 1
result: executed successfully (denominator=53444)
date: 2026-04-26

command: python scripts/gen/coverage-matrix.py --check
result: coverage matrix OK (up to date)
date: 2026-04-26
```

**Implementation:**
- Added `REFERENCE_REPOS` configuration dictionary to `scripts/run/reference-coverage.py` with paths and clone/init commands for test262, tsc, and tsgo
- Added `check_reference_repo()` function to detect missing reference repos early and print exact clone/init command hints
- Added denominator-zero check in `main()` to prevent invalid matrix updates when no files are found
- Added documentation for check mode (--limit 0) vs ramp mode (--limit N) in script docstring
- Added Reference Repository Setup section to script docstring with example commands

