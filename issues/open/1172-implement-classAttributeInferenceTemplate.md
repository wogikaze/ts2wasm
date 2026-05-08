---
id: 1172
title: "Implement Classattributeinferencetemplate"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1172.

## Summary

Closed as stale build-pass.

## Problem

Problem: `classAttributeInferenceTemplate.ts` no longer has a compiler blocker;
fresh coverage and triage both show the generated type-system bucket is stale.

## Current failure

Representative path:

- `reference/typescript/tests/cases/compiler/classAttributeInferenceTemplate.ts`

Fresh coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classAttributeInferenceTemplate.ts --detail --no-dashboard-data
```

Result:

```text
executed=1
build_pass=1
unsupported=0
```

Fresh triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classAttributeInferenceTemplate.ts
```

Result:

```text
BuildPass: ts2wasm build succeeded
```

## Desired final state

No implementation issue is required for this stale generated bucket.

## Scope

In scope:

- [x] Refresh representative coverage.
- [x] Refresh representative triage.
- [x] Close the stale generated bucket.

Out of scope:

- Semantic parity beyond build coverage.

## Acceptance criteria

- [x] Coverage reports `build_pass=1`.
- [x] Triage reports `BuildPass`.
- [x] 1172 is moved to `done/`.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classAttributeInferenceTemplate.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classAttributeInferenceTemplate.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Completion evidence

Completed as stale build-pass on 2026-05-06.
