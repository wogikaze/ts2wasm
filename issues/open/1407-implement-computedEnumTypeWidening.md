---
id: 1407
title: "Implement Computedenumtypewidening"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: [5284]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1407.

## Summary

Closed as superseded by
`issues/done/5284-bind-plain-enum-declarations-before-member-access.md`.

Fresh triage shows the old parser-syntax bucket now parses the file far enough
to fail name resolution for `E.B`, because the plain enum declaration does not
create a binding.

## Problem

Reference test results originally showed 1 case failing in directory
`computedEnumTypeWidening` with diagnostics: parser-syntax. Fresh focused
coverage now reports `UnresolvedName` for `E` at the first `E.B` member access.

Problem: 1407 is not a standalone computed-enum type-widening work order in the
current runner view. The first actionable blocker is the plain enum binding
gap already owned by issue 5284.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedEnumTypeWidening.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedEnumTypeWidening.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implementation should proceed through issue
5284 until plain enum declarations bind before member access.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5284
- [x] Preserve exact reproduction commands and representative evidence

Out of scope:

- Direct implementation from this generated bucket
- TypeScript enum type-widening semantics after enum binding advances

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
- [x] Existing issue 5284 owns the current plain enum binding blocker
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedEnumTypeWidening.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedEnumTypeWidening.ts
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

- [x] superseded by: `issues/done/5284-bind-plain-enum-declarations-before-member-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/computedEnumTypeWidening.ts`

## Duplicate detection

- `issues/done/5284-bind-plain-enum-declarations-before-member-access.md`
  owns the exact current behavior: a plain `enum E { ... }` declaration does
  not create a frontend binding before later `E.B` member access.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage name resolution: computedEnumTypeWidening

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/computedEnumTypeWidening.ts
```

Source context:

```ts
declare function computed(x: number): number;

enum E {
    A = computed(0),
    B = computed(1),
    C = computed(2),
    D = computed(3),
}

function f1() {
    const c1 = E.B;
```

Compiler evidence:

```text
tokens: ok; includes Ident("enum"), Ident("E"), computed member initializers, and E.B
ast: ok; contains function computed and function f1 with Member(Ident E, property B), but no enum declaration node
resolved: UnresolvedName `E` at 247..248
```

TypeScript oracle evidence:

```text
ok=true, diagnostics=[]
hints include c1: E.B, v1: E, and later widened enum member types
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
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedEnumTypeWidening.ts
result: pass; reproduces UnresolvedName `E` at first `E.B`
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedEnumTypeWidening.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, blocked=0
date: 2026-05-07
```

Remaining risks:

- After issue 5284 advances plain enum binding, this reference file is expected
  to expose computed enum member and TypeScript type-widening diagnostics.
