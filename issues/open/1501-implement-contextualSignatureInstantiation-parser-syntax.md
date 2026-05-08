---
id: 1501
title: "Implement Contextualsignatureinstantiation Parser Syntax"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1501.

## Summary

Triage contextualSignatureInstantiation-parser-syntax across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `contextualSignatureInstantiation-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualSignatureInstantiation-parser-syntax has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualSignatureInstantiation2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstantiation2.ts --detail
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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstantiation2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualSignatureInstantiation2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5371-parse-generic-function-type-annotations.md`
- [x] created: `issues/done/5372-parse-ambient-function-asi-with-constructor-types.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualSignatureInstantiation2.ts`
- `reference/typescript/tests/cases/compiler/contextualSignatureInstantiation4.ts`

## Duplicate detection

- `issues/done/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/done/550-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/done/663-implement-arrayAssignmentTest-parser-syntax.md` - Implement Arrayassignmenttest Parser Syntax (same feature label, title overlap)
- `issues/done/734-implement-assignmentCompatability-parser-syntax.md` - Implement Assignmentcompatability Parser Syntax (same feature label, title overlap)
- `issues/done/753-implement-asyncFunctionReturnType-parser-syntax.md` - Implement Asyncfunctionreturntype Parser Syntax (same feature label, title overlap)
- `issues/done/767-implement-augmentedTypesEnum-parser-syntax.md` - Implement Augmentedtypesenum Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)

## Smart triage

Fresh triage on 2026-05-07 shows this generated parser-syntax bucket contains
two independent parser blockers, so it was split instead of implemented
directly.

`contextualSignatureInstantiation2.ts` fails while parsing a variable type
annotation that starts with a generic function type:

```text
UnsupportedSyntax: expected Semicolon, got Some(Greater) at 58..59
```

Source context:

```ts
var dot: <T, S>(f: (_: T) => S) => <U>(g: (_: U) => T) => (_: U) => S;
dot = <T, S>(f: (_: T) => S) => <U>(g: (_: U) => T): (r:U) => S => (x) => f(g(x));
```

Compiler evidence:

```text
tokens: ok through `<T, S>` and nested arrow/function type tokens
ast/resolved: fail at the closing `>` in the generic function type annotation
TypeScript oracle: parses FunctionType nodes and only reports later TS2454 for `id`
```

This parser slice was split to
`issues/done/5371-parse-generic-function-type-annotations.md`.

`contextualSignatureInstantiation4.ts` fails on ASI after an ambient function
declaration whose parameter type is a constructor signature:

```text
UnsupportedTypeScriptSyntax: issue-400: unterminated ambient function declaration at 147..154
```

Source context:

```ts
declare function fruitFactory1<TFruit>(Fruit: new (...args: any[]) => TFruit): TFruit
const banana1 = fruitFactory1(Banana) // Banana<any>
```

Compiler evidence:

```text
tokens: ok through declare class Banana and the ambient function signature
ast/resolved: fail at `declare` with the issue-400 unterminated ambient function boundary
TypeScript oracle: diagnostics=[]; accepts the declaration and infers Banana<any>
```

This is adjacent to `issues/done/705-implement-asiAmbientFunctionDeclaration.md`,
but 705 is still a generated triage-needed bucket. The implementation-ready
child for this concrete shape was split to
`issues/done/5372-parse-ambient-function-asi-with-constructor-types.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- Contextual signature instantiation semantics remain hidden until issues 5371
  and 5372 advance these files past the current parser boundaries.
