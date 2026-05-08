---
id: 1263
title: "Implement Coandcontravariantinferences Parser Syntax"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1263.

## Summary

Closed as superseded by
`issues/done/5226-w0-ast-node-span-requirement.md`. Fresh
triage shows this generated parser-syntax bucket is now blocked by duplicate
validation for ambient `declare function` overload signatures, not parser
syntax.

## Problem

Reference test results originally showed 1 case failing in directory
`coAndContraVariantInferences-parser-syntax` with diagnostics: parser-syntax.
Fresh coverage and triage now show tokens and AST succeed; `validate_ast`
stops on `DuplicateFunction` for the second ambient
`declare function assertNode(...)` overload declaration.

Problem: `coAndContraVariantInferences2.ts` fails with `DuplicateFunction` at
the second ambient `declare function assertNode(...)` declaration, which is
already tracked by issue 5226.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences2.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/coAndContraVariantInferences2.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is superseded by implementation-ready issue 5226, which
owns ambient `declare function` overload declarations. Do not implement
directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm issue 5226 already covers the current blocker
- [x] Close this stale generated bucket as superseded
- [x] Preserve exact reproduction commands and representative diagnostic evidence

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

- [x] Duplicate candidates below are confirmed and this issue is superseded by 5226
- [x] Issue 5226 contains exact reproduction commands for its representative cases
- [x] This issue records failing path, diagnostic code, source context, visible symbols, and parser/TypeScript oracle evidence
- [x] Issue 5226 acceptance covers the exact duplicate-function diagnostic family

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/coAndContraVariantInferences2.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences2.ts
```

Not run:

- `cargo fmt --all --check` (not run; issue metadata only)
- `cargo nextest run` (not run; issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by existing issue 5226

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/coAndContraVariantInferences2.ts`

Source context:

```ts
declare function assertNode<T extends Node, U extends T>(node: T | undefined, test: (node: T) => node is U): asserts node is U;
declare function assertNode(node: Node | undefined, test: ((node: Node) => boolean) | undefined): void;
```

## Duplicate detection

- `issues/done/5226-w0-ast-node-span-requirement.md` is the exact
  owner for multiple bodyless ambient `declare function` declarations with the
  same name.
- `issues/open/5200-validate-top-level-function-overload-implementations.md` is
  related but not exact: it handles non-ambient overload signatures with an
  implementation declaration.
- `issues/done/5199-report-function-overload-list-class-merge-diagnostics.md`
  is related but not exact: it handles overload lists followed by class
  declarations.
- Broad parser-syntax candidates are stale because fresh triage shows tokens
  and AST succeed.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/coAndContraVariantInferences2.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences2.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: DuplicateFunction:1
unsupported_features: duplicate-function:1

Diagnostic: DuplicateFunction
Message: duplicate function definition: `assertNode` at 1859..1869
Source: declare function assertNode(node: Node | undefined, test: ((node: Node) => boolean) | undefined): void;
tokens: ok
AST: ok; includes two ambient Function assertNode declarations
resolved: validate_ast fails on DuplicateFunction for the second ambient declaration
TypeScript oracle: ok, no diagnostics
Superseded by: 5226
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by `issues/done/5226-w0-ast-node-span-requirement.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences2.ts
result: pass; reproduced ambient declare-function overload DuplicateFunction and confirmed issue 5226 owns it
date: 2026-05-06
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5226
