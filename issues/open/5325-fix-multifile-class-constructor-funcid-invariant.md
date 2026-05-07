---
id: 5325
title: "Fix multifile class constructor FuncId invariant"
type: feature
area: ir/compiler
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Fix the lowered-IR/WAT validation path that reports an out-of-range class
constructor function id for a multi-file TypeScript reference case.

## Problem

`classMemberInitializerWithLamdaScoping2.ts` focused coverage reports
`build_pass`, but `reference-triage` exposes a compiler invariant while dumping
the build pipeline:

```text
InvariantViolation: ClassDecl constructor FuncId 0 is out of range (program has 0 function(s))
```

Problem: a class with a constructor in the second virtual file can produce a
`ClassDecl` whose constructor `FuncId` references function index 0 even though
the lowered program has zero emitted functions.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping2.ts
```

Source context:

```ts
// @Filename: classMemberInitializerWithLamdaScoping2_0.ts
var field1: string;

// @Filename: classMemberInitializerWithLamdaScoping2_1.ts
declare var console: {
    log(msg?: any): void;
};
class Test1 {
    constructor(private field1: string) {
    }
    messageHandler = () => {
        console.log(field1);
    };
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; Let field1 plus ClassDecl Test1 with constructor parameter property
resolved: ok; ClassDecl Test1 has constructor Some([...])
triage diagnostic: InvariantViolation ClassDecl constructor FuncId 0 is out of range (program has 0 function(s))
focused coverage: build_pass=1, unsupported=0
```

TypeScript oracle evidence:

```text
typescript ok: false
diagnostics:
- TS2403 console redeclaration
- TS2301 class field initializer cannot reference constructor parameter field1
```

## Desired final state

The build/triage pipeline either preserves a valid constructor function id or
removes the constructor reference consistently when no function is emitted, so
`reference-triage` no longer reports an invariant violation for this path.

## Scope

In scope:

- [ ] Fix the zero-emitted-function class constructor metadata path for this multi-file TypeScript shape.
- [ ] Add focused validation coverage for a class constructor in a non-exported virtual file when no functions are emitted.
- [ ] Re-run the representative reference triage and confirm the invariant is gone.

Out of scope:

- TS2301 class field initializer scoping semantics.
- Module export/import support; this representative has no import/export first blocker.
- Broad class runtime behavior unrelated to constructor function id validity.

## Affected paths

Expected:

- `crates/ir/`
- `crates/compiler/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- module export semantics
- frontend parser syntax unless a focused regression proves AST production changed

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping2.ts` no longer reports `ClassDecl constructor FuncId 0 is out of range`.
- [ ] Focused coverage remains `build_pass=1` or advances to a narrower non-invariant diagnostic.
- [ ] Existing class constructor lowering tests still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli class
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping2.ts --detail --no-dashboard-data
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

Split from generated bucket
`issues/open/1224-implement-classMemberInitializerWithLamdaScoping-module-system-amd.md`.

Related but not duplicates:

- `issues/open/5247-fix-js-noemit-class-constructor-funcid-invariant.md`
  owns the same invariant in a JS/noEmit reference path. This issue keeps the
  multi-file TypeScript reference window explicit so the fix is verified against
  both shapes.

## Completion evidence

Fill when implemented.
