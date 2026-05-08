---
id: 3402
title: "Split multiExtendsSplitInterfaces bucket to DOM self global issue"
type: maintenance
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5005, 5429]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated name-resolution bucket by splitting the current failure
to issue 5429. Fresh triage shows the affected file no longer contains
interface inheritance; it is a two-line DOM global reference that fails on
unresolved `self`.

## Problem

The original bucket listed one `multiExtendsSplitInterfaces` reference file
under `name-resolution` without smart-triage evidence.

Fresh focused coverage reports:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

## Current failure

Smart triage reports:

```text
UnresolvedName: unresolved name: `self` at self.cancelAnimationFrame(0)
```

Source:

```ts
// @target: es2015
self.cancelAnimationFrame(0);
```

Compiler evidence:

```text
tokens: ok through self.cancelAnimationFrame(0)
ast: Call(Member(Ident self, cancelAnimationFrame), Number 0)
resolved: UnresolvedName for self during resolve_names
TypeScript oracle: accepts the file with diagnostics=[]
```

## Desired final state

This generated bucket remains closed. The DOM `self.cancelAnimationFrame`
global binding or precise unsupported-DOM diagnostic is owned by
`issues/open/5429-bind-dom-self-cancelanimationframe-global.md`.

## Scope

Completed:

- [x] Re-ran focused coverage for the affected reference file.
- [x] Re-ran smart triage for the affected reference file.
- [x] Confirmed the file is not currently an interface-inheritance blocker.
- [x] Created focused implementation-ready issue 5429.

Out of scope:

- Direct implementation from this generated bucket.
- Interface inheritance diagnostics.
- Broad DOM lib declaration modeling.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/multiExtendsSplitInterfaces1.ts`

## Acceptance criteria

- [x] Current first diagnostic state is recorded.
- [x] Focused child issue 5429 owns the current blocker.
- [x] This bucket is moved to `done/`.

## Validation

Commands run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiExtendsSplitInterfaces1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiExtendsSplitInterfaces1.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Notes

Issue 5386 covers DOM `setTimeout`, but its out-of-scope section explicitly
excludes unrelated DOM APIs. This bucket therefore creates issue 5429 for the
separate animation-frame global shape.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: focused coverage and triage listed above
result: pass; first blocker is UnresolvedName for DOM self global
date: 2026-05-08
```

Remaining risks:

- After issue 5429 lands, the reference may expose a narrower DOM runtime
  boundary.
