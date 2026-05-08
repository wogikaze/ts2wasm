---
id: 5451
title: "Classify number toString after typeof switch narrowing"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Classify `x.toString(radix)` as a number method call when `x` has been narrowed
by a `switch (typeof x)` case.

Split from generated bucket
`issues/open/3453-implement-narrowingByTypeofInSwitch.md`.

## Problem

Problem: `narrowingByTypeofInSwitch.ts` parses and resolves, then
lower_program reports issue-211 for `x.toString(2)` in the `case 'number'`
branch of `switch (typeof x)`.

The representative `x` starts as `number | string | L | R` and should be
number-like inside the `case 'number'` branch. The current lowering path does
not classify the narrowed receiver and falls into the generic unknown receiver
diagnostic.

Current diagnostic:

```text
UnsupportedSyntax: issue-211: unknown receiver class for method `toString` at 3599..3612
```

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingByTypeofInSwitch.ts
```

Representative source:

```ts
type L = (x: number) => string;
type R = { x: string, y: number }

function exhaustiveChecks(x: number | string | L | R): string {
    switch (typeof x) {
        case 'number': return x.toString(2);
        case 'string': return x;
        case 'function': return x(42);
        case 'object': return x.x;
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; switch typeof cases and return member call parse
resolved: ok through builtins
lower_program: issue-211 unknown receiver class for method `toString`
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

The compiler classifies the receiver as number-like in the `case 'number'`
branch and no longer reports the generic issue-211 unknown receiver diagnostic
for `x.toString(2)`. The representative path should advance to the next
`typeof` narrowing or callable-union blocker.

## Scope

In scope:

- [ ] Preserve or consult `typeof` switch narrowing facts for the `case
  'number'` branch.
- [ ] Classify `toString(radix)` on a number-narrowed receiver before generic
  issue-211 receiver lowering.
- [ ] Add focused coverage for `switch (typeof x) { case 'number': return
  x.toString(2); }`.
- [ ] Re-run the representative triage and record the next blocker.

Out of scope:

- Full TypeScript `typeof` switch exhaustiveness and all narrowing semantics.
- Number-annotated arrow parameter `toFixed()` calls, tracked by
  `issues/open/5383-classify-number-parameter-tofixed-calls.md`.
- Function-valued union calls such as `x(42)` after this blocker advances.
- Object-property access narrowing such as `x.x` after this blocker advances.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- focused CLI/reference fixtures

Do not touch:

- broad class or method receiver lowering unrelated to primitive number
  receivers
- backend/runtime ABI unless a focused fixture proves runtime `toString` support
  is the smallest implementation step

## Acceptance criteria

- [ ] `narrowingByTypeofInSwitch.ts` no longer reports `issue-211` for
  `x.toString(2)` in `case 'number'`.
- [ ] A focused fixture covers `typeof` switch number narrowing followed by
  `toString(2)`.
- [ ] Existing unsupported method-call receiver diagnostics remain
  source-spanned.
- [ ] Any later callable-union or object-property narrowing blocker is recorded
  here or split into a follow-up issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(number) or test(method) or test(switch)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingByTypeofInSwitch.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingByTypeofInSwitch.ts --detail --no-dashboard-data
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

Related but distinct:

- `issues/open/5383-classify-number-parameter-tofixed-calls.md` owns
  `toFixed()` on number-annotated arrow parameters, not `typeof` switch
  narrowing.

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
