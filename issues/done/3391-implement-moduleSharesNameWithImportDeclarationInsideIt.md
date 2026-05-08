---
id: 3391
title: "Close moduleSharesNameWithImportDeclarationInsideIt bucket after build pass"
type: maintenance
area: frontend/syntax
class: completed
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import-export bucket because all six affected reference
paths now build successfully.

## Problem

The original issue was generated from an older coverage window that classified
the `moduleSharesNameWithImportDeclarationInsideIt*` files as unsupported
`import-export` cases.

Fresh prefix coverage now reports:

```text
executed=6
build_pass=6
unsupported=0
blocked=0
```

Each focused triage reports:

```text
BuildPass: ts2wasm build succeeded
```

## Current failure

None for the current build step.

## Desired final state

This obsolete generated bucket remains closed. No successor issue is needed
unless a future semantic coverage gate reports a concrete mismatch.

## Scope

Completed:

- [x] Re-ran prefix coverage for all six affected files.
- [x] Re-ran smart triage for each affected file.
- [x] Confirmed the current build path succeeds for the full bucket.

Out of scope:

- Adding semantic parity work without a failing semantic gate.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/moduleSharesNameWithImportDeclarationInsideIt.ts`
- `reference/typescript/tests/cases/compiler/moduleSharesNameWithImportDeclarationInsideIt2.ts`
- `reference/typescript/tests/cases/compiler/moduleSharesNameWithImportDeclarationInsideIt3.ts`
- `reference/typescript/tests/cases/compiler/moduleSharesNameWithImportDeclarationInsideIt4.ts`
- `reference/typescript/tests/cases/compiler/moduleSharesNameWithImportDeclarationInsideIt5.ts`
- `reference/typescript/tests/cases/compiler/moduleSharesNameWithImportDeclarationInsideIt6.ts`

## Acceptance criteria

- [x] Prefix coverage reports all six paths as `build_pass`.
- [x] Smart triage reports `BuildPass` for each listed path.
- [x] The issue is moved to `done/`.

## Validation

Commands run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleSharesNameWithImportDeclarationInsideIt --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleSharesNameWithImportDeclarationInsideIt.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleSharesNameWithImportDeclarationInsideIt2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleSharesNameWithImportDeclarationInsideIt3.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleSharesNameWithImportDeclarationInsideIt4.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleSharesNameWithImportDeclarationInsideIt5.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleSharesNameWithImportDeclarationInsideIt6.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Notes

Some TypeScript oracle output for cases 3 and 5 reports duplicate identifier
diagnostics, but this issue was an unsupported build bucket. Semantic diagnostic
parity should be tracked only from a semantic coverage failure.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: prefix coverage and six focused triage commands listed above
result: pass; all six paths report build_pass / BuildPass
date: 2026-05-08
```

Remaining risks:

- none for build coverage.
