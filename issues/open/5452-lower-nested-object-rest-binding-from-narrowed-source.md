---
id: 5452
title: "Lower nested object rest binding from narrowed source"
type: feature
area: ir/lowering
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Support the next issue-251 destructuring slice: nested object rest binding from
a non-literal source whose discriminant branch narrows the source shape.

Split from generated bucket
`issues/open/3455-implement-narrowingDestructuring.md`.

## Problem

Problem: `narrowingDestructuring.ts` parses and resolves, then lower_program
rejects the nested object rest declaration in the narrowed `"f"` branch:

```ts
const { f: { a, ...spread } } = value;
```

Current diagnostic:

```text
UnsupportedRuntimeSubset: issue-251: object rest binding currently requires a static object literal source in this runtime slice at 499..537
```

TypeScript accepts the reference file with no diagnostics. Existing done issue
251 intentionally kept dynamic-source object rest outside its completed runtime
slice, so this issue owns the next narrow lowering step.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingDestructuring.ts
```

Representative source:

```ts
type Z = { kind: "f", f: { a: number, b: string, c: number } }
    | { kind: "g", g: { a: string, b: number, c: string }};

function func2<T extends Z>(value: T) {
    if (value.kind === "f") {
        const { f: f1 } = value;
        const { f: { a, ...spread } } = value;
        value.f;
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; Function func2, If value.kind === "f", Let name "{f:{a,...spread}}" expr value
resolved: ok through builtins
lower_program: issue-251 at const { f: { a, ...spread } } = value
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

The representative nested object rest binding lowers without the static object
literal source guard. The reference path should advance past the current
`499..537` diagnostic to later destructuring/narrowing behavior or a narrower
source-spanned diagnostic.

## Scope

In scope:

- [ ] Lower object rest for a nested binding pattern whose source is a narrowed
  object member, such as `value.f`.
- [ ] Preserve excluded keys for the nested rest object; in the representative
  case `spread` excludes `a` and retains `b` and `c`.
- [ ] Add focused coverage for `const { f: { a, ...rest } } = value` where
  `value` is a known object-like source.
- [ ] Re-run the representative triage and record any later blocker.

Out of scope:

- Full object property enumeration semantics for arbitrary dynamic objects.
- Destructuring assignment expressions.
- Array rest narrowing in `farr`; that is a later blocker if it appears.
- Computed object binding aliases, tracked by
  `issues/done/5297-lower-computed-object-binding-aliases.md` and
  `issues/done/5299-lower-computed-object-binding-parameters.md`.
- Default binding initializer support, tracked by issues 5373 and 5379.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`

Do not touch:

- parser syntax for destructuring patterns unless a focused fixture proves the
  AST representation is missing
- unrelated async/runtime-subset guards

## Acceptance criteria

- [ ] `narrowingDestructuring.ts` no longer reports issue-251 at
  `const { f: { a, ...spread } } = value`.
- [ ] A focused fixture covers nested object rest from an object/member source.
- [ ] Existing unsupported diagnostics remain source-spanned for broader
  dynamic object rest forms that remain out of scope.
- [ ] Any later blocker in the representative reference file is recorded here
  or split to a follow-up issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(destructuring) or test(object_rest)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingDestructuring.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingDestructuring.ts --detail --no-dashboard-data
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

Issue 251 is done, but its notes explicitly keep dynamic-source object rest
outside the completed runtime slice. This issue is the narrow follow-up for the
first concrete reference case now exposed by `narrowingDestructuring.ts`.

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
