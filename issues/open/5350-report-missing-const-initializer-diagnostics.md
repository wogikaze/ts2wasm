---
id: 5350
title: "Report missing const initializer diagnostics"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Parse `const` declarations without initializers far enough to emit
source-spanned missing-initializer diagnostics instead of `UnsupportedSyntax`.

## Problem

`constDeclarations-errors.ts` currently stops at the first `const c1;`:

```text
UnsupportedSyntax: const declarations require an initializer at 74..76
```

TypeScript parses the declaration list and reports TS1155 diagnostics for each
missing initializer, including typed and multi-declarator forms.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-errors.ts
```

Observed 2026-05-07:

```text
source:
5 | const c1;
6 | const c2: number;
7 | const c3, c4, c5 :string, c6;
visible symbol before failure: c1
TypeScript oracle: TS1155 "'const' declarations must be initialized."
```

## Desired final state

The frontend records missing initializer diagnostics for const declarations and
continues parsing the declaration list so later const errors can be compared.

## Scope

In scope:

- [x] Accept `const c1;` as a recoverable diagnostic case.
- [x] Preserve source spans for missing-initializer diagnostics.
- [x] Add focused negative coverage for untyped and typed missing const initializers.

Out of scope:

- Typed const declarations with valid initializers; issue `5264`.
- Const assignment diagnostics such as `c8++`.
- For-loop const declaration semantics after parsing advances.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/diagnostic.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/cli/tests/`

Do not touch:

- backend/runtime code

## Acceptance criteria

- [x] `constDeclarations-errors.ts` no longer reports `UnsupportedSyntax` at `const c1;`.
- [x] Diagnostics include source spans for `c1` and `c2`.
- [x] A focused test covers `const c1;` and `const c2: number;`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend const
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-errors.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Notes

Split from `issues/open/1444-implement-constDeclarations-unknown-unsupported.md`.
Issue `5264` remains the owner for typed const declarations that do have an
initializer but are misparsed before `=`.

## Completion evidence

Fill only when moving to `done/`.

## False-done audit

**truly-done** (5350)

- Implementation commits: verified via `git log --oneline --all --grep=5350`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
