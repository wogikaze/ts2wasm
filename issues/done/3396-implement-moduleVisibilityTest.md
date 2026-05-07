---
id: 3396
title: "Close moduleVisibilityTest bucket to namespace qualified access owner"
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

Closed this generated import-export bucket as superseded by issue 5287 for the
remaining failing subset. Fresh coverage shows cases 3 and 4 now build, while
cases 1 and 2 stop at same-file namespace qualified value access.

## Problem

The original bucket grouped four `moduleVisibilityTest*` reference files under
`import-export` without smart-triage evidence.

Fresh prefix coverage reports:

```text
executed=4
build_pass=2
unsupported=2
unsupported_diagcodes=UnresolvedName:2
unsupported_features=name-resolution:2
```

## Current failure

`moduleVisibilityTest1.ts`:

```text
UnresolvedName: unresolved name: `M` at M.x
```

`moduleVisibilityTest2.ts`:

```text
UnresolvedName: unresolved name: `M` at M.x
```

Both files contain same-file `namespace M { ... }` declarations followed by
top-level qualified value accesses such as:

```ts
var c = new M.C();
var z = M.x;
var alpha = M.E.A;
var omega = M.exported_var;
```

Cases 3 and 4 report `BuildPass` in current smart triage.

## Desired final state

This generated bucket remains closed. The remaining namespace qualified value
access blocker is owned by
`issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`.

## Scope

Completed:

- [x] Re-ran prefix coverage for all four affected files.
- [x] Re-ran smart triage for each affected file.
- [x] Confirmed cases 1 and 2 match issue 5287's same-file namespace value
      binding scope.
- [x] Confirmed cases 3 and 4 are no longer unsupported build blockers.
- [x] Added an ownership note to issue 5287.

Out of scope:

- Direct implementation from this generated bucket.
- Semantic visibility diagnostics after namespace qualified access resolves.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/moduleVisibilityTest1.ts`
- `reference/typescript/tests/cases/compiler/moduleVisibilityTest2.ts`
- `reference/typescript/tests/cases/compiler/moduleVisibilityTest3.ts`
- `reference/typescript/tests/cases/compiler/moduleVisibilityTest4.ts`

## Acceptance criteria

- [x] Current first diagnostics are recorded for all affected paths.
- [x] Matching owner issue 5287 is identified for the remaining failing subset.
- [x] This bucket is moved to `done/`.

## Validation

Commands run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleVisibilityTest --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleVisibilityTest1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleVisibilityTest2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleVisibilityTest3.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleVisibilityTest4.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Notes

Issue 5287 already covers binding non-ambient namespace declarations so
qualified value accesses such as `M.f()` and `new M.N.C()` are resolver-visible.
The `moduleVisibilityTest1/2.ts` failures are the same resolver-visible
namespace root problem.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: prefix coverage and four focused triage commands listed above
result: pass; cases 1/2 stop at unresolved namespace M, cases 3/4 BuildPass
date: 2026-05-08
```

Remaining risks:

- TypeScript visibility diagnostics may need later issues after issue 5287
  resolves namespace qualified value access.
