---
id: 5222
title: "Parse ambient generic variable type annotations"
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

After leading decimal numeric literals parse, `builtinIterator.ts` advances to
`declare const g1: Generator<string, number, boolean>;` and reports an
unterminated ambient variable declaration.

## Problem

The ambient variable parser still fails to treat generic type annotations as a
complete declaration-only ambient variable type.

Problem: declaration-only ambient variables with generic type annotations can
still report `issue-400: unterminated ambient variable declaration`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/builtinIterator.ts
```

Current diagnostic after issue 5191:

```text
UnsupportedTypeScriptSyntax: issue-400: unterminated ambient variable declaration at 1331..1338
```

Representative source:

```ts
declare const g1: Generator<string, number, boolean>;
const iter1 = Iterator.from(g1);
```

## Desired final state

Declaration-only ambient variables with generic type annotations are erased
cleanly, and `builtinIterator.ts` advances past this ambient declaration parser
boundary.

## Scope

In scope:

- [x] Parse or skip generic type annotation lists in declaration-only ambient variables.
- [x] Preserve rejection for ambient variable declarations with initializers.
- [x] Add focused parser coverage for `declare const g1: Generator<string, number, boolean>;`.
- [x] Re-run `builtinIterator.ts` triage and record the next narrower blocker.

Out of scope:

- Iterator helper runtime or type/value diagnostics.
- Ambient namespace/module ownership.
- Runtime variable emission for ambient declarations.

## Affected paths

Expected:

- `crates/frontend/src/parser/`

Do not touch:

- `crates/backend-wasm/src/`
- Iterator runtime/builtin implementation.

## Acceptance criteria

- [x] `declare const g1: Generator<string, number, boolean>;` parses as an erased ambient declaration.
- [x] `builtinIterator.ts` no longer reports `unterminated ambient variable declaration` at `Generator`.
- [x] Existing ambient initializer rejection coverage remains intact.
- [x] Follow-up work is represented if triage advances to iterator diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend ambient
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/builtinIterator.ts
```

Impacted commands:

```sh
python scripts/manager.py check issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] completed: `issues/open/5223-report-iterator-type-only-value-use-diagnostics.md`

## Notes

Split while closing issue 5191. Issue 5193 covered ASI after ambient variable
declarations and type literals with call/construct signatures; this issue tracks
the generic type-annotation form exposed later in `builtinIterator.ts`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit: frontend: parse ambient generic variable annotations

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-frontend ambient
result: pass; 12 tests passed
date: 2026-05-06

command: cargo build -q -p ts2wasm-cli && python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/builtinIterator.ts
result: pass for issue 5222; no ambient generic declaration parser diagnostic; advanced to generic UnresolvedName for Iterator tracked by issue 5223
date: 2026-05-06
```

Remaining risks:

- `Iterator` type-only value-use diagnostic remains open under issue 5223.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

