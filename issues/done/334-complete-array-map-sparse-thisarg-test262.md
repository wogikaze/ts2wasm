---
id: 334
title: "Array.prototype.map completion: sparse array, thisArg, and generic call"
type: meta
area: runtime/builtins
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-05-01
completed: 2026-05-01
status: done
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

- [x] Issue 338: Sparse array holes handling for Array.prototype.map
- [x] Issue 339: Callback thisArg for Array.prototype.map
- [x] Issue 340: Generic call for Array.prototype.map (Array.prototype.map.call(...))
- [x] Issue 379: Test262 verification for Array.prototype.map callback thisArg
- [x] Issue 403: Sparse array hole representation contract (blocks issue 338)

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

## Completion evidence

Closed after child issues 338, 339, 340, 379, and 403 were all moved to
`issues/done/`.

Validation result:

```text
command: cargo nextest run -p ts2wasm-cli array_map
result: pass; 18/18
date: 2026-05-01

command: mise run reference-coverage -- test262 --path-filter reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js --detail --no-web-ui
result: pass; build_pass=1, semantic_pass=1
date: 2026-05-01
```
