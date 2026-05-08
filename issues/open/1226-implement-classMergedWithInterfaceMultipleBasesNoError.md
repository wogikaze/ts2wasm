---
id: 1226
title: "Implement Classmergedwithinterfacemultiplebasesnoerror"
type: spike
area: frontend/syntax
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
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1226.

## Summary

Triage classMergedWithInterfaceMultipleBasesNoError across 1 failing reference
test case and split the current default class export boundary into an
implementation-ready child issue.

## Problem

Reference test results previously showed 1 case failing in directory
`classMergedWithInterfaceMultipleBasesNoError` with diagnostics: import-export.
Fresh triage shows the current first blocker is `export default class`, not the
interface/class merge semantics.

Problem: `export default class extends Foo { ... }` stops at the generic
issue-055 default class export boundary before the class body or later TS2729
semantic diagnostic can be reached.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing default export work does not cover default class declarations
- [x] Split the default class export boundary into child issue 5326
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is split
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5326-support-default-class-export-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts`

## Duplicate detection

- `issues/open/231-parse-static-es-module-declarations.md` - related parser work but explicitly kept default class exports out of scope
- `issues/open/5008-static-es-module-export-default-namespace-reexport.md` - related static default export infrastructure, but current triage still reports issue-055 default class export
- `issues/open/2282-implement-exportDefaultClassAndValue.md` and sibling generated default-class buckets - related generated buckets that still need smart triage before superseding

## Smart triage

Fresh triage shows this generated import/export bucket is currently blocked by
the default class export parser/module boundary.

### Smart triage: classMergedWithInterfaceMultipleBasesNoError

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Current compiler message: `issue-055: unsupported default class export; module resolution and loading are not implemented`
- Path: `reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
interface Bar { }
interface Baz { }
interface Q { }
interface Foo extends Bar, Baz { }
class Foo { }

export default class extends Foo {
    readonly observer = this.handleIntersection;
    readonly handleIntersection = () => { }
}
```

Compiler evidence:

```text
tokens: ok; Export, Default, Class, Extends, Ident("Foo")
ast/resolved: fail at issue-055 unsupported default class export
```

TypeScript oracle evidence:

```text
typescript ok: false
diagnostic TS2729: Property 'handleIntersection' is used before its initialization.
AST topLevel includes a default-exported anonymous ClassDeclaration extending Foo.
```

Split result:

- `issues/open/5326-support-default-class-export-declarations.md`

## Completion evidence

Commits:

- Split to `issues/open/5326-support-default-class-export-declarations.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; issue-055 default class export split to issue 5326
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; unsupported=1, UnsupportedSyntax unknown-unsupported
date: 2026-05-07
```

Remaining risks:

- After issue 5326 lands, this reference may expose the later TS2729 property
  initialization-order diagnostic.
