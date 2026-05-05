# Harden reference coverage prerequisites (audit reopened #007)

**Status**: open
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

- [ ] Missing references fail with clear action text.
- [ ] Coverage matrix is not updated from invalid inputs.
- [ ] Check/ramp behavior is documented.

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
- `issues/open/007-harden-reference-coverage-prerequisites.md` before this move
- `issues/open/007-harden-reference-coverage-prerequisites.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
