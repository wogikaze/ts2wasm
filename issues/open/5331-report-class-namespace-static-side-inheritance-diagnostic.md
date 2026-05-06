---
id: 5331
title: "Report class namespace static side inheritance diagnostic"
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

Report TypeScript TS2417 when a derived class merged with a namespace has a
static side that is incompatible with the base class's namespace-augmented
static side.

This is the semantic follow-up exposed by `clodulesDerivedClasses.ts`, which
now build-passes.

## Problem

`clodulesDerivedClasses.ts` declares `class Shape`, augments
`Shape.Utils.convert`, then declares `class Path extends Shape` and augments
`Path.Utils.convert2`. The current compiler erases the namespaces, keeps the
class inheritance, and build-passes. TypeScript reports TS2417 because
`typeof Path.Utils` is missing `convert` required by `typeof Shape.Utils`.

Problem: namespace-augmented static-side inheritance compatibility currently
produces a false build pass instead of a source-spanned TS2417-style diagnostic.

## Current failure

Reproduction: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/clodulesDerivedClasses.ts`.

Focused coverage: `python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/clodulesDerivedClasses.ts --detail --no-dashboard-data`.

Observed compiler result:

```text
clodulesDerivedClasses.ts: build_pass
```

TypeScript oracle:

```text
TS2417: Class static side 'typeof Path' incorrectly extends base class static side 'typeof Shape'.
Types of property 'Utils' are incompatible.
Property 'convert' is missing in type 'typeof Path.Utils' but required in type 'typeof Shape.Utils'.
```

Representative source:

```ts
class Shape {
    id: number;
}

namespace Shape.Utils {
    export function convert(): Shape { return null; }
}

class Path extends Shape {
    name: string;
}

namespace Path.Utils {
    export function convert2(): Path {
        return null;
    }
}
```

Compiler evidence:

```text
tokens: ok through class declarations, dotted namespaces, and namespace functions
ast/resolved: ok; ClassDecl Shape and ClassDecl Path extends Shape retained
oracle: TS2417 at the Path identifier in class Path extends Shape
```

## Desired final state

The compiler reports a source-spanned diagnostic for namespace-augmented static
side incompatibility in class inheritance instead of treating the representative
file as a clean build pass.

## Scope

In scope:

- [ ] Detect derived class static-side mismatch caused by namespace-augmented static members.
- [ ] Report a TS2417-style diagnostic at the derived class identifier.
- [ ] Add focused coverage for `class B extends A` where `namespace A.Utils` has an export missing from `namespace B.Utils`.

Out of scope:

- Full TypeScript structural type checking.
- Strict property initialization diagnostics TS2564.
- Null assignability diagnostics TS2322.
- Runtime namespace member lowering.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused frontend/resolver tests or fixtures

Do not touch:

- backend namespace emit
- static ES module resolution
- unrelated class heritage parser support

## Acceptance criteria

- [ ] `clodulesDerivedClasses.ts` no longer build-passes silently; it reports TS2417-style static-side mismatch.
- [ ] A focused test covers base and derived namespace `Utils` exports with a missing derived export.
- [ ] Existing simple `class Derived extends Base` fixtures continue to build.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(class) or test(inherit)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/clodulesDerivedClasses.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/clodulesDerivedClasses.ts --detail --no-dashboard-data
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

Split from `issues/done/1261-implement-clodulesDerivedClasses.md` on
2026-05-07.

Related but not duplicates:

- `issues/open/5314-report-non-constructor-local-class-heritage.md` covers
  non-constructor local values in heritage clauses.
- `issues/open/5315-report-class-extends-interface-diagnostics.md` covers
  class-extends-interface diagnostics.
- `issues/open/5225-support-qualified-class-heritage-names.md` covers qualified
  heritage parsing/lowering, not namespace-augmented static-side compatibility.

## Completion evidence

Fill only when implemented.
