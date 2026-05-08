---
id: 5272
title: "Support generic return interface method receivers"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Preserve enough generic return and interface constraint information for method
calls on locals initialized from generic function calls, such as
`const messageList = fetchMsg(this.messageList); messageList.methodOnMessageList();`.

## Problem

`collectionPatternNoError.ts` tokenizes, parses, resolves names, and reaches
lowering, but `lower_program` rejects `messageList.methodOnMessageList()`
because the receiver class is unknown. TypeScript infers `messageList` as `U`,
where `U extends MessageList<T>`, and accepts the interface method call.

Current diagnostic:

```text
UnsupportedSyntax: issue-211: unknown receiver class for method `methodOnMessageList` at 619..652
```

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collectionPatternNoError.ts
```

Representative source:

```ts
interface MessageList<T extends Message> extends Message {
  methodOnMessageList(): T[];
}

function fetchMsg<V extends Message>(protoCtor: MsgConstructor<V>): V {
  return null!;
}

class DataProvider<T extends Message, U extends MessageList<T>> {
  constructor(
    private readonly message: MsgConstructor<T>,
    private readonly messageList: MsgConstructor<U>,
  ) { }

  fetch() {
    const messageList = fetchMsg(this.messageList);
    messageList.methodOnMessageList();
  }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; ClassDecl DataProvider contains constructor parameter property assignments and fetch()
resolved/lowered: issue-211 unknown receiver class for method `methodOnMessageList`
TypeScript oracle: ok, no diagnostics; binding messageList has type U
```

## Desired final state

The compiler no longer reports the generic unknown receiver class diagnostic
for method calls where a local is initialized from a generic function call whose
return type is constrained to an interface with the requested method. The
representative path should either lower/classify the interface method call or
advance to the next semantic blocker with a narrower diagnostic.

## Scope

In scope:

- [ ] Preserve generic function return type information enough to recognize
  `fetchMsg<T>(...) -> T` style results.
- [ ] Propagate `U extends MessageList<T>` constraints to locals initialized
  from generic call results.
- [ ] Classify `messageList.methodOnMessageList()` before the generic
  `issue-211` unknown receiver class path.
- [ ] Add a focused fixture for a generic function returning `U extends
  InterfaceWithMethod` and an immediate method call on the inferred local.

Out of scope:

- Full TypeScript generic inference or assignability.
- Runtime implementation of arbitrary erased interface values.
- Interface-typed erased local method calls without generic call inference,
  tracked by `issues/done/5222-parse-ambient-generic-variable-type-annotations.md`.
- Array-typed parameter receiver tracking, tracked by
  `issues/done/5234-w0-implement-host-deny-and-auditable-e2e-manifest-verification.md`.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/` unless lowering already produces a supported runtime representation
- unrelated builtin array/object/string receiver methods

## Acceptance criteria

- [ ] `collectionPatternNoError.ts` no longer reports
  `issue-211: unknown receiver class for method methodOnMessageList`.
- [ ] A focused fixture covers `function make<U extends HasMethod>(...): U`;
  `const value = make(...); value.method();`.
- [ ] Existing unknown receiver diagnostics remain for unconstrained generic
  return values and erased interface locals not covered by this slice.
- [ ] Re-run the representative reference triage and record the next diagnostic
  or pass state.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(method) or test(interface) or test(generic)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collectionPatternNoError.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collectionPatternNoError.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1265-implement-collectionPatternNoError.md`.

Related but distinct:

- `issues/done/5222-parse-ambient-generic-variable-type-annotations.md`
  handles method calls on locals directly annotated with interface types.
- `issues/done/5234-w0-implement-host-deny-and-auditable-e2e-manifest-verification.md`
  handles array-shaped parameter annotations.
- `issues/done/5261-report-class-typed-missing-instance-method-calls.md`
  handles class-typed ambient locals and missing instance methods.

## Completion evidence

Fill when implemented.
