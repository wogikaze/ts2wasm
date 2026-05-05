---
id: 5151
title: "Parse ASI after multiline const initializer"
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

Implement the narrow parser ASI slice for a `const` declaration whose multi-line initializer ends before a newline expression statement.

## Problem

The representative TypeScript reference case has a `const result = canYouInferThis(() => ({ ... }))` declaration without an explicit semicolon, followed after a blank line by `result.BLAH;`. TypeScript accepts this through automatic semicolon insertion. The current parser instead expects an explicit semicolon and fails at the next `result` identifier.

Problem: semicolonless `const` declarations with multi-line call/object initializers currently fail before the next expression statement.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badInferenceLowerPriorityThanGoodInference.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("result")) at 252..258
```

Source context:

```text
const result = canYouInferThis(() => ({
    a: { BLAH: 33 },
    b: x => { }
}))

result.BLAH;
```

TypeScript oracle evidence:

```text
TypeScript diagnostics: none.
Top-level AST includes a `FirstStatement` for the const declaration followed by an `ExpressionStatement` for `result.BLAH;`.
```

## Desired final state

The parser accepts automatic semicolon insertion after a completed variable declaration initializer when the next token starts on a later line and can begin a new statement.

## Scope

In scope:

- [ ] Accept ASI after `const name = <expression>` when the initializer is complete and the next token is on a later line.
- [ ] Add a focused parser regression for a multi-line call returning an object literal followed by a property access statement.
- [ ] Re-run the representative triage and confirm it no longer reports `expected Semicolon, got Some(Ident("result"))`.

Out of scope:

- Broad ASI policy beyond variable declarations.
- TypeScript generic inference semantics in the reference case.
- Runtime support for the later `goofus` calls if they expose a separate blocker.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- type inference or type checker behavior
- runtime lowering for object methods

## Acceptance criteria

- [ ] A focused parser test accepts `const result = call(() => ({ value: 1 }))` followed by a newline property access statement.
- [ ] The representative triage no longer reports the semicolon expectation at `result.BLAH`.
- [ ] Existing explicit-semicolon variable declaration tests continue to pass.
- [ ] Any next blocker in `badInferenceLowerPriorityThanGoodInference.ts` is recorded separately if outside this ASI slice.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend asi_after_multiline_const_initializer
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badInferenceLowerPriorityThanGoodInference.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badInferenceLowerPriorityThanGoodInference.ts --detail
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

Split from generated bucket `issues/done/1029-implement-badInferenceLowerPriorityThanGoodInference.md`.

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
