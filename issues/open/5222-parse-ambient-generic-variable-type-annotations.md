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

- [ ] Parse or skip generic type annotation lists in declaration-only ambient variables.
- [ ] Preserve rejection for ambient variable declarations with initializers.
- [ ] Add focused parser coverage for `declare const g1: Generator<string, number, boolean>;`.
- [ ] Re-run `builtinIterator.ts` triage and record the next narrower blocker.

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

- [ ] `declare const g1: Generator<string, number, boolean>;` parses as an erased ambient declaration.
- [ ] `builtinIterator.ts` no longer reports `unterminated ambient variable declaration` at `Generator`.
- [ ] Existing ambient initializer rejection coverage remains intact.
- [ ] Follow-up work is represented if triage advances to iterator diagnostics.

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

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split while closing issue 5191. Issue 5193 covered ASI after ambient variable
declarations and type literals with call/construct signatures; this issue tracks
the generic type-annotation form exposed later in `builtinIterator.ts`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
