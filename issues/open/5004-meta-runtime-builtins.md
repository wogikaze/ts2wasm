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
updated: 2026-05-05
status: open
---

## Summary

Covers missing JavaScript runtime builtin implementations (~40 triage-needed issues + 6 blocked issues). These are test262 failures requiring builtin function/object implementations in the runtime-abi layer.

## Problem

~46 reference test262 test families fail due to missing or incomplete runtime builtin implementations including Array, String, Object, Date, JSON, RegExp, Math, and others.

Problem: runtime builtin reference buckets currently need child issue classification by builtin family and observable Node/iwasm behavior before individual implementations are selected.

## Current failure

Current failure: `mise run reference-coverage -- test262 --limit 50 --detail` reports builtin-family gaps, but this reopened meta issue lacks a concrete selection and evidence contract for remaining child work.

## Scope

In scope:

- [ ] Review runtime builtin child issues by builtin family.
- [ ] Keep Array, String, Object, Date, JSON, RegExp, Math, and host builtin implementation children under `5004`.
- [ ] Split or relink generated buckets that combine unrelated builtin families.

Out of scope:

- Parser/frontend changes
- Non-builtin runtime semantics

## Affected paths

Expected:

- `crates/runtime-abi/src/`
- `crates/backend-wasm/src/`
- `fixtures/`
- `issues/open/`

Do not touch:

- Parser-only frontend syntax issues.
- TypeScript declaration emit issues.

## Acceptance criteria

- [ ] Runtime builtin child issues are dependency-linked to `5004` only when they require builtin implementation or runtime/backend builtin integration.
- [ ] Mixed generated buckets are split or relinked to one builtin family per child issue.
- [ ] `issues/index.md` is regenerated after dependency or class edits.

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

Required commands:

```sh
mise run update-issue-index
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 50 --detail
```

Not run:

- none

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5004-meta-runtime-builtins.md` before this move
- `issues/open/5004-meta-runtime-builtins.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
