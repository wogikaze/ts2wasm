---
id: 1173
title: "Implement Classattributeinferencetemplatejs"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: [5247]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Closed after splitting the current JS/noEmit invariant blocker to
`issues/open/5247-fix-js-noemit-class-constructor-funcid-invariant.md`.

## Problem

Problem: `classAttributeInferenceTemplateJS.ts` is not a broad type-system
bucket. Fresh triage shows a focused lowered-IR invariant for JS/noEmit class
constructors.

## Current failure

Fresh coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classAttributeInferenceTemplateJS.ts --detail --no-dashboard-data
```

Result:

```text
executed=1
build_pass=1
unsupported=0
```

Fresh triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classAttributeInferenceTemplateJS.ts
```

Result:

```text
InvariantViolation: ClassDecl constructor FuncId 0 is out of range (program has 0 function(s))
```

The parser/resolver/WAT dumps are present; the triage failure is the lowered-IR
invariant exposed by the JS/noEmit path, not type inference.

## Desired final state

The implementation work is tracked by issue 5247. This generated bucket should
not be implemented directly.

## Scope

In scope:

- [x] Refresh representative coverage and triage.
- [x] Split the focused invariant blocker to issue 5247.
- [x] Close the stale generated bucket.

Out of scope:

- Type-system inference implementation.
- Fixing the invariant in this cleanup issue.

## Acceptance criteria

- [x] Fresh triage identifies the exact `ClassDecl constructor FuncId` invariant.
- [x] Issue 5247 owns the focused implementation.
- [x] 1173 is moved to `done/`.

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classAttributeInferenceTemplateJS.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classAttributeInferenceTemplateJS.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Completion evidence

Completed by split to issue 5247 on 2026-05-06.
