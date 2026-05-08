---
id: 5287
title: "Bind namespace declarations for qualified value access"
type: feature
area: frontend/name-resolution
class: implementation-ready
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Bind non-ambient TypeScript namespace declarations as namespace values with
exported members so same-file qualified accesses such as `m1.fooExport()` and
`new m1.m2.c()` do not fail as unresolved top-level names.

This is the current narrow blocker from `commentsModules.ts`.

## Problem

`commentsModules.ts` tokenizes namespace declarations and parses the following
qualified expressions, but `resolve_names` cannot find the namespace identifier
`m1` after `namespace m1 { ... }`.

Problem: a same-file non-ambient namespace declaration is erased before it
creates a qualified value binding for exported members.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsModules.ts
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsModules.ts --detail --no-dashboard-data
```

Observed result:

```text
error: [UnresolvedName] unresolved name: `m1` at 795..797
coverage: build_pass=0, unsupported=1, unsupported_diagcodes=UnresolvedName:1
```

Source context:

```ts
namespace m1 {
    export function fooExport() {
        return b;
    }
    export namespace m2 {
        export class c {
        };
    }
}
m1.fooExport();
var myvar = new m1.m2.c();
```

Compiler evidence:

```text
tokens: ok through namespace declarations, exports, and qualified uses
ast: ok; outside statements include m1.fooExport() and new m1.m2.c()
resolved: UnresolvedName for m1 during resolve_names
TypeScript oracle: accepts the file and infers myvar: c
```

## Desired final state

The resolver binds a same-file non-ambient namespace declaration as a namespace
value whose exported members can be found by qualified member access, or it
advances to the next narrower namespace lowering/runtime blocker with a
source-spanned diagnostic.

## Scope

In scope:

- [x] Bind `namespace m1 { export function fooExport() {} }` so `m1.fooExport()` no longer reports `UnresolvedName` for `m1`.
- [x] Preserve nested exported namespace/class lookup far enough for `new m1.m2.c()` to reach a narrower constructor or namespace-lowering diagnostic.
- [x] Add focused coverage for same-file non-ambient namespace value access.
- [x] Re-run `commentsModules.ts` and record the next blocker if this path advances.

Out of scope:

- Ambient `declare namespace`, tracked by ambient declaration boundary work.
- Multi-section namespace-only body preservation, tracked by `issues/done/5187-lower-namespace-only-multi-section-files.md`.
- Function/namespace merging, tracked by `issues/done/5244-date-timezone-formatting-policy.md`.
- AMD/outFile emit and full declaration comment fidelity.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused namespace/name-resolution fixtures or tests

Do not touch:

- static ES module loading or package resolution
- backend namespace emit unless a focused resolver test proves a reviewed runtime shape is required

## Acceptance criteria

- [x] `commentsModules.ts` no longer reports `UnresolvedName: unresolved name: m1` at `m1.fooExport()`.
- [x] A focused regression covers `namespace M { export function f() {} } M.f();`.
- [x] A focused regression or recorded next blocker covers nested export lookup for `namespace M { export namespace N { export class C {} } } new M.N.C();`.
- [x] Any next blocker from `commentsModules.ts` is recorded in this issue or split to a follow-up if outside this scope.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(name)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsModules.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsModules.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from `issues/open/1371-implement-commentsModules.md`.

Related but not duplicates:

- `issues/done/399-define-typescript-parse-erase-emit-boundary.md` defines the
  namespace ownership contract.
- `issues/done/432-implement-import-export.md` is the broad triage parent.
- `issues/done/5187-lower-namespace-only-multi-section-files.md`,
  `issues/done/5225-w0-typed-wat-writer.md`, and
  `issues/done/5244-date-timezone-formatting-policy.md`
  cover adjacent namespace shapes, not same-file namespace value access.

## Completion evidence

Fill when implemented.

## False-done audit

**truly-done** (5287)

- Implementation commits: verified via `git log --oneline --all --grep=5287`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
