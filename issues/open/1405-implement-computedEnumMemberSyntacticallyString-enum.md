---
id: 1405
title: "Implement Computedenummembersyntacticallystring Enum"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5001]
blocks: [5296]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1405.

## Summary

Triage computedEnumMemberSyntacticallyString-enum across 1 failing reference
test case and split this generated bucket into implementation-ready child issue
5296.

## Problem

Reference test results originally showed 1 case failing in directory
`computedEnumMemberSyntacticallyString-enum` with diagnostics: enum. Fresh
smart triage shows the current first concrete blocker is the parser handling of
`2..toFixed(0)` before the enum declaration.

Problem: `computedEnumMemberSyntacticallyString.ts` reports
`UnsupportedSyntax` for the second dot in `2..toFixed(0)`; issue 5296 now owns
the actionable parser fix.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed because the actionable work has been split into
issue 5296. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

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
- [x] At least one child issue contains an exact `python scripts/manager.py reference-triage ...` command
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString.ts
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

- [x] created: `issues/open/5296-parse-double-dot-numeric-literal-property-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString.ts`

## Duplicate detection

Fresh smart triage listed `issues/open/1406-implement-computedEnumMemberSyntacticallyString-parser-syntax.md`,
`issues/open/428-implement-enum.md`, and enum-related done issues. Issue 1406
tracks the sibling `computedEnumMemberSyntacticallyString2.ts` reference path,
which does not start with `2..toFixed(0)`. Issue 428 is a broad generated enum
bucket whose body currently points at unrelated test262 non-enumerable-global
evidence, so it is not an implementation-ready owner for this parser blocker.
Existing enum issues 5284 and 5277 cover plain/export enum boundaries after the
parser can reach the enum declaration.

Resolution:

```text
Split to issue 5296: parse double-dot numeric literal property access.
```

## Smart triage

### Smart triage: Triage enum: computedEnumMemberSyntacticallyString

- Issue class: `triage-needed`
- Feature label: `enum`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString.ts
```

Failure location:

```text
message: expected member property name, got Dot at 95..102
line: 5, column: 16
```

Source context:

```ts
const BAR = 2..toFixed(0);

enum Foo {
    A = `${BAR}`,
    B = "2" + BAR,
    C = (`${BAR}`),
```

Visible symbols before failure:

```text
binding BAR initializer: 2..
```

Compiler evidence:

```text
tokens: ok; includes Number(2), Dot, Dot, Ident("toFixed")
ast: ok=False; UnsupportedSyntax expected member property name, got Dot
resolved: ok=False; same parser diagnostic
```

TypeScript oracle evidence:

```text
AST path at failure position:
SourceFile -> FirstStatement -> VariableDeclarationList -> VariableDeclaration
-> CallExpression -> PropertyAccessExpression -> Identifier
text: 2..toFixed(0)

TypeScript then reports TS18033 diagnostics for string-valued computed enum
member values after parsing the file.
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=enum:1
semantic_enabled=0
```

## Completion evidence

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString.ts
result: pass; reproduces UnsupportedSyntax for double-dot numeric literal property access
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, blocked=0
date: 2026-05-07
```

Remaining risks:

- After issue 5296 advances the parser, this reference file is expected to
  expose the enum-specific computed member diagnostics shown by the TypeScript
  oracle.
