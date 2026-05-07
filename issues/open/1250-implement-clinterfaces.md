---
id: 1250
title: "Implement Clinterfaces"
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
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1250.

## Summary

Triage `clinterfaces` across 1 failing reference test case and close it as
superseded by the existing export-assignment parser issue.

## Problem

Reference test results show 1 case failing in directory `clinterfaces` with
diagnostics: import-export. Fresh triage confirms the current first blocker is
`issue-055` at `export = Foo;`.

Problem: `clinterfaces.ts` reports `issue-055: unsupported static export` at
`export = Foo;`. Existing issue 5186 already owns parsing `export = expr` as an
AST statement with expression-span evidence before later module/semantic
diagnostics.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/clinterfaces.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/clinterfaces.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5186 covers the current `export = Foo` parser/export-assignment blocker
- [x] Confirm no narrower clinterfaces-specific child is needed before export assignment support
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
- [x] Superseding issue 5186 contains the implementation-ready export-assignment parser scope
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/clinterfaces.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/clinterfaces.ts
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

- [x] superseded by `issues/open/5186-parse-export-assignment-for-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/clinterfaces.ts`

## Duplicate detection

- `issues/open/5186-parse-export-assignment-for-diagnostics.md` - exact current owner for representing `export = expr;` instead of stopping at generic `issue-055`
- `issues/open/5306-report-export-assignment-with-other-exports.md` is related but covers the additional rule for `export =` mixed with other exported declarations, which is not the current `clinterfaces.ts` shape.
- Generated export-assignment buckets such as `2259` and `2263` are no-match parent buckets because 5186 is the narrower implementation-ready owner.

## Smart triage

### Smart triage: clinterfaces

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Current compiler message: `issue-055: unsupported static export; module resolution and loading are not implemented`
- Path: `reference/typescript/tests/cases/compiler/clinterfaces.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/clinterfaces.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/clinterfaces.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
namespace M {
    class C { }
    interface C { }
    interface D { }
    class D { }
}

interface Foo<T> {
    a: string;
}

class Foo<T>{
    b: number;
}

export = Foo;
```

Compiler evidence:

```text
tokens: ok; namespace, class/interface merge declarations, and `Export`, `Equal`, `Ident("Foo")` are tokenized
ast/resolved: UnsupportedModule issue-055 at `export = Foo`
visible symbols: classes C, D, Foo, and Bar are visible before the export assignment
```

TypeScript oracle evidence:

```text
AST topLevel includes ExportAssignment `export = Foo;`
diagnostics: TS2564 for uninitialized class properties `b`; no export-assignment syntax error
```

Superseding owner:

- `issues/open/5186-parse-export-assignment-for-diagnostics.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by existing issue 5186; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/clinterfaces.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; current issue-055 export-assignment boundary superseded by 5186
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/clinterfaces.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; executed=1, unsupported=1
date: 2026-05-07
```

Remaining risks:

- After issue 5186 lands, this path may expose CommonJS export assignment
  lowering, interface/class merge diagnostics, or TS2564 strict property
  initialization parity.
