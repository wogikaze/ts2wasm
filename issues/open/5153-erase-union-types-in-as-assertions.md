---
id: 5153
title: "Erase union types in as assertions"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement the narrow parser erasure slice for TypeScript `as` assertions whose type annotation is a union type.

## Problem

The representative reference case contains `return 10 as number | string;`. TypeScript treats `number | string` as the erased assertion type. The current parser erases only the first type atom and leaves `| string` in expression position, producing a runtime `BitwiseOr` binary expression that later fails lowering.

Problem: `expr as A | B` is currently parsed as runtime bitwise-or instead of erasing the whole union type annotation.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseClassImprovedMismatchErrors.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] binary operator BitwiseOr not yet supported
```

Source context:

```text
return 10 as number | string;
```

Compiler evidence:

```text
AST contains `Binary { left: Number(10), op: BitwiseOr, right: Ident("string") }`.
```

TypeScript oracle evidence:

```text
TypeScript diagnostics are class member mismatch errors after parsing the assertion type.
The `| string` segment belongs to the assertion type, not to runtime expression evaluation.
```

## Desired final state

The parser consumes and erases the full union type annotation after `as`, leaving the asserted expression unchanged for lowering.

## Scope

In scope:

- [x] Extend TypeScript `as` assertion erasure to consume union type tails such as `number | string`.
- [x] Add a focused parser regression for `return 10 as number | string;`.
- [x] Re-run the representative triage and confirm it no longer reports runtime `BitwiseOr` from the assertion type.

Out of scope:

- Runtime bitwise-or operator implementation.
- Type checking for class member override compatibility.
- Full TypeScript type grammar beyond union tails needed by `as` assertions.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- backend lowering for bitwise operators
- class type-checking semantics

## Acceptance criteria

- [x] A focused parser test erases `10 as number | string` to the same expression as `10 as number`.
- [x] The representative triage no longer reports `binary operator BitwiseOr not yet supported`.
- [x] Existing `as` assertion erasure tests continue to pass.
- [x] Any later class mismatch/type-system blocker is recorded separately if outside this parser slice.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend as_assertion_union_type_erasure
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseClassImprovedMismatchErrors.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/baseClassImprovedMismatchErrors.ts --detail
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

Split from generated bucket `issues/open/1034-implement-baseClassImprovedMismatchErrors.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit: frontend: erase union type assertions

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-frontend as_assertion_union_type_erasure
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-frontend
result: pass (172 passed)
date: 2026-05-06

command: cargo build -p ts2wasm-cli
result: pass
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseClassImprovedMismatchErrors.ts
result: pass; BuildPass, AST no longer contains runtime BitwiseOr for `as number | string`
date: 2026-05-06

command: mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/baseClassImprovedMismatchErrors.ts --detail
result: pass; executed=1, build_pass=1, unsupported=0
date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

