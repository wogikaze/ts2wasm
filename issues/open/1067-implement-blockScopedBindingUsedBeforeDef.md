---
id: 1067
title: "Implement Blockscopedbindingusedbeforedef"
type: spike
area: frontend/resolver
class: superseded
priority: P2
depends_on: [5180]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage blockScopedBindingUsedBeforeDef across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `blockScopedBindingUsedBeforeDef` with diagnostics: scope-analysis. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: blockScopedBindingUsedBeforeDef has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedBindingUsedBeforeDef.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedBindingUsedBeforeDef.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedBindingUsedBeforeDef.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedBindingUsedBeforeDef.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/done/5180-parse-computed-property-object-binding-patterns.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/blockScopedBindingUsedBeforeDef.ts`

## Duplicate detection

- `issues/done/5006-meta-tsc-scope-analysis.md` is not an implementation-ready child for this current parser blocker.
- Other `scope-analysis` generated buckets share only the broad feature label.
- No open issue was found for computed property names in object binding patterns.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/blockScopedBindingUsedBeforeDef.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `issue-247: expected object binding property key, got Some(SpannedToken { kind: LeftBracket, span: Span { start: 56, end: 57 } }) at 57..58`
- First failing source line: `for (let {[a]: a} of [{ }]) continue;`
- Visible symbols before failure: none
- Compiler evidence: token dump includes `LeftBrace`, `LeftBracket`, `Ident("a")`, `RightBracket`, `Colon`, and target `Ident("a")`; AST/resolved construction fails before representing the binding element.
- TypeScript AST path: `ForOfStatement -> VariableDeclarationList -> VariableDeclaration -> ObjectBindingPattern -> BindingElement -> ComputedPropertyName -> Identifier`
- TypeScript oracle: reports `TS2448` used-before-declaration and `TS2538` invalid index type diagnostics for the computed key identifiers.
- Superseding child: `issues/done/5180-parse-computed-property-object-binding-patterns.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingUsedBeforeDef.ts
result: pass; current blocker identified as computed object binding property parser support, split to issue 5180
date: 2026-05-06
```

Remaining risks:

- Later triage may expose the intended `TS2448` and `TS2538` diagnostics after issue 5180 advances past parsing.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

