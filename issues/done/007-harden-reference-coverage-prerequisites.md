# Harden reference coverage prerequisites (audit reopened #007)

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Closed**: 2026-04-26
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

- [x] Missing references fail with clear action text.
- [x] Coverage matrix is not updated from invalid inputs.
- [x] Check/ramp behavior is documented.

Validation:

```sh
mise run check-scripts
mise run reference-coverage -- test262 --limit 1
mise run update-coverage-matrix -- --check
```

Validation result:

```text
mise run check-scripts
  ❌ failed: Syntax error in scripts/dev/install-git-hooks.sh (pre-existing issue)

mise run reference-coverage -- test262 --limit 1
  ✅ failed early with clear remediation text and clone/pull command

mise run update-coverage-matrix -- --check
  ✅ OK (matrix up to date)

mise run reference-coverage -- tsc --limit 1
  ✅ failed early with clear remediation text and clone/pull command

mise run reference-coverage -- tsgo --limit 1
  ✅ failed early with clear remediation text and clone/pull command
```

Close evidence:

- 2026-04-26: Added `scripts/run/reference-coverage.py` repository/suite pre-check with actionable remediation text and shallow-checkout resume hints.
- 2026-04-26: Updated `AGENTS.md` check/ramp command guidance to manager-based invocation.
- 2026-04-26: Updated `README.md` FAQ coverage section with manager-based check/ramp workflow.
- 2026-04-26: `mise run update-coverage-matrix -- --check` passes after invalid-suite failures no longer produce denominator-zero runs.
- 2026-04-26: Environment validation (`mise run nextest`) shows pre-existing failures unrelated to this change (iwasm availability + known differential fixture expectations).

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/007-harden-reference-coverage-prerequisites.md` before this close move
- `issues/done/007-harden-reference-coverage-prerequisites.md` after this close move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Reverification evidence

Date: 2026-05-05

- `python scripts/manager.py check-scripts`: passed.
- `python scripts/manager.py reference-coverage test262 --limit 1`: failed early before compiler binary resolution with clear missing-reference remediation text and clone/pull commands for `reference/test262`.
- `python scripts/manager.py update-coverage-matrix --check`: failed because `artifacts/coverage/results/test262.json` is an older/invalid result shape without `denominator`, `executed`, coverage percentages, `status`, `evidence`, and breakdown objects. `scripts/gen/coverage-matrix.py` now refuses to update the matrix from that invalid input instead of rendering a misleading zero-denominator row.

Parent follow-up:

- The parent refreshed `artifacts/coverage/results/test262.json` through `reference-coverage test262 --limit 1` and confirmed `update-coverage-matrix --check` passes with valid result inputs.

## Completion evidence

Date: 2026-05-05

- `scripts/run/reference-coverage.py` now resolves the ts2wasm binary lazily, after suite reference prerequisites are checked, so missing references report setup guidance before compiler binary failures.
- `scripts/gen/coverage-matrix.py` validates required result fields and refuses invalid or zero-denominator coverage inputs instead of rendering misleading matrix rows.
- Check/ramp behavior was already documented in `README.md` and `AGENTS.md`; this close revalidated those acceptance claims against the current scripts.
- Validation:
  - `PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH" python3 scripts/manager.py check-scripts` passed.
  - `python3 -m py_compile scripts/run/reference-coverage.py scripts/gen/coverage-matrix.py` passed.
  - `TS2WASM_REFERENCE_ROOT=<empty-temp-dir> PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH" python3 scripts/manager.py reference-coverage test262 --limit 1` failed early with clear missing-reference clone/pull instructions and exit status 1.
  - `PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH" python3 scripts/manager.py reference-coverage test262 --limit 1` generated a valid one-case `test262.json` coverage result.
  - `PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH" python3 scripts/manager.py update-coverage-matrix` refreshed the matrix from valid inputs.
  - `PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH" python3 scripts/manager.py update-coverage-matrix --check` passed.
  - `PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH" python3 scripts/manager.py check issues` passed.
  - `PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH" python3 scripts/manager.py check` passed.
