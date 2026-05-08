---
id: 1469
title: "Implement Constructorargwithgenericcallsignature"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: [5287]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1469.

## Summary

Closed as superseded by
`issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`.

Fresh triage shows this bucket stops at the same same-file namespace value
binding boundary: `namespace Test { export function F...; export class
MyClass... }` is erased as a namespace binding, so later qualified value accesses
`Test.F(func)` and `new Test.MyClass(func)` report `UnresolvedName` for `Test`.

## Problem

Reference test results originally showed an import/export bucket. Fresh focused
triage on 2026-05-07 reports `UnresolvedName` / `name-resolution` instead.

Problem: non-ambient namespace declarations are parsed far enough for later
qualified accesses, but the namespace identifier is not bound as a value with
exported members. The first observed failure is `Test.F(func)`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgWithGenericCallSignature.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorArgWithGenericCallSignature.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
constructorArgWithGenericCallSignature.ts: UnresolvedName for `Test`
coverage: executed=1, build_pass=0, unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Source context:

```text
10 |  export function F(func: MyFunc) { }
11 | }
12 | var func: Test.MyFunc;
13 | Test.F(func); // OK
14 | var test = new Test.MyClass(func); // Should be OK
```

Compiler evidence:

```text
tokens: ok through namespace Test, exported interface, exported class, and exported function
ast: ok; retained runtime statements include var func, Test.F(func), and new Test.MyClass(func)
visible symbols: MyClass, F, and func; namespace Test itself is not bound
resolved: UnresolvedName for Test during resolve_names at line 13 column 1
```

TypeScript oracle evidence:

```text
TypeScript parses ModuleDeclaration namespace Test and resolves Test.F and
Test.MyClass; it reports only later definite-assignment diagnostics for func.
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5287,
which owns binding same-file non-ambient namespaces as namespace values for
qualified member access.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5287's namespace value binding work
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Constructor overload or generic call signature semantics after namespace lookup advances
- Definite-assignment diagnostics for `func` after namespace lookup advances

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
- [x] Existing issue 5287 covers same-file namespace value binding for qualified access
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorArgWithGenericCallSignature.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgWithGenericCallSignature.ts
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

- [x] superseded by: `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorArgWithGenericCallSignature.ts`

## Duplicate detection

- `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`
  owns binding same-file non-ambient namespace declarations as namespace values
  so qualified accesses can resolve exported members.
- `issues/open/5225-w0-typed-wat-writer.md` is related but
  covers qualified names in class heritage clauses, not value access.
- `issues/open/5294-resolve-sibling-namespaces-in-nested-namespace-scopes.md`
  is related but covers nested sibling namespace lookup.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage name resolution: constructorArgWithGenericCallSignature

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/constructorArgWithGenericCallSignature.ts
```

Resolved evidence:

```text
[pipeline] validate_ast
[pipeline] module_graph
[pipeline] resolve_names
error: [UnresolvedName] unresolved name: `Test` at 246..250
```

## Completion evidence

Commits:

- filled by local commit that moves this issue to `done/`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorArgWithGenericCallSignature.ts --detail --no-dashboard-data
result: pass; reproduced executed=1 build_pass=0 unsupported=1 UnresolvedName=1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorArgWithGenericCallSignature.ts
result: pass; reproduced UnresolvedName Test for Test.F(func) with parser evidence and TypeScript oracle context
date: 2026-05-07
```

Remaining risks:

- implementation remains tracked by issue 5287
