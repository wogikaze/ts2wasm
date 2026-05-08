---
id: 1164
title: "Implement Circularreferenceinreturntype Parser Syntax"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5242]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1164.

## Summary

Triage circularReferenceInReturnType-parser-syntax across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `circularReferenceInReturnType-parser-syntax`. Fresh triage shows tokens succeed, but AST construction stops while parsing `object<Something>()({ ... })`, a valid direct generic call on a declared callable const.

Problem: `circularReferenceInReturnType2.ts` is not a standalone generated parser bucket. The current first blocker is the focused generic-call parser gap split to issue 5242.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularReferenceInReturnType2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularReferenceInReturnType2.ts --detail
```

## Desired final state

This generated bucket is closed after splitting `issues/open/5242-w2-completion-declaration.md`. Do not implement directly from this bucket.

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
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularReferenceInReturnType2.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularReferenceInReturnType2.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5242-w2-completion-declaration.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularReferenceInReturnType2.ts`

## Duplicate detection

- `issues/open/5194-report-empty-call-type-arguments.md` covers malformed empty call type-argument lists; this case is valid syntax.
- `issues/open/5202-parse-member-call-explicit-type-arguments.md` covers member callees such as `obj.method<T>()`; this case is a direct identifier call.
- `issues/open/059-implement-parser-syntax-extensions.md` and `issues/open/442-implement-parser-syntax.md` are broad parser parents, not focused implementation-ready owners.

## Smart triage

Fresh triage shows this generated parser-syntax bucket is currently blocked by
valid direct generic-call type-argument syntax on a declared callable const.

### Smart triage: circularReferenceInReturnType2

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `unsupported expression: Some(SpannedToken { kind: RightParen, ... }) at 1077..1078`
- Path: `reference/typescript/tests/cases/compiler/circularReferenceInReturnType2.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularReferenceInReturnType2.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularReferenceInReturnType2.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
declare const object: <Source>() => <
  Fields extends {
    [Key in keyof Fields]: Field<Source, Key & string>;
  }
>(config: {
  name: string;
  fields: Fields | (() => Fields);
}) => ObjectType<Source>;

const A = object<Something>()({
  name: "A",
  fields: () => ({})
});
```

Compiler evidence:

```text
tokens: ok
ast: fails at `object<Something>()`; visible initializer is partial `object<Something>()`
resolved: same UnsupportedSyntax because AST construction failed
```

TypeScript oracle evidence:

```text
typescript ok: true
diagnostics: []
binding A type: any
```

Split result:

- `issues/open/5242-w2-completion-declaration.md`

## Completion evidence

Fill only when moving to `done/`.

The `circularReferenceInReturnType2` parser-syntax bucket is complete. The current failure is split to issue 5242.

Commits:

- split to `issues/open/5242-w2-completion-declaration.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularReferenceInReturnType2.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedSyntax/unknown-unsupported
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularReferenceInReturnType2.ts
result: pass; AST construction reports unsupported expression at `object<Something>()`, split to issue 5242
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5242 may expose the nested call-expression callee blocker covered by issue 5163, then later circular return-type inference semantics.
