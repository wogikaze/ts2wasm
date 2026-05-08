---
id: 1131
title: "Implement Checkforobjecttoostrict"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5225]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1131.

## Summary

Triage checkForObjectTooStrict across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `checkForObjectTooStrict`. Fresh triage shows tokens and AST succeed; the current blocker is `builtin_resolver` rejecting `class Bar extends Foo.Object` with the generic simple-inheritance diagnostic.

Problem: `checkForObjectTooStrict.ts` is too broad for direct implementation. Its current observable blocker is now tracked by `issues/open/5225-w0-typed-wat-writer.md`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkForObjectTooStrict.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkForObjectTooStrict.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/open/5225-w0-typed-wat-writer.md`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the current observable blocker into issue 5225
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this issue and issue 5225

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
- [x] Issue 5225 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Issue 5225 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5225 acceptance names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkForObjectTooStrict.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkForObjectTooStrict.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5225-w0-typed-wat-writer.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkForObjectTooStrict.ts`

## Duplicate detection

- `issues/open/1195-implement-classExtendingQualifiedName.md` is related but was a generated blocked bucket; issue 5225 records the exact current implementation-ready blocker.
- Broad object-literal/import-export buckets are not exact matches because the AST succeeds and the failure is class heritage resolution.

## Smart triage

### Smart triage: Triage object literal: checkForObjectTooStrict

- Issue class: `triage-needed`
- Feature label: `object-literal`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/checkForObjectTooStrict.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkForObjectTooStrict.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkForObjectTooStrict.ts --detail --no-dashboard-data
```

Source context:

```text
namespace Foo {
    export class Object {
    }
}

class Bar extends Foo.Object {
    constructor () {
        super();
    }
}

class Baz extends Object {
    constructor () {
        super();
    }
}
```

Current compiler failure:

```text
error: [UnsupportedSyntax] only simple inheritance (extends ClassName) is supported
```

Compiler evidence:

- Tokens succeed for the namespace class, `class Bar extends Foo.Object`, `super()`, and `class Baz extends Object`.
- AST succeeds with `ClassDecl Bar extends Member(Ident Foo, "Object")` and `ClassDecl Baz extends Ident Object`.
- Resolved output stops in `builtin_resolver` because class heritage currently accepts only simple identifiers.

TypeScript oracle evidence:

```text
TS2725: Class name cannot be 'Object' when targeting ES5 and above with module CommonJS.
```

Resolution:

```text
The current blocker is now tracked by child issue 5225. It is narrower than the generated bucket: support or precisely diagnose qualified class heritage names.
```

## Completion evidence

Fill only when moving to `done/`.

checkForObjectTooStrict triage is complete. The actionable blocker is tracked
by child issue 5225.

Commits:

- child issue: `issues/open/5225-w0-typed-wat-writer.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkForObjectTooStrict.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedSyntax object-literal
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkForObjectTooStrict.ts
result: pass; reproduced simple-inheritance heritage diagnostic and split to issue 5225
date: 2026-05-06
```

Remaining risks:

- none
