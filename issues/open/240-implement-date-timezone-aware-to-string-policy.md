---
id: 240
title: "Implement Date timezone-aware toString policy"
type: feature
area: runtime/builtins
class: blocked
priority: P1
depends_on: ["239"]
blocks: ["050"]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement or explicitly policy-gate `Date.prototype.toString()` for Date values after the host timezone/formatting boundary is decided.

Problem: deterministic Date values support epoch-millisecond extraction, but `Date.prototype.toString()` currently reports an issue-050 diagnostic because timezone and host formatting policy are undefined.

## Current failure

`issues/open/050-implement-date.md` records this diagnostic:

```text
error: [UnsupportedSyntax] issue-050: Date.prototype.toString() requires timezone/host formatting policy; use getTime() or valueOf() for deterministic epoch milliseconds
```

## Desired final state

`Date.prototype.toString()` has a documented support policy and tests that either prove supported output under the selected timezone/host model or prove stable issue-linked rejection.

## Scope

In scope:

- [ ] Decide whether `toString()` uses a fixed deterministic timezone, an explicit host timezone capability, or remains unsupported.
- [ ] Implement the selected behavior or stable diagnostic.
- [ ] Add Node differential or policy-specific regression coverage.
- [ ] Update issue 050 completion notes with the chosen support boundary.

Out of scope:

- Implementing live host time for `new Date()` or `Date.now()`.
- Implementing Annex B legacy Date methods.
- Implementing full Intl/date formatting APIs.

## Affected paths

Expected:

- `crates/`
- `fixtures/`
- `issues/open/050-implement-date.md`
- `current-state.md`

Do not touch:

- unrelated runtime builtins

## Acceptance criteria

- [ ] `Date.prototype.toString()` no longer depends on an implicit host formatting assumption.
- [ ] Tests cover the selected timezone/formatting policy or the stable unsupported diagnostic.
- [ ] Existing deterministic `getTime()` and `valueOf()` Date fixtures still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(date)'
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated if policy is documented in numbered docs

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none unless the selected policy requires a separate host capability implementation

## Notes

This issue is blocked on issue 239 because formatting policy must be explicit before implementation.

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
