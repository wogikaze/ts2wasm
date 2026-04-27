---
id: 060
title: "Investigate and classify unknown-unsupported diagnostic cases"
type: spike
area: frontend
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-26
---

## Summary

Investigate unknown-unsupported diagnostic cases to determine their root causes and classify them into appropriate feature issues.

## Problem

Reference test results show 48 cases with unknown-unsupported diagnostic (test262:5, tsc:20, tsgo:23). These cases are not classified into specific feature categories, making it unclear what implementation work is needed.

## Desired final state

All unknown-unsupported cases are investigated and classified into specific feature categories or diagnostic codes. Unknown-unsupported diagnostic is only used for genuinely unclassifiable cases.

## Scope

In scope:

- [ ] Investigate each unknown-unsupported case
- [ ] Determine root cause (parser, runtime, type system, etc.)
- [ ] Classify into appropriate feature categories
- [ ] Update feature-labels.sh with new categories if needed
- [ ] Create or update feature issues for classified gaps

Out of scope:

- [ ] Implementing the features (separate issues)

## Affected paths

Expected:

- `scripts/lib/feature-labels.sh`
- `issues/open/`

Do not touch:

- `crates/frontend/src/`
- `crates/cli/src/`

## Acceptance criteria

- [ ] All unknown-unsupported cases are classified
- [ ] Feature-labels.sh updated with new categories if needed
- [ ] Appropriate feature issues created or updated
- [ ] unknown-unsupported count reduced to 0 or only genuinely unclassifiable cases

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/manager reference-coverage test262 --limit 200
scripts/manager reference-coverage tsc --limit 100
scripts/manager reference-coverage tsgo --limit 50
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] Feature issues based on classification results

## Notes

This is a spike to understand the unknown cases before implementation.

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
