---
id: 3495
title: "Implement Newabstractinstance Parser Syntax"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: [5465]
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage newAbstractInstance-parser-syntax across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed after splitting the current `export default abstract class {}` parser
blocker to
`issues/open/5465-parse-abstract-anonymous-default-class-exports.md`.

## Problem

Reference test results show 1 cases fail in directory `newAbstractInstance-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: newAbstractInstance-parser-syntax has 1 current reference failure.
Fresh evidence shows the blocker is the abstract anonymous default class export
form, not a broad parser-syntax bucket.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newAbstractInstance2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newAbstractInstance2.ts --detail
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5465-parse-abstract-anonymous-default-class-exports.md`.

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
- [x] Child issue contains an exact reference-triage command
- [x] Child issue includes failing path, diagnostic code, source context,
  visible symbols, parser evidence, and TypeScript oracle evidence
- [x] Child issue acceptance names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newAbstractInstance2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newAbstractInstance2.ts
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

- [x] created: `issues/open/5465-parse-abstract-anonymous-default-class-exports.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/newAbstractInstance2.ts`

## Duplicate detection

- `issues/open/5326-support-default-class-export-declarations.md` owns the
  anonymous default class export form without `abstract`.
- `issues/open/5367-support-named-default-class-export-declarations.md` owns the
  named default class export form.
- No exact implementation-ready owner was found for the `abstract` modifier
  variant, so this bucket was split to issue 5465.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage unknown unsupported: newAbstractInstance2

- Issue class: triage-needed
- Feature label: unknown-unsupported
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/newAbstractInstance2.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Abstract, ... }) at 86..91
```

Focused coverage:

```text
suite=tsc
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0

reference/typescript/tests/cases/compiler/newAbstractInstance2.ts: UnsupportedSyntax: unknown-unsupported
```

Source context:

```ts
// @Filename: /a.ts
export default abstract class {}

// @Filename: /b.ts
import A from "./a";
new A();
```

Compiler evidence:

```text
tokens: ok; Export, Default, Abstract, Class, LeftBrace, RightBrace, Import, New A()
ast: fails before AST construction
resolved: fails with the same UnsupportedSyntax
visible symbols before failure: []
```

TypeScript oracle evidence:

```text
AST topLevel includes:
- ClassDeclaration "export default abstract class {}"
- ImportDeclaration "import A from \"./a\";"
- ExpressionStatement "new A();"
diagnostics: TS2307 for later import resolution of "./a"
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newAbstractInstance2.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newAbstractInstance2.ts
result: pass; current blocker split to issue 5465
date: 2026-05-08
```

Remaining risks:

- Issue 5465 may expose a later import-resolution or abstract-class
  constructability diagnostic after the default abstract class export parses.
