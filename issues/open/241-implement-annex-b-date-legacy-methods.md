---
id: 241
title: "Implement Annex B Date legacy methods"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
depends_on: []
blocks: ["050"]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement or explicitly diagnose Annex B legacy Date methods tracked from the merged Date reference issue.

Problem: issue 061 was closed as a duplicate of the Date epic, but its Annex B Date reference evidence needs a closeable child issue instead of remaining only as a note on the broad epic.

## Current failure

Issue 050 preserves affected test262 evidence for:

- `Date.prototype.getYear`
- `Date.prototype.setYear`
- `Date.prototype.toGMTString`

The preserved cases include NaN return behavior, not-a-constructor checks, receiver validation, and setYear valid/invalid date-value handling.

## Desired final state

The listed Annex B Date methods either match ECMAScript Annex B behavior in supported scope or produce stable issue-linked unsupported diagnostics with reference coverage evidence.

## Scope

In scope:

- [ ] Cover `Date.prototype.getYear` Annex B behavior or diagnostics.
- [ ] Cover `Date.prototype.setYear` Annex B behavior or diagnostics.
- [ ] Cover `Date.prototype.toGMTString` Annex B behavior or diagnostics.
- [ ] Preserve deterministic Date epoch behavior for `getTime()` and `valueOf()`.

Out of scope:

- Live host time support for `new Date()` or `Date.now()`.
- Timezone-aware `Date.prototype.toString()` policy.
- Full Date API implementation beyond the listed Annex B methods.

## Affected paths

Expected:

- `crates/`
- `fixtures/`
- `issues/open/050-implement-date.md`
- `current-state.md`

Do not touch:

- unrelated parser syntax work

## Acceptance criteria

- [ ] The affected Annex B Date test262 cases from issue 050/061 no longer rely on the broad Date epic as their only tracking target.
- [ ] Supported behavior has Node/reference differential evidence, or unsupported behavior has issue-linked diagnostics.
- [ ] Deterministic Date subset tests still pass.

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
mise run reference-coverage -- test262 --limit 500 --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected unless Annex B policy is documented

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none unless implementation reveals a separate Date object-model dependency

## Notes

The historical issue 061 diagnostic work used issue-061 wording. New diagnostics should point at this issue or another active child issue, not the closed duplicate.

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
