---
id: 409
title: "Implement tsgo declaration emit: package-json exports and subpath reexport cases"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: [399]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Handle two remaining tsgo `declaration-emit` cases related to package-json exports and re-export declaration emit:

- `declarationEmitResolvePackageJsonExportsFalse.ts`
- `declarationEmitSubpathImportsReexport.ts`

## Problem

Both fixtures are currently blocked by `UnsupportedSyntax: declaration-emit` and are independently implementable from parser and emit-erasure perspective.

## Current failure

```sh
mise run reference-coverage -- tsgo --limit 166 --detail --no-web-ui | rg 'declarationEmitResolvePackageJsonExportsFalse|declarationEmitSubpathImportsReexport'
```

## Desired final state

- Neither fixture is blocked by `declaration-emit` after implementation.
- The relevant declaration/emit path supports package-json export and subpath-import declaration emission rules for these forms.

## Scope

In scope:

- [ ] Parse and accept the package-json-export/re-export declaration forms in the two targeted files.
- [ ] Ensure emit/erasure behavior is deterministic and aligned with parser/IR boundary decisions from issue 399.
- [ ] Add focused frontend/fixture coverage for both cases.

Out of scope:

- Generic module-resolution changes across unrelated `declaration-emit` cases.
- New runtime builtin support for non-declaration emit features.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/ir/src/lowered.rs`

Do not touch:

- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitResolvePackageJsonExportsFalse.ts` is no longer `UnsupportedSyntax: declaration-emit`.
- [ ] `reference/typescript-go/testdata/tests/cases/compiler/declarationEmitSubpathImportsReexport.ts` is no longer `UnsupportedSyntax: declaration-emit`.
- [ ] Focused verification command confirms the two cases exit pass.

## Validation

Required commands:

```sh
mise run reference-coverage -- tsgo --path-filter declarationEmitResolvePackageJsonExportsFalse.ts,declarationEmitSubpathImportsReexport.ts --limit 166 --no-web-ui
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none
