---
id: 5329
title: "Report class namespace duplicate member diagnostics"
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

Report TypeScript-style TS2300 duplicate identifier diagnostics when a class
merged with a namespace has duplicate exported/static member names.

This is the semantic follow-up exposed by `cloduleWithDuplicateMember1.ts` and
`cloduleWithDuplicateMember2.ts`, both of which now build successfully.

## Problem

The current frontend parses and build-passes the duplicate member clodule cases,
but TypeScript reports duplicate identifiers for names repeated between class
static accessors/methods and exported namespace members.

Problem: class/namespace duplicate member names currently produce a false build
pass instead of source-spanned duplicate identifier diagnostics.

## Current failure

Reproduction: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember1.ts`.

Focused coverage: `python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember --detail --no-dashboard-data`.

Observed compiler result:

```text
cloduleWithDuplicateMember1.ts: build_pass
cloduleWithDuplicateMember2.ts: build_pass
```

TypeScript oracle:

```text
cloduleWithDuplicateMember1.ts: TS2300 Duplicate identifier 'x' and 'foo'
cloduleWithDuplicateMember2.ts: TS2300 Duplicate identifier 'x'
```

Representative source:

```ts
class C {
    get x() { return 1; }
    static get x() {
        return '';
    }
    static foo() { }
}

namespace C {
    export var x = 1;
}
namespace C {
    export function foo() { }
    export function x() { }
}
```

Compiler evidence:

```text
tokens: ok through class accessors, static methods, namespace exports
ast/resolved: ok; class C retained, namespace declarations currently erased
oracle: ModuleDeclaration entries produce duplicate identifier diagnostics
```

## Desired final state

The compiler reports source-spanned duplicate identifier diagnostics for the
representative class/namespace duplicate member cases instead of treating them
as clean build passes.

## Scope

In scope:

- [ ] Detect duplicate names between class static members/accessors and exported namespace members for a same-name class/namespace merge.
- [ ] Report TS2300-style duplicate identifier diagnostics at the duplicate member identifiers.
- [ ] Add focused coverage for static getter plus namespace `export var x` and namespace `export function x`.

Out of scope:

- Full declaration merge semantics for arbitrary namespaces.
- Runtime namespace member lowering.
- Var/function duplicate identifier diagnostics; tracked by `issues/done/5307-report-var-function-duplicate-identifier-diagnostics.md`.
- Object type literal duplicate property diagnostics.

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

- [ ] `cloduleWithDuplicateMember1.ts` no longer build-passes silently; it reports duplicate identifiers for `x` and `foo`.
- [ ] `cloduleWithDuplicateMember2.ts` no longer build-passes silently; it reports duplicate identifier `x`.
- [ ] A focused test covers class static accessor/method names colliding with exported namespace members.
- [ ] Existing valid class/namespace merge build-pass cases continue to build.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(duplicate) or test(class)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleWithDuplicateMember --detail --no-dashboard-data
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

Split from `issues/open/1257-implement-cloduleWithDuplicateMember.md` on
2026-05-07.

Related but not duplicates:

- `issues/done/5307-report-var-function-duplicate-identifier-diagnostics.md`
  covers var/function duplicate diagnostics.
- `issues/open/343-implement-duplicate-local-detection.md` is broader duplicate
  local infrastructure and explicitly leaves TypeScript-specific duplicate
  identifier rules out of scope.

## Completion evidence

Fill only when implemented.
