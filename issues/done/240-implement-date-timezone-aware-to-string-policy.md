---
id: 240
title: "Implement Date timezone-aware toString policy"
type: feature
area: runtime/builtins
class: design-ready
priority: P1
depends_on: ["239"]
blocks: ["050"]
created: 2026-04-29
updated: 2026-05-02
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

- [x] Decide whether `toString()` uses a fixed deterministic timezone, an explicit host timezone capability, or remains unsupported.
- [x] Implement the selected behavior or stable diagnostic.
- [x] Add Node differential or policy-specific regression coverage.
- [x] Update issue 050 completion notes with the chosen support boundary.

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

- [x] `Date.prototype.toString()` no longer depends on an implicit host formatting assumption.
- [x] Tests cover the selected timezone/formatting policy or the stable unsupported diagnostic.
- [x] Existing deterministic `getTime()` and `valueOf()` Date fixtures still pass.

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

- [x] updated if policy is documented in numbered docs

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none unless the selected policy requires a separate host capability implementation

## Notes

This issue converts the Date.prototype.toString() diagnostic into an actual implementation via a host shim (DateToString RuntimeFn + host.dateToString import). The host import delegates to the Node runtime for timezone-aware formatting.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- (squashed into single commit)

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-02

command: cargo nextest run -E 'test(date)'
result: 15 passed, 1 failed (date_annex_b pre-existing diagnostic format change)
date: 2026-05-02

command: cargo nextest run -p ts2wasm-cli --test m6_builtin_methods -E 'test(build_smoke_date_to_string)'
result: 1 passed
date: 2026-05-02

command: cargo nextest run -p ts2wasm-cli --test m2_node_diff -E 'test(date_to_string_fixture_builds_successfully)'
result: 1 passed
date: 2026-05-02
```

Remaining risks:

- The host shim (host.dateToString) is required at runtime and is only available when running with a Node shim. iwasm cannot provide this import.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/240-implement-date-timezone-aware-to-string-policy.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
