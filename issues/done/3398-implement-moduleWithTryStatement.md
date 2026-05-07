---
id: 3398
title: "Close moduleWithTryStatement bucket to namespace value owner"
type: maintenance
area: frontend/name-resolution
class: superseded
priority: P1
depends_on: [432, 5287]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import-export bucket as superseded by issue 5287. Fresh
triage shows the parser accepts a `try` statement inside a namespace body, then
name resolution fails because the same-file namespace root is not visible as a
value.

## Problem

The original bucket listed one `moduleWithTryStatement` reference file under
`import-export` without smart-triage evidence.

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
UnresolvedName: unresolved name: `M` at var v = M
```

Source shape:

```ts
namespace M {
  try {
  }
  catch (e) {
  }
}
var v = M;
```

Compiler evidence:

```text
tokens: ok through namespace, try/catch, and var initializer
ast: Let v = Ident M
resolved: UnresolvedName for M during resolve_names
TypeScript oracle: accepts the file and infers v as typeof M
```

## Desired final state

This generated bucket remains closed. The remaining same-file namespace value
binding blocker is owned by
`issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`.

## Scope

Completed:

- [x] Re-ran focused coverage for the affected reference file.
- [x] Re-ran smart triage for the affected reference file.
- [x] Confirmed try/catch syntax is not the current first blocker.
- [x] Confirmed the current blocker is the same namespace root value binding
      family as issue 5287.
- [x] Added an ownership note to issue 5287.

Out of scope:

- Direct implementation from this generated bucket.
- Try/catch lowering or runtime behavior inside namespace bodies.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/moduleWithTryStatement1.ts`

## Acceptance criteria

- [x] Current first diagnostic state is recorded.
- [x] Matching owner issue 5287 is identified.
- [x] This bucket is moved to `done/`.

## Validation

Commands run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleWithTryStatement1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleWithTryStatement1.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Notes

Issue 5287 already tracks binding non-ambient namespace declarations as
resolver-visible namespace values. This file reaches the same root binding
boundary before any try/catch-specific implementation work is actionable.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: focused coverage and triage listed above
result: pass; first blocker is UnresolvedName for namespace root M
date: 2026-05-08
```

Remaining risks:

- After issue 5287 lands, namespace body preservation or try/catch lowering may
  expose a later, narrower blocker.
