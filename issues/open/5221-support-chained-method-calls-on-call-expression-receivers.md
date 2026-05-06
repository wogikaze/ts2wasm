---
id: 5221
title: "Support chained .then calls on call-expression receivers"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Support the narrow `.then(...).then(...)` receiver shape where the second
`.then` receiver is the result of the previous call expression.

## Problem

Problem: both `chainedCallsWithTypeParameterConstrainedToOtherTypeParameter`
reference paths parse and produce nested call/member ASTs, but lowering rejects
the second `.then(...)` because its receiver is a call expression rather than an
identifier.

Current diagnostics:

```text
chainedCallsWithTypeParameterConstrainedToOtherTypeParameter.ts: issue-211: method `then` requires an identifier receiver at 323..408
chainedCallsWithTypeParameterConstrainedToOtherTypeParameter2.ts: issue-211: method `then` requires an identifier receiver at 257..298
```

## Current failure

Use the validation commands below to reproduce the representative failures.

Source shapes:

```ts
(new Chain(new A)).then(a => new B).then(b => new C).then(c => new B).then(b => new A);
(new Chain(t)).then(tt => s).then(ss => t);
```

Compiler evidence:

```text
tokens: ok
ast: ok; nested Call(Member(Call(...), property="then"), args=[ArrowFn ...])
resolved/lowered: issue-211 at chained .then()
TypeScript oracle: emits type diagnostics, not parser/lowering diagnostics
```

## Desired final state

Lowering supports the representative chained `.then(...).then(...)` receiver by
evaluating the receiver once and preserving the receiver binding for the call.

## Scope

In scope:

- [ ] Lower `Call(Member(Call(...), "then"), args)` for the representative `.then(...).then(...)` chain.
- [ ] Add one focused fixture for `factory().then(...).then(...)`.
- [ ] Confirm both reference paths no longer report `issue-211` for `.then`.

Out of scope:

- Generic type inference or TypeScript assignability diagnostics.
- Promise/thenable semantics.
- Optional chaining, computed names, or extracted calls.
- First-call new-expression receiver work already covered by issue 5142.

## Affected paths

Expected:

- `crates/ir/src/lowered/`
- focused compiler fixtures
- `crates/cli/tests/`

Do not touch:

- parser grammar unless triage proves the AST shape regressed
- generic type checker implementation
- unrelated builtin method semantics

## Acceptance criteria

- [ ] `chainedCallsWithTypeParameterConstrainedToOtherTypeParameter.ts` no longer reports `then requires an identifier receiver`.
- [ ] `chainedCallsWithTypeParameterConstrainedToOtherTypeParameter2.ts` no longer reports `then requires an identifier receiver`.
- [ ] A focused fixture covers `factory().then(...).then(...)`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(method) or test(receiver) or test(node_diff)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter.ts --detail
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/chainedCallsWithTypeParameterConstrainedToOtherTypeParameter2.ts --detail
```

Not run:

- none

## Notes

Split from generated bucket `issues/done/1126-implement-chainedCallsWithTypeParameterConstrainedToOtherTypeParameter.md`.
Issue 5217 remains focused on a smaller one-step call-expression receiver
fixture; this issue tracks longer chained method-call expressions from the two
reference paths.

## Completion evidence

Fill when implemented.
