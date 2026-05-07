---
id: 1479
title: "Implement Constructorstaticparamname"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: [5362]
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed by splitting the current `constructor(static)` parser failure to
`issues/open/5362-report-strict-mode-static-constructor-parameter-name.md`.

Fresh triage shows the current blocker is not broad parser-syntax work. It is a
focused strict-mode reserved-word parameter-name diagnostic for `static` inside
a class constructor.

## Problem

Reference test results originally showed one parser-syntax failure. Current
triage shows:

```text
UnsupportedSyntax: issue-247: expected binding identifier or pattern, got Some(Static)
```

TypeScript parses the parameter and reports TS1213 because class definitions
are automatically strict mode.

Problem: `constructor(static)` is rejected by the parser as a generic binding
failure instead of a source-spanned strict-mode reserved-word parameter
diagnostic.

## Current failure

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorStaticParamName.ts --detail --no-dashboard-data
```

Observed:

```text
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Fresh triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorStaticParamName.ts
```

Compiler evidence:

```text
tokens: ok; Static token at 152..158
ast: fails before AST construction
diagnostic: issue-247 expected binding identifier or pattern, got Some(Static)
```

TypeScript oracle evidence:

```text
TS1213: Identifier expected. 'static' is a reserved word in strict mode. Class definitions are automatically in strict mode.
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5362.

## Scope

In scope:

- [x] Inspect fresh triage for `constructorStaticParamName.ts`
- [x] Compare against existing invalid constructor parameter modifier issue 5355
- [x] Split the strict-mode `static` parameter-name diagnostic to issue 5362

Out of scope:

- Direct implementation from this generated bucket
- Invalid constructor parameter modifiers such as `static a`, tracked by issue 5355
- Broad parser-syntax work

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/diagnostic.rs`
- focused parser tests

Do not touch:

- backend/runtime code for this issue-metadata closure

## Acceptance criteria

- [x] Fresh triage records the exact `constructor(static)` parser failure
- [x] Existing issue 5355 is confirmed related but not exact
- [x] Child issue 5362 contains exact reproduction and acceptance criteria

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorStaticParamName.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorStaticParamName.ts
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

- [x] created: `issues/open/5362-report-strict-mode-static-constructor-parameter-name.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorStaticParamName.ts`

## Duplicate detection

- `issues/open/5355-report-invalid-constructor-parameter-modifiers.md` is
  related but not exact: it owns `static a`, `public static a`, and `export a`
  invalid parameter modifiers. This issue owns `static` as the parameter name in
  a class constructor.

## Smart triage

Generated 2026-05-07.

```text
Path: reference/typescript/tests/cases/compiler/constructorStaticParamName.ts
Compiler: UnsupportedSyntax issue-247 expected binding identifier or pattern, got Some(Static)
TypeScript: TS1213 strict-mode reserved word parameter name
```

## Completion evidence

Commits:

- filled by local commit that moves this issue to `done/`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorStaticParamName.ts --detail --no-dashboard-data
result: pass; unsupported=1, UnsupportedSyntax parser failure reproduced
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorStaticParamName.ts
result: pass; issue-247 parser failure and TS1213 oracle evidence captured
date: 2026-05-07
```

Remaining risks:

- implementation remains tracked by issue 5362
