---
id: 5383
title: "Classify number parameter toFixed calls"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Classify `toFixed()` calls on arrow-function parameters annotated as `number`
before generic method-call receiver lowering reports issue-211.

## Problem

Problem: `x.toFixed()` inside an arrow callback with `x: number` falls through to `issue-211: unknown receiver class for method toFixed`.

`contextualTypingOfGenericFunctionTypedArguments1.ts` parses successfully,
including explicit member-call type arguments on `_.forEach<number>(...)`, but
lowering stops inside the callback body:

```text
UnsupportedSyntax: issue-211: unknown receiver class for method `toFixed` at 405..416
```

TypeScript accepts `x.toFixed()` as a valid number method and reports the later
callback assignability diagnostic:

```text
TS2345: Argument of type '(x: number) => string' is not assignable to parameter of type '(x: number) => Date'.
```

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfGenericFunctionTypedArguments1.ts
```

Representative source:

```ts
var f = (x: number) => { return x.toFixed() };
var r5 = _.forEach<number>(c2, f);
var r6 = _.forEach<number>(c2, (x) => { return x.toFixed() });
```

Compiler evidence:

```text
tokens: ok
AST: ok; typed arrow `f` body contains Call(Member(Ident("x"), "toFixed"))
resolved/lowered: issue-211 unknown receiver class for method `toFixed`
TypeScript oracle: TS2345 at the callback arguments to `_.forEach<number>`
```

## Desired final state

The compiler no longer reports the generic unknown receiver diagnostic for
`x.toFixed()` where `x` is a number-annotated arrow parameter. The
representative reference advances to the generic callback type diagnostic or
another more specific blocker.

## Scope

In scope:

- [ ] Preserve or consult the `x: number` parameter annotation for direct
  `x.toFixed()` calls inside arrow bodies.
- [ ] Classify `toFixed()` on a number-typed local before the generic
  issue-211 unknown receiver path.
- [ ] Add focused coverage for `(x: number) => { return x.toFixed() }`.

Out of scope:

- Full `Number.prototype.toFixed` formatting/runtime compatibility.
- Generic member overload resolution for `_.forEach<number>(...)`.
- Reporting the final TS2345 callback return mismatch.
- Explicit member-call type argument parsing; see
  `issues/open/5202-parse-member-call-explicit-type-arguments.md`.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/` unless a focused fixture proves runtime lowering is
  the smallest way to remove the current issue-211 blocker.
- General method-call receiver support.

## Acceptance criteria

- [ ] `contextualTypingOfGenericFunctionTypedArguments1.ts` no longer reports
  `issue-211: unknown receiver class for method toFixed` at the standalone
  `var f = (x: number) => ...` callback body.
- [ ] A focused test covers `let f = (x: number) => { return x.toFixed() };`.
- [ ] Existing unsupported arbitrary method-call receiver diagnostics remain
  source-spanned and unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(number) or test(method) or test(callback)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfGenericFunctionTypedArguments1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfGenericFunctionTypedArguments1.ts --detail --no-dashboard-data
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

Split from generated bucket
`issues/done/1526-implement-contextualTypingOfGenericFunctionTypedArguments.md`.

Related but distinct:

- `issues/open/5202-parse-member-call-explicit-type-arguments.md` owned the
  earlier parser boundary for `_.map<number, string>(...)`; this representative
  now parses explicit type arguments and reaches callback body lowering.

## Completion evidence

Fill when implemented.
