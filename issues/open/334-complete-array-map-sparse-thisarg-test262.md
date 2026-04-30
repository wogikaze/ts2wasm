---
id: 334
title: "Array.prototype.map completion: sparse array, thisArg, and generic call"
type: meta
area: runtime/builtins
class: ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Parent tracking issue for completing `Array.prototype.map` compatibility beyond
the dense-array named-callback slice (issue 270) and arrow/chained receiver slice
(issue 295).

This issue tracks child issues for:
- Sparse array holes handling
- Callback `thisArg` support
- Generic call semantics (`Array.prototype.map.call(...)`)

## Problem

Problem: supported dense-array map calls work, but `Array.prototype.map` still
lacks sparse array hole handling, callback `thisArg`, and generic call behavior.

## Desired final state

`Array.prototype.map` has Node-compatible behavior for sparse arrays, `thisArg`,
and generic call, with Test262-backed compatibility evidence.

## Child issues

- [ ] Issue 338: Sparse array holes handling for Array.prototype.map
- [ ] Issue 339: Callback thisArg for Array.prototype.map
- [ ] Issue 340: Generic call for Array.prototype.map (Array.prototype.map.call(...))

## Acceptance criteria

This meta issue is complete when all child issues are moved to `done/`.

## Validation

No validation commands for this meta issue. See child issues for validation.

## Docs / current-state / issue sync

Final-state docs:

- Updated when child issues complete their respective work.

Current state:

- Updated when child issues complete their respective work.

## Notes

Issue 270 is now the historical dense-array named-callback slice. Issue 295 is
the historical arrow/chained receiver slice. This meta issue tracks the remaining
work needed for full Array.prototype.map compatibility.
