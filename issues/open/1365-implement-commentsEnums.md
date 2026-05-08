---
id: 1365
title: "Implement Commentsenums"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: [5284]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1365.

## Summary

Closed after splitting the current plain enum declaration binding blocker into
`issues/done/5284-bind-plain-enum-declarations-before-member-access.md`.

## Problem

Reference test results show 1 case failing in directory `commentsEnums` with
diagnostics: parser-syntax. Fresh triage shows tokens and non-enum statements
parse, but the plain enum declaration is omitted from the ts2wasm AST, so later
member access fails in name resolution.

Problem: `commentsEnums.ts` currently reports `UnresolvedName` for `Colors` in
`var x = Colors.Cornflower;` because `enum Colors { ... }` did not create a
binding.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsEnums.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsEnums.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
UnresolvedName: unresolved name: `Colors` at 254..260
unsupported_features=name-resolution:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/done/5284-bind-plain-enum-declarations-before-member-access.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one observable behavior into an implementation-ready child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is split
- [x] Child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, token/AST evidence, and TypeScript oracle evidence
- [x] Child issue acceptance names the exact reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsEnums.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsEnums.ts
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

- [x] `issues/done/5284-bind-plain-enum-declarations-before-member-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsEnums.ts`

## Duplicate detection

- `issues/open/428-implement-enum.md` is the broad enum generated bucket and is
  too wide to implement directly.
- `issues/open/2121-implement-enumBasics-parser-syntax.md` and
  `issues/open/2143-implement-enumPropertyAccess.md` are generated buckets that
  may expose the same family later, but they are not implementation-ready
  children.
- `issues/done/5184-parse-const-enum-declarations.md` covers `const enum`.
- `issues/done/5277-parse-export-enum-declarations-to-enum-boundary.md` covers
  `export enum`.
- `issues/done/5284-bind-plain-enum-declarations-before-member-access.md` owns
  this current plain enum binding blocker.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage name resolution: commentsEnums

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/commentsEnums.ts
```

Failure:

```text
unresolved name: `Colors` at 254..260
```

Source context:

```ts
enum Colors {
    /** Fancy name for 'blue'*/
    Cornflower /* blue */,
    /** Fancy name for 'pink'*/
    FancyPink
} // trailing comment
var x = Colors.Cornflower;
x = Colors.FancyPink;
```

Compiler evidence:

```text
tokens: ok, with enum spelled as Ident("enum") followed by Ident("Colors")
ast: ok but contains only Let x = Member(Ident Colors, Cornflower) and Assign x = Member(Ident Colors, FancyPink)
resolved: UnresolvedName for Colors
```

TypeScript oracle:

```text
ok: true
diagnostics: []
binding x: Colors
TypeScript AST includes EnumDeclaration for Colors
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsEnums.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=UnresolvedName:1, unsupported_features=name-resolution:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsEnums.ts
result: UnresolvedName for Colors because the enum declaration is not bound; split to issue 5284
date: 2026-05-07
```

Remaining risks:

- Full enum runtime transform and comment/declaration emit fidelity remain out
  of scope for the child issue.
