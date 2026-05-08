---
id: 3502
title: "Implement Newmissingidentifier"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: []
blocks: [5467]
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage newMissingIdentifier across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed after splitting the current malformed `new ()` parser diagnostic
blocker to `issues/open/5467-report-missing-new-expression-callee.md`.

## Problem

Reference test results show 1 cases fail in directory `newMissingIdentifier` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: newMissingIdentifier has 1 current reference failure. Fresh evidence
shows the blocker is a narrow parser/frontend diagnostic gap for malformed
`new ()` syntax, not a broad unknown-unsupported bucket.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newMissingIdentifier.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newMissingIdentifier.ts --detail
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5467-report-missing-new-expression-callee.md`.

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newMissingIdentifier.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newMissingIdentifier.ts
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

- [x] created: `issues/open/5467-report-missing-new-expression-callee.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/newMissingIdentifier.ts`

## Duplicate detection

- No exact existing owner was found for malformed `new ()`.
- `issues/open/5203-report-indexed-new-type-only-callee-diagnostics.md`
  covers indexed new callees such as `new any[1]`, not a missing callee.
- `issues/open/5466-report-malformed-new-angle-bracket-casts.md` covers
  malformed `new <any>...`, not empty parenthesized callees.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage unknown unsupported: newMissingIdentifier

- Issue class: triage-needed
- Feature label: unknown-unsupported
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/newMissingIdentifier.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: RightParen, ... }) at 34..35
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

reference/typescript/tests/cases/compiler/newMissingIdentifier.ts: UnsupportedSyntax: unknown-unsupported
```

Source context:

```ts
var x = new ();
```

Compiler evidence:

```text
tokens: ok; Var, Ident("x"), Equal, New, LeftParen, RightParen, Semicolon
ast: fails before AST construction
resolved: fails with the same UnsupportedSyntax
visible symbols before failure: binding x
```

TypeScript oracle evidence:

```text
TS1109: Expression expected.
AST path: VariableDeclaration -> NewExpression -> ParenthesizedExpression "()"
binding hint: x has type any
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newMissingIdentifier.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newMissingIdentifier.ts
result: pass; current blocker split to issue 5467
date: 2026-05-08
```

Remaining risks:

- Issue 5467 may expose a later malformed-new recovery detail after the generic
  unsupported-expression boundary is replaced.
