---
id: 1264
title: "Implement Coandcontravariantinferences Type System"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1264.

## Summary

Triage coAndContraVariantInferences-type-system across 6 reference cases and
close it after mapping current blockers to existing implementation-ready issues
plus one new parser child.

## Problem

Reference test results previously showed 6 cases failing in directory
`coAndContraVariantInferences-type-system` with diagnostics: type-system.
Fresh triage shows this generated bucket is mixed: one listed case now
build-passes, four cases are superseded by existing parser issues, and one
case needed a new focused interface call-signature parser issue.

Problem: the generated bucket is not one executable type-system slice. Its
current blockers are parser-frontier issues that must be tracked separately.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/coAndContraVariantInferences --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Current blockers are owned by focused
implementation-ready issues.

## Scope

In scope:

- [x] Inspect fresh smart triage for the listed cases
- [x] Confirm existing open issues cover most current blockers
- [x] Split the uncovered interface call-signature parser blocker
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing owner issues cover generic type-argument skipping, const enum parsing, and typed function trailing commas
- [x] Child issue 5332 contains exact reference-triage and coverage commands
- [x] Child issue 5332 includes failing path, diagnostic, source context, compiler evidence, and TypeScript AST evidence

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/coAndContraVariantInferences --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences3.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences4.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences5.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences6.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences7.ts
```

Not run:

- `cargo fmt --all --check`; issue split/close only, no Rust code changed
- `cargo nextest run`; issue split/close only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5332-parse-interface-call-signatures.md`
- [x] superseded by `issues/done/5184-parse-const-enum-declarations.md`
- [x] superseded by `issues/done/5303-parse-trailing-comma-in-typed-function-parameters.md`
- [x] superseded by `issues/open/5309-skip-generic-type-arguments-in-type-annotations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/coAndContraVariantInferences.ts`
- `reference/typescript/tests/cases/compiler/coAndContraVariantInferences3.ts`
- `reference/typescript/tests/cases/compiler/coAndContraVariantInferences5.ts`
- `reference/typescript/tests/cases/compiler/coAndContraVariantInferences6.ts`
- `reference/typescript/tests/cases/compiler/coAndContraVariantInferences4.ts`
- `reference/typescript/tests/cases/compiler/coAndContraVariantInferences7.ts`

## Duplicate detection

- `issues/done/5184-parse-const-enum-declarations.md` - exact owner for `coAndContraVariantInferences3.ts` and `coAndContraVariantInferences4.ts` current `const enum` parser blocker
- `issues/done/5303-parse-trailing-comma-in-typed-function-parameters.md` - exact owner for `coAndContraVariantInferences5.ts` trailing comma in typed function parameters
- `issues/open/5309-skip-generic-type-arguments-in-type-annotations.md` - exact owner for `coAndContraVariantInferences.ts` generic type arguments inside function parameter annotations
- `issues/open/5332-parse-interface-call-signatures.md` - new child for `coAndContraVariantInferences6.ts` interface call-signature parsing
- `issues/open/345-implement-tsc-type-alias-coverage.md` - Implement TypeScript type alias coverage for tsc suite (23 cases) (same feature label, title overlap)

## Smart triage

Fresh triage shows this generated type-system bucket is a set of parser-frontier
blockers rather than a single co/contravariant inference implementation.

Coverage over the family:

```text
executed=8
build_pass=1
unsupported=7
unsupported_diagcodes=UnsupportedSyntax:6,UnresolvedName:1
unsupported_features=type-system:6,name-resolution:1
```

The path filter also matched sibling files outside this bucket:
`coAndContraVariantInferences2.ts` and `coAndContraVariantInferences8.ts`.
Issue 1262 owns the `8.ts` name-resolution cleanup; issue 1263 already tracks
the `2.ts` generated bucket history.

### coAndContraVariantInferences.ts

- Diagnostic: `UnsupportedSyntax`
- Message: `expected Comma, got Some(Greater) at 655..656`
- Source: `fn: (action: Action<TName,TPayload>)=> any`
- Owner: `issues/open/5309-skip-generic-type-arguments-in-type-annotations.md`

### coAndContraVariantInferences3.ts

- Diagnostic: `UnsupportedSyntax`
- Message: `const declarations require an initializer at 1965..1969`
- Source: `const enum SyntaxKind { ... }`
- Owner: `issues/done/5184-parse-const-enum-declarations.md`

### coAndContraVariantInferences4.ts

- Diagnostic: `UnsupportedSyntax`
- Message: `const declarations require an initializer at 46..50`
- Source: `const enum SyntaxKind { ... }`
- Owner: `issues/done/5184-parse-const-enum-declarations.md`

### coAndContraVariantInferences5.ts

- Diagnostic: `UnsupportedSyntax`
- Message: `issue-247: expected binding identifier or pattern, got Some(RightParen) at 181..182`
- Source: trailing comma after `onChange: (status: Thing | null) => void,`
- Owner: `issues/done/5303-parse-trailing-comma-in-typed-function-parameters.md`

### coAndContraVariantInferences6.ts

- Diagnostic: `UnsupportedSyntax`
- Message: `unsupported expression: Some(SpannedToken { kind: Greater, span: Span { start: 190, end: 191 } }) at 192..193`
- Source: `interface FunctionComponent<P = {}> { (props: P): ReactElement<any> | null; }`
- Split result: `issues/open/5332-parse-interface-call-signatures.md`

### coAndContraVariantInferences7.ts

- Diagnostic: `BuildPass`
- Message: `ts2wasm build succeeded`
- Result: no current compiler blocker in this metadata bucket

## Completion evidence

Commits:

- Split to `issues/open/5332-parse-interface-call-signatures.md`; other current blockers superseded by existing implementation-ready issues.

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/coAndContraVariantInferences --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; listed current family blockers
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences.ts
result: pass; current blocker superseded by issue 5309
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences3.ts
result: pass; current blocker superseded by issue 5184
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences4.ts
result: pass; current blocker superseded by issue 5184
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences5.ts
result: pass; current blocker superseded by issue 5303
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences6.ts
result: pass; current blocker split to issue 5332
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences7.ts
result: pass; BuildPass
date: 2026-05-07
```

Remaining risks:

- After these parser-frontier issues land, the family may expose actual
  co/contravariant inference or call compatibility diagnostics.
