---
id: 5330
title: "Report namespace before class merge diagnostic"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report TypeScript TS2434 when a non-ambient namespace declaration appears before
the class or function declaration it later merges with.

This is the semantic follow-up exposed by
`cloduleWithPriorInstantiatedModule.ts`, which now build-passes.

## Problem

The current frontend erases the first `namespace Moclodule { ... }`, keeps the
later `class Moclodule {}`, and build-passes. TypeScript reports TS2434 at the
first namespace name because an instantiated namespace cannot be located before
the class/function it merges with.

Problem: prior instantiated namespace/class merge ordering currently produces a
false build pass instead of a source-spanned TS2434-style diagnostic.

## Current failure

Reproduction: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleWithPriorInstantiatedModule.ts`.

Focused coverage: `python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleWithPriorInstantiatedModule.ts --detail --no-dashboard-data`.

Observed compiler result:

```text
cloduleWithPriorInstantiatedModule.ts: build_pass
```

TypeScript oracle:

```text
TS2434: A namespace declaration cannot be located prior to a class or function with which it is merged.
```

Representative source:

```ts
namespace Moclodule {
    export interface Someinterface {
        foo(): void;
    }
    var x = 10;
}

class Moclodule {
}

namespace Moclodule {
    export class Manager {
    }
}
```

Compiler evidence:

```text
tokens: ok through both namespace declarations and class Moclodule
ast/resolved: ok; only ClassDecl Moclodule is retained after namespace erasure
oracle: TS2434 at the first namespace Moclodule identifier
```

## Desired final state

The compiler reports a source-spanned diagnostic for a namespace declaration
that appears before the class/function it merges with. The representative file
should no longer build-pass silently.

## Scope

In scope:

- [ ] Detect same-name namespace declarations that precede a later class/function declaration.
- [ ] Report a TS2434-style diagnostic at the prior namespace identifier.
- [ ] Add focused coverage for `namespace M { var x = 1; } class M {}`.

Out of scope:

- Full declaration merge runtime lowering.
- Namespace member export/value lookup.
- Duplicate member diagnostics; tracked by `issues/open/5329-report-class-namespace-duplicate-member-diagnostics.md`.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused frontend/resolver tests or fixtures

Do not touch:

- backend namespace emit
- static ES module resolution
- unrelated duplicate-local handling

## Acceptance criteria

- [ ] `cloduleWithPriorInstantiatedModule.ts` no longer build-passes silently; it reports TS2434-style namespace-before-class merge ordering.
- [ ] A focused test covers `namespace M { var x = 1; } class M {}`.
- [ ] A valid class-then-namespace merge fixture continues to build or reach its existing narrower blocker.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(class) or test(merge)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleWithPriorInstantiatedModule.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleWithPriorInstantiatedModule.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from `issues/open/1258-implement-cloduleWithPriorInstantiatedModule.md`
on 2026-05-07.

Related but not duplicates:

- `issues/done/771-implement-augmentedTypesModules.md` also contains TS2434
  oracle evidence, but it is a broad generated bucket with five files and an
  older namespace/module ownership failure.
- `issues/open/5329-report-class-namespace-duplicate-member-diagnostics.md`
  covers class/namespace duplicate member TS2300 diagnostics, not declaration
  order.

## Completion evidence

Fill only when implemented.
