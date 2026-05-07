---
id: 1254
title: "Implement Clodulesplitacrossfiles"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1254.

## Summary

Triage cloduleSplitAcrossFiles across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results originally showed 1 case failing in directory
`cloduleSplitAcrossFiles` with diagnostics: import-export. Fresh focused triage
on 2026-05-07 reports `UnresolvedName` / `name-resolution` instead.

Problem: cloduleSplitAcrossFiles has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cloduleSplitAcrossFiles.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/cloduleSplitAcrossFiles.ts --detail
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
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleSplitAcrossFiles.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleSplitAcrossFiles.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5328-share-script-globals-across-filename-sections.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/cloduleSplitAcrossFiles.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-07.

Fresh focused triage shows this generated import/export bucket is stale. The
current blocker is a cross-section script binding issue now split to
`issues/open/5328-share-script-globals-across-filename-sections.md`.

```text
### Smart triage: Triage name resolution: cloduleSplitAcrossFiles

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/cloduleSplitAcrossFiles.ts
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
reference/typescript/tests/cases/compiler/cloduleSplitAcrossFiles.ts: UnresolvedName: name-resolution
```

Source shape:

```ts
// @Filename: cloduleSplitAcrossFiles_class.ts
class D { }

// @Filename: cloduleSplitAcrossFiles_module.ts
namespace D {
    export var y = "hi";
}
D.y;
```

Compiler evidence:

```text
tokens: ok for class D, namespace D, export var y, and D.y
ast: contains ClassDecl D and Expr Member(Ident("D").y); namespace D is erased
resolved: UnresolvedName for D during resolve_names
visible symbols before failure: []
```

TypeScript oracle evidence:

```text
typescriptVersion: 6.0.3
diagnostics: []
topLevel: ClassDeclaration, ModuleDeclaration, ExpressionStatement
binding hint: y has type string
```

Duplicate review:

- `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`
  is related but owns same-file namespace value access, not class declarations
  shared across `@Filename` script sections.
- `issues/done/5187-lower-namespace-only-multi-section-files.md` is related
  but owns namespace-only/declaration-only sections dropped as empty bodies.
- `issues/done/5229-w0-user-runtime-string-origin.md` is related
  but owns local import specifier resolution between virtual files.

## Completion evidence

Closed as split on 2026-05-07.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleSplitAcrossFiles.ts
result: pass; reproduced UnresolvedName for D and split cross-section global script binding to issue 5328
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleSplitAcrossFiles.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-07
```

Remaining risks:

- Class/namespace merge semantics after cross-section lookup advances remain
  tracked by issue 5328 or by follow-up issues split from it.
