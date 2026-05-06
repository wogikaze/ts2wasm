---
id: 1171
title: "Implement Classaccessorinitializationinferencewithelementaccess"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5232]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Closed as superseded by `issues/open/5232-support-entry-export-class-declarations.md`.

## Problem

Problem: `classAccessorInitializationInferenceWithElementAccess1.ts` no longer
has a distinct class-accessor parser blocker. Fresh triage reaches AST/resolved
output and then stops at the existing entry-module `export class` boundary.

## Current failure

Representative path:

- `reference/typescript/tests/cases/compiler/classAccessorInitializationInferenceWithElementAccess1.ts`

Fresh coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classAccessorInitializationInferenceWithElementAccess1.ts --detail --no-dashboard-data
```

Result:

```text
executed=1
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=class-accessor:1
```

Fresh triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classAccessorInitializationInferenceWithElementAccess1.ts
```

Result:

```text
UnsupportedModule: issue-5005: entry module `export Cls` uses a declaration form
outside the current static export slice; only export const and export default are supported
```

The parser and resolver dumps are successful. They include `ExportDecl(ClassDecl)`,
auto-accessor tokens, constructor `this['x']` / `this[0]` assignments, and resolved
`PropertyAssign` / `PropertyAssignDynamic` statements.

## Desired final state

Issue 5232 implements the shared `export class` module boundary. This generated
bucket should not be implemented directly.

## Scope

In scope:

- [x] Refresh the representative coverage and triage evidence.
- [x] Confirm the current blocker is not a distinct class-accessor parser failure.
- [x] Link the bucket to the existing implementation-ready export-class issue.

Out of scope:

- Implementing entry-module `export class`; tracked by issue 5232.
- Broad class-accessor semantics after the export-class boundary is removed.

## Affected paths

Expected:

- `issues/open/5232-support-entry-export-class-declarations.md`

Do not touch:

- frontend/runtime/backend implementation in this issue-only cleanup

## Acceptance criteria

- [x] Fresh triage identifies issue-5005 `export class` as the current blocker.
- [x] Existing issue 5232 covers that blocker.
- [x] 1171 is moved to done with dependency on 5232.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classAccessorInitializationInferenceWithElementAccess1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classAccessorInitializationInferenceWithElementAccess1.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Notes

The coverage feature label remains `class-accessor` because the path belongs to
that generated family, but the fresh compiler diagnostic is the shared
`UnsupportedModule` export-class boundary.

## Completion evidence

Completed by superseding to issue 5232.
