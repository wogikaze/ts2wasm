---
id: 3392
title: "Close moduleSymbolMerging bucket to namespace-only multi-section owner"
type: maintenance
area: compiler
class: superseded
priority: P1
depends_on: [432, 5187]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import-export bucket as superseded by issue 5187.
Fresh coverage no longer reports a build blocker, but direct smart triage still
shows the same namespace-only multi-section empty-body guard already tracked by
5187.

## Problem

The original bucket grouped `moduleSymbolMerging.ts` under `import-export`
without smart-triage evidence.

The reference file has two virtual sections:

```ts
// @Filename: A.ts
namespace A { export interface I {} }

// @Filename: B.ts
///<reference path="A.ts" preserve="true" />
namespace A { ; }
namespace B {
    export function f(): A.I { return null; }
}
```

Focused coverage on the current binary reports `build_pass`, but direct triage
still reports the compiler dump path's empty multi-section body guard.

## Current failure

Focused coverage:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
```

Direct smart triage:

```text
UnsupportedSyntax: multi-section file has no module bodies
```

TypeScript oracle reports the later semantic diagnostic:

```text
TS2322: Type 'null' is not assignable to type 'I'.
```

## Desired final state

This generated bucket remains closed. The direct-triage empty-body guard is
owned by `issues/open/5187-lower-namespace-only-multi-section-files.md`.

## Scope

Completed:

- [x] Re-ran focused coverage for `moduleSymbolMerging.ts`.
- [x] Re-ran smart triage for `moduleSymbolMerging.ts`.
- [x] Confirmed the remaining direct-triage diagnostic matches issue 5187.
- [x] Added an ownership note to issue 5187.

Out of scope:

- Direct implementation from this generated bucket.
- TypeScript semantic assignability diagnostics after namespace lowering.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/moduleSymbolMerging.ts`

## Acceptance criteria

- [x] Current coverage and triage evidence is recorded.
- [x] Matching owner issue 5187 is identified.
- [x] This bucket is moved to `done/`.

## Validation

Commands run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleSymbolMerging.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleSymbolMerging.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Notes

Related precedent: `issues/open/3344-implement-moduleKeywordDeprecated.md`
uses the same closure model where coverage reports `build_pass` but direct
triage still exposes the issue-5187 empty-body guard.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: focused coverage and triage commands listed above
result: pass; coverage build_pass=1, direct triage reports issue-5187 empty-body guard
date: 2026-05-08
```

Remaining risks:

- TS2322 semantic parity may need a later issue after issue 5187 removes the
  empty-body direct-triage guard.
