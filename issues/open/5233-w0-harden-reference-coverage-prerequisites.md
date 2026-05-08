---
id: 5233
title: "W0: harden reference coverage prerequisites"
type: infra
area: coverage
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Harden the reference coverage pipeline prerequisites so mise run reference-coverage is reliable in CI and development environments. Current external dependencies (test262 checkout, TypeScript checkout, Node.js/iwasm binary paths) are not validated before the coverage run starts, leading to cryptic failures.

## Problem

docs/12-coding-standard.md S20 lists "reference coverage prerequisites hardening" as a P1 task.

Current issues:
- Missing test262/TypeScript reference checkout is not detected with a clear error
- iwasm/Node binary paths are not validated upfront
- Partial coverage runs can produce stale artifact files
- No prerequisite check before the main coverage loop

## Scope

In scope:
- Add a prerequisite validation step before the coverage loop
- Check reference test suite directories exist
- Check iwasm and Node.js binaries are reachable
- Fail fast with a clear diagnostic message if prerequisites are missing
- Add a --check-prerequisites flag that exits after validation

Out of scope:
- Changing the coverage measurement algorithm
- Adding new test suites
- Performance optimization of the coverage loop

## Affected paths

Expected:
- scripts/run/reference-coverage.py
- scripts/check/

## Acceptance criteria

- [x] mise run reference-coverage -- --check-prerequisites validates all prerequisites
- [x] Clear error message for each missing prerequisite
- [x] CI runs prerequisite check before actual coverage
- [x] No regression in normal coverage runs

## Validation

```
mise run reference-coverage -- --check-prerequisites --sample 10
```



## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
