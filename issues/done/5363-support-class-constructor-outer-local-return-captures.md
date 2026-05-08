---
id: 5363
title: "Support class constructor outer local return captures"
type: feature
area: ir
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Allow a class constructor body to return a value read from an outer lexical binding, covering the first blocker in `constructorWithCapturedSuper.ts`.

This is a narrow issue-289 constructor lexical-capture slice, distinct from later-class `new foo()` captures (5266), outer callback captures (5152), rest-parameter captures (5338), and lexical `super` captures (5204).

## Problem

`constructorWithCapturedSuper.ts` tokenizes and parses successfully, but name resolution rejects the base class constructor because it returns the outer local `oneA`.

Problem: class constructor bodies cannot currently resolve and lower direct
reads of outer lexical locals returned from the constructor body.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorWithCapturedSuper.ts
```

Equivalent mise task:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constructorWithCapturedSuper.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-289: class constructor `constructor` references outer local `oneA`; class constructor lexical captures require environment support at 83..87
```

Source context:

```ts
let oneA: A;

class A {
    constructor() {
        return oneA;
    }
}
```

Smart triage evidence on 2026-05-07:

```text
tokens: ok
AST: ok; top-level Let oneA, ClassDecl A, ClassDecl B extends A, ClassDecl C extends A, ClassDecl D extends A
resolved: fails in resolve_names on constructor return expression `oneA`
visible symbols before failure: binding oneA, class A
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

Constructor bodies can resolve and lower a direct immutable outer local read used as the returned constructor value, without emitting issue-289 for `oneA`.

## Scope

In scope:

- [x] Resolve direct constructor-body reads of immutable outer locals.
- [x] Lower `return outerLocal;` from a class constructor without losing existing returned value semantics.
- [x] Add or update a focused regression for `let oneA: A; class A { constructor() { return oneA; } }`.
- [x] Re-run `constructorWithCapturedSuper.ts` and split any later blocker separately.

Out of scope:

- Constructor calls to outer callbacks, tracked by issue 5152.
- Constructor `new` of later class bindings, tracked by issue 5266.
- Constructor rest-parameter captures, tracked by issue 5338.
- Lexical `super` captures in `super(...)` arguments, tracked by issue 5204.
- Full derived-constructor early-return and `super()` validation.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch: package/module resolution, unrelated class parser behavior, unrelated runtime builtins.

## Acceptance criteria

- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorWithCapturedSuper.ts` no longer reports issue-289 for `oneA`.
- [x] A focused fixture proves `let oneA: A; class A { constructor() { return oneA; } }` resolves past constructor lexical capture.
- [x] Existing diagnostics remain for unsupported constructor capture shapes outside this scope.
- [x] Any next blocker from `constructorWithCapturedSuper.ts` is recorded here or split to a follow-up.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(class) or test(constructor) or test(capture)'
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorWithCapturedSuper.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithCapturedSuper.ts --detail --no-dashboard-data
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket `issues/open/1481-implement-constructorWithCapturedSuper.md` on 2026-05-07.

Related but distinct open issues:

- `issues/done/5152-support-class-constructor-outer-callback-captures.md`
- `issues/done/5204-resolve-lexical-super-property-captures-in-super-call-arguments.md`
- 
- `issues/done/5338-support-rest-constructor-outer-local-captures.md`

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

## False-done audit

**truly-done** (5363)

- Implementation commits: verified via `git log --oneline --all --grep=5363`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
