---
id: 1495
title: "Implement Contextualreturntypeofiife Import Export"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextualReturnTypeOfIIFE-import-export across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `contextualReturnTypeOfIIFE-import-export` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualReturnTypeOfIIFE-import-export has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE3.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE3.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE3.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5370-bind-ambient-namespace-declarations-for-qualified-value-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE3.ts`
- `reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE2.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/done/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/543-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/549-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/662-implement-arrayAssignmentTest-import-export.md` - Implement Arrayassignmenttest Import Export (same feature label, title overlap)
- `issues/open/732-implement-assignmentCompatability-import-export.md` - Implement Assignmentcompatability Import Export (same feature label, title overlap)
- `issues/done/766-implement-augmentedTypesEnum-import-export.md` - Implement Augmentedtypesenum Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)

## Smart triage

Fresh triage on 2026-05-07 shows this generated import/export bucket now
reaches a name-resolution blocker in both affected reference files. There is no
current static import/export syntax blocker in this window.

Focused coverage:

```text
contextualReturnTypeOfIIFE3.ts: UnresolvedName: name-resolution
contextualReturnTypeOfIIFE2.ts: blocked
```

Focused triage for both files reports the same resolver shape:

```text
UnresolvedName: unresolved name: `app`
```

Representative source context:

```ts
declare namespace app {
  var foo: {
    bar: {
      someFun: (arg: number) => void;
    };
  };
}

app.foo.bar = (function () {
  return { someFun(arg) {} };
})();
```

The compiler tokenizes and parses the ambient namespace and the qualified
assignment/call. The AST then keeps the outside statements, but `resolve_names`
cannot find the top-level `app` identifier. TypeScript accepts both files with
no diagnostics.

This is adjacent to, but not covered by,
`issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`,
which explicitly scopes non-ambient namespaces and excludes ambient
`declare namespace`. It is also narrower than broad import/export umbrella
`issues/open/432-implement-import-export.md`.

The ambient namespace value-access blocker was split to
`issues/open/5370-bind-ambient-namespace-declarations-for-qualified-value-access.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` (filled by commit that moves this issue)

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE3.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE2.ts --detail --no-dashboard-data
result: pass; executed=1, blocked=1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE3.ts
result: pass; UnresolvedName for ambient namespace `app` split to issue 5370
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualReturnTypeOfIIFE2.ts
result: pass; UnresolvedName for ambient namespace `app` split to issue 5370
date: 2026-05-07
```

Remaining risks:

- The intended contextual return type inference behavior remains hidden until
  issue 5370 advances these files past the current ambient namespace
  name-resolution boundary.
