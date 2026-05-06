---
id: 5192
title: "Support RegExp match fallback array map receivers"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: [5160]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

After plain ternary lowering, `bestChoiceType.ts` advances past the previous `ternary operator not yet supported` blocker and now stops on `y.map(...)` where `y` is initialized from a RegExp match fallback expression.

## Problem

The representative pattern is:

```ts
let x = ''.match(/ /);
let y = x || [];
let z = y.map(s => s.toLowerCase());

let y2 = x ? x : [];
let z2 = y2.map(s => s.toLowerCase());
```

Problem: locals initialized from RegExp match fallback expressions are not classified as array-like receivers for supported `.map(...)` lowering, producing `issue-211: unknown receiver class for method map`.

## Current failure

Fresh triage after issue 5160:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestChoiceType.ts
```

Current diagnostic:

```text
UnsupportedSyntax: issue-211: unknown receiver class for method `map` at 225..252
```

The TypeScript oracle reports no diagnostics for `bestChoiceType.ts`.

## Desired final state

The compiler recognizes the supported RegExp-match-or-empty-array receiver pattern and lowers the `.map(...)` calls, or reports a narrower issue-linked diagnostic if full RegExpMatchArray receiver semantics remain out of scope.

## Scope

In scope:

- [ ] Track locals initialized from `''.match(/ /) || []` as supported array-like receivers for `.map(...)`.
- [ ] Track locals initialized from `x ? x : []` where `x` is a supported RegExp match result.
- [ ] Preserve existing dense-array `.map(...)` behavior.

Out of scope:

- Full RegExpMatchArray object parity.
- General union type or TypeScript best-choice inference.
- Non-`map` array methods.

## Affected paths

Expected:

- `crates/ir/src/lowered/`
- `crates/cli/tests/ir_lowering.rs`
- `fixtures/`

Do not touch:

- TypeScript oracle scripts.

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestChoiceType.ts` no longer reports `issue-211: unknown receiver class for method \`map\``.
- [ ] A focused test or fixture covers `let y = x || []; y.map(...)` for RegExp match fallback.
- [ ] A focused test or fixture covers `let y = x ? x : []; y.map(...)` for RegExp match fallback.
- [ ] Existing array map fixtures remain passing.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli map
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestChoiceType.ts
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split while closing issue 5160 after fresh triage proved the ternary blocker had advanced to receiver classification.

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
