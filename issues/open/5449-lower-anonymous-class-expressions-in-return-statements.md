---
id: 5449
title: "Lower anonymous class expressions in return statements"
type: feature
area: ir/compiler
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Lower anonymous `class { ... }` expressions when they appear as returned values.

Split from generated bucket `issues/done/3450-implement-narrowedConstInMethod.md`.

## Problem

Problem: `narrowedConstInMethod.ts` parses and resolves, then lower_program
rejects the anonymous class expression returned from `f2` with `issue-313`.

The object-literal method case in `f` parses into a `FunctionExpr` property and
resolves. The remaining current blocker is the returned anonymous class
expression in `f2`.

Current diagnostic:

```text
UnsupportedSyntax: issue-313: class expression lowering not yet implemented
```

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowedConstInMethod.ts
```

Representative source:

```ts
function f2() {
    const x: string | null = <any>{};
    if (x !== null) {
        return class {
            bar() { return x.length; }
        };
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; `f` object-literal method and `f2` returned anonymous class expression parse
resolved: ok through builtins
lower_program: UnsupportedSyntax issue-313 class expression lowering not yet implemented
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

Anonymous class expressions returned from functions lower far enough for the
representative path to advance beyond `issue-313`.

## Scope

In scope:

- [ ] Lower an anonymous class expression used directly as a return value.
- [ ] Preserve methods on the anonymous class body enough to keep lowering
  source-spanned.
- [ ] Add focused coverage for `return class { m() { return 1; } };`.
- [ ] Re-run the representative triage and record the next blocker.

Out of scope:

- Named class expression assignment/initializer lowering, tracked by
  `issues/open/5248-lower-class-expressions.md`.
- Full class runtime value semantics outside direct returned anonymous class
  expressions.
- TypeScript control-flow narrowing of captured consts after lowering advances.

## Affected paths

Expected:

- `crates/compiler/src/`
- `crates/ir/src/`
- focused CLI/compiler fixtures

Do not touch:

- parser syntax unless fresh implementation evidence proves this regressed
  before lowering
- unrelated class declaration runtime support

## Acceptance criteria

- [ ] `narrowedConstInMethod.ts` no longer reports `issue-313` for the returned
  anonymous `class { ... }`.
- [ ] A focused fixture covers `return class { m() { return 1; } };`.
- [ ] Existing named class expression lowering diagnostics remain source-spanned
  or are handled by issue 5248.
- [ ] Any later captured-const narrowing diagnostic is recorded here or split
  into a follow-up issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class) or test(return)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowedConstInMethod.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowedConstInMethod.ts --detail --no-dashboard-data
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

- `issues/open/5248-lower-class-expressions.md` owns named class expressions
  assigned to locals or used as initializer values.

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
