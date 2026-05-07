---
id: 1411
title: "Implement Computedpropertieswithsetterassignment"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: []
blocks: [5184]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1411.

## Summary

Closed as superseded by
`issues/done/5184-parse-const-enum-declarations.md`.

Fresh triage shows the current first blocker is not setter assignment: the
`const enum Props` declaration is misparsed as a bogus `const enum` binding, so
later `foo[Props.k]` fails name resolution.

## Problem

Reference test results originally showed 1 case failing in directory
`computedPropertiesWithSetterAssignment` with diagnostics: unknown-unsupported.
Fresh focused coverage now reports `UnresolvedName` for `Props`.

Problem: 1411 is not a standalone computed setter assignment work order in the
current runner view. The first actionable blocker is the const-enum parser and
binding behavior already owned by issue 5184.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesWithSetterAssignment.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesWithSetterAssignment.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implementation should proceed through issue
5184 until `const enum Props { ... }` is represented or diagnosed before
ordinary name resolution reaches `Props.k`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5184
- [x] Preserve exact reproduction commands and representative evidence

Out of scope:

- Direct implementation from this generated bucket
- Computed setter assignment semantics after const enum handling advances

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
- [x] Existing issue 5184 owns the current const-enum parser/binding blocker
- [x] This issue includes failing path, diagnostic code, source context, compiler evidence, and TypeScript oracle evidence
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesWithSetterAssignment.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesWithSetterAssignment.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only
- `cargo nextest run`; issue metadata only

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/done/5184-parse-const-enum-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/computedPropertiesWithSetterAssignment.ts`

## Duplicate detection

- `issues/done/5184-parse-const-enum-declarations.md` owns the current first
  blocker: `const enum` is parsed as a `const` declaration named `enum`,
  leaving `Props` unresolved.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage name resolution: computedPropertiesWithSetterAssignment

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/computedPropertiesWithSetterAssignment.ts
```

Source context:

```ts
const k = Symbol();

const enum Props {
    k = 'k',
}

interface Foo {
    get [k](): Set<string>;
    set [k](v: Iterable<string>);
}

declare const foo: Foo;
foo[Props.k] = ['foo'];
```

Compiler evidence:

```text
tokens: ok; includes Const Ident("enum") Ident("Props")
ast: ok; const enum and interface are omitted, visible symbols include bogus binding `enum`
resolved: UnresolvedName `Props` at 319..324
```

TypeScript oracle evidence:

```text
ok=true, diagnostics=[]
AST includes EnumDeclaration "const enum Props { k = 'k' }" and InterfaceDeclaration Foo.
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
semantic_enabled=0
```

## Completion evidence

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesWithSetterAssignment.ts
result: pass; reproduces UnresolvedName `Props` after const-enum misparse
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesWithSetterAssignment.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, blocked=0
date: 2026-05-07
```

Remaining risks:

- After issue 5184 advances const enum parsing, this file may expose interface
  accessor parsing, computed setter assignment, `Symbol()` or `Iterable` support
  as later blockers.
