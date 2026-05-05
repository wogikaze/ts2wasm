---
id: 5004
title: "Meta: Runtime Builtins Coverage (test262) (audit reopened #5004)"
type: meta
area: runtime/builtins
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-02
updated: 2026-05-05status: open
---

## Summary

Covers missing JavaScript runtime builtin implementations (~40 triage-needed issues + 6 blocked issues). These are test262 failures requiring builtin function/object implementations in the runtime-abi layer.

## Problem

~46 reference test262 test families fail due to missing or incomplete runtime builtin implementations including Array, String, Object, Date, JSON, RegExp, Math, and others.

## Scope

In scope:

- Builtin JavaScript object and function implementations
- Runtime-abi and backend integration for builtins
- Individual child issues for each builtin family

Out of scope:

- Parser/frontend changes
- Non-builtin runtime semantics

## Affected paths

Expected:

- `crates/runtime-abi/src/`

## Acceptance criteria

- [ ] Builtin implementation coverage increases per child issue resolution

## Progress

| Wave | Builtin | Status |
|------|---------|--------|
| 1 | String.prototype.includes | Done (fixture + node diff test passing) |
| 1 | String.prototype.padStart | Done (fixture + node diff test passing) |
| 1 | String.prototype.padEnd | Done (fixture + node diff test passing) |
| 1 | String.prototype.repeat | Done (fixture + node diff test passing) |
| 1 | String.prototype.charAt | Done (fixture + node diff test passing) |
| 1 | String.prototype.indexOf | Done (fixture + node diff test passing) |
| 1 | String.prototype.split | Done (fixture + node diff test passing) |
| 1 | String.prototype.substring | Done (fixture + node diff test passing) |
| 1 | String.prototype.charCodeAt | Done (fixture + node diff test passing) |
| 1 | String.fromCharCode | Done (fixture + node diff test passing) |

## Validation

```sh
mise run reference-coverage -- test262 --limit 50 --detail
```

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5004-meta-runtime-builtins.md` before this move
- `issues/open/5004-meta-runtime-builtins.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
