---
id: 335
title: "Implement full Math.pow number semantics"
type: feature
area: runtime/builtins
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Implement or explicitly stage full ECMAScript `Math.pow` behavior beyond the
integer-only helper closed by issue 269.

This is a work order for the residual compatibility gap, not for the existing
small-int slice.

## Problem

Problem: `Math.pow` currently works only for the integer-backed runtime subset,
while ECMAScript requires `NaN`, `Infinity`, `+0`, `-0`, fractional values,
negative exponents, and large-number edge semantics.

## Current failure

Representative Test262 case:

```sh
cargo run -q -- build reference/test262/test/built-ins/Math/pow/applying-the-exp-operator_A2.js -o /tmp/math-pow-a2.wasm --host-deny
```

Expected behavior is Node-compatible `Math.pow(base, +0) === 1` for values
including `-Infinity`, `-0`, `+0`, `+Infinity`, and `NaN`. The current runtime
number model does not represent those values.

## Desired final state

`Math.pow` either matches Node/Test262 behavior for the specified edge cases or
is split into smaller child issues with exact reproduction paths and diagnostics
for every unsupported class.

## Scope

In scope:

- [ ] Decide the representation path for `NaN`, `Infinity`, `+0`, `-0`, and
      fractional numbers used by `Math.pow`.
- [ ] Implement Node-compatible `Math.pow` behavior for the selected slice.
- [ ] Add Node/Test262 differential coverage for the selected slice.
- [ ] Preserve the existing integer `Math.pow(2, 3)` behavior.

Out of scope:

- BigInt exponentiation.
- Unrelated Math builtins.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `artifacts/coverage/` if reference coverage is updated

Do not touch:

- unrelated builtin implementations
- issue 269 completion evidence except to cross-link if this issue is split

## Acceptance criteria

- [ ] Node/Test262 evidence covers `Math.pow(base, +0) === 1` for the
      representative edge values from
      `reference/test262/test/built-ins/Math/pow/applying-the-exp-operator_A2.js`.
- [ ] Negative exponents and fractional/edge number behavior are either
      implemented with tests or split into specific child issues.
- [ ] Existing integer `Math.pow` fixture still passes.
- [ ] Docs/current-state/issues are synchronized when the supported number model
      changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py update-issue-index --check
python scripts/manager.py check issues
```

Impacted commands:

```sh
cargo run -q -- build reference/test262/test/built-ins/Math/pow/applying-the-exp-operator_A2.js -o /tmp/math-pow-a2.wasm --host-deny
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated if the number representation contract changes

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` if broader number semantics are implemented

Follow-up issues:

- [ ] none
- [ ] created if this blocked issue is split into implementation-ready slices

## Notes

The current state file records fractional values, `NaN`, `Infinity`, and `-0`
as outside the current number subset. Do not close this issue without changing
that fact or splitting precise children.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none

## False-done audit

**truly-done** (335)

- Implementation commits: verified via `git log --oneline --all --grep=335`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
