# Cycle Report: Issue 007 - Harden reference coverage prerequisites

**Date**: 2026-04-26
**Issue**: 007 - Harden reference coverage prerequisites
**Status**: Completed

## Summary

Successfully hardened reference coverage prerequisites by adding early detection of missing reference repositories, printing exact clone/init command hints, preventing denominator-zero matrix updates, and documenting check/ramp behavior.

## Implementation Details

### Changes Made

1. **Reference Repository Configuration** (`scripts/run/reference-coverage.py`):
   - Added `REFERENCE_REPOS` configuration dictionary with paths and clone/init commands for test262, tsc, and tsgo
   - Each entry includes the repository path, clone command, and init command

2. **Early Detection** (`scripts/run/reference-coverage.py`):
   - Added `check_reference_repo()` function to detect missing reference repos early
   - Prints helpful error messages with exact clone/init commands when a repository is missing
   - Called at the start of `main()` before any file operations

3. **Denominator-Zero Prevention** (`scripts/run/reference-coverage.py`):
   - Added check for `denominator == 0` after finding files
   - Prints detailed error message explaining possible causes (not initialized, structure changed, incorrect suite name)
   - Exits with error code to prevent invalid matrix updates

4. **Documentation** (`scripts/run/reference-coverage.py`):
   - Added "Modes" section to docstring explaining check mode (--limit 0) vs ramp mode (--limit N)
   - Added "Reference Repository Setup" section with example commands
   - Clarified that the script requires reference repositories to be cloned and initialized

5. **Import Addition**:
   - Added `import shutil` to support future shell command checks

## Validation Results

### Shell Syntax Check

```bash
python scripts/check/shell-syntax.py
```

Result: All shell syntax checks passed

### Reference Coverage Test

```bash
python scripts/manager.py reference-coverage test262 --limit 1
```

Result: Executed successfully (denominator=53444, executed=1)

### Coverage Matrix Check

```bash
python scripts/gen/coverage-matrix.py --check
```

Result: Coverage matrix OK (up to date)

## Acceptance Criteria Evidence

- **Missing references fail with clear action text**: The `check_reference_repo()` function prints exact clone/init commands when a repository is missing
- **Coverage matrix is not updated from invalid inputs**: The denominator-zero check exits with error before any matrix update can occur
- **Check/ramp behavior is documented**: The script docstring now includes a "Modes" section explaining the difference between check mode and ramp mode

## Follow-up Work

None identified. The reference coverage prerequisites are now hardened with clear error messages and early detection.

## Files Modified

- `scripts/run/reference-coverage.py`
- `issues/done/007-harden-reference-coverage-prerequisites.md`
- `issues/index.md`
- `.agents/state/current_task.json`
