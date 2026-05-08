---
id: 1231
title: "Implement Classorderbug"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1231.

## Summary

Closed after splitting the current blocker to
.
Fresh triage shows parsing succeeds and the remaining failure is the issue-289
constructor lexical-capture boundary for `new foo()`.

## Problem

Reference test results previously showed 1 case failing in directory
`classOrderBug` with diagnostics: parser-syntax. Fresh triage shows tokens and
AST are ok; resolution fails inside the class constructor.

Problem: `classOrderBug.ts` reports `issue-289` because constructor
`constructor` references later class binding `foo` through `new foo()`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classOrderBug.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classOrderBug.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5152 is related but covers callback locals, not this later class binding shape
- [x] Split one observable behavior into child issue 5266
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

- [x] Duplicate candidates below are confirmed as no-match for the exact behavior
- [x] Child issue 5266 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classOrderBug.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classOrderBug.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: 

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classOrderBug.ts`

Source context:

```ts
class bar {
    public baz: foo;
    constructor() {
        this.baz = new foo();
    }
}

class baz {}
class foo extends baz {}
```

## Duplicate detection

- `issues/done/5152-support-class-constructor-outer-callback-captures.md` is
  related by the same issue-289 constructor lexical-capture diagnostic, but its
  acceptance is intentionally scoped to callback-local calls and nested arrow
  `this`.
- No existing open issue owned the later class binding `new foo()` behavior, so
  this bucket was split to issue 5266.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classOrderBug.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classOrderBug.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: issue-289: class constructor `constructor` references outer local `foo`; class constructor lexical captures require environment support at 101..104
Source: this.baz = new foo();
tokens: ok
AST: ok; ClassDecl bar, ClassDecl baz, ClassDecl foo extends baz
resolved: fails in resolve_names on constructor `new foo()`
TypeScript oracle: ok, diagnostics=[]
Child issue: 5266
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to ; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classOrderBug.ts
result: pass; reproduced issue-289 constructor lexical-capture diagnostic and split child issue 5266
date: 2026-05-06
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5266
