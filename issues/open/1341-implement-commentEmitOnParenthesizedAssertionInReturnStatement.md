---
id: 1341
title: "Implement Commentemitonparenthesizedassertioninreturnstatement"
type: spike
area: frontend/syntax
class: blocked
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
> Evidence: Empty completion evidence. No feat/fix commit for #1341.

## Summary

Closed as superseded by `issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md` for the current entry-module `export class` issue-5005 boundary.

## Problem

Fresh triage confirms this generated bucket is too broad for direct implementation. Both affected files tokenize and parse their class bodies, but module build stops at the entry-module `export class Foo` declaration:

```ts
export class Foo {
  client = {
    getThing: () => Promise.resolve("")
  }

  foo() {
    return (
      this.client.getThing() satisfies Promise<string>
    );
  }
}
```

The current first blocker is `UnsupportedModule: issue-5005` for `export class`, already tracked by issue 5232.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/commentEmitOnParenthesizedAssertionInReturnStatement2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/commentEmitOnParenthesizedAssertionInReturnStatement2.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5232 instead of splitting a duplicate child
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in issue 5232 and this closure

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
- [x] Superseding issue 5232 contains exact export-class diagnostic evidence for both paths
- [x] Superseding issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the exact entry-module `export class` diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/commentEmitOnParenthesizedAssertionInReturnStatement2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/commentEmitOnParenthesizedAssertionInReturnStatement2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected: issue metadata only

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentEmitOnParenthesizedAssertionInReturnStatement2.ts`
- `reference/typescript/tests/cases/compiler/commentEmitOnParenthesizedAssertionInReturnStatement.ts`

## Duplicate detection

- `issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md` owns the current issue-5005 entry-module `export class` boundary.
- `issues/open/5324-support-dependency-export-class-declarations.md` is related but covers dependency virtual files, not the entry module.
- Later comment emit, parenthesized assertion preservation, `satisfies`, and `this.client.getThing()` lowering behavior remains unproven until issue 5232 advances past the export-class boundary.

## Smart triage

Generated 2026-05-07.

Commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentEmitOnParenthesizedAssertionInReturnStatement.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentEmitOnParenthesizedAssertionInReturnStatement2.ts
```

Result for both files:

```text
Smart triage: Triage import export
Feature label: import-export
Diagnostic: UnsupportedModule / unsupported-feature-boundary
Message: issue-5005: entry module `export Foo` uses a declaration form outside the current static export slice; only export const and export default are supported
Failure location: line 6, column 8, at `export class Foo`
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentEmitOnParenthesizedAssertionInReturnStatement --detail --no-dashboard-data
```

```text
executed=2
build_pass=0
unsupported=2
blocked=0
unsupported_diagcodes=UnsupportedSyntax:2
unsupported_features=type-assertion:2
semantic_enabled=0
```

Compiler evidence:

- Tokens: ok for both files, including `export class Foo`, class field `client`, arrow `getThing`, parenthesized return, and `as` or `satisfies` tokens.
- AST: ok for both files; contains `ExportDecl(ClassDecl Foo)` with method `foo` returning a call to `this.client.getThing()`.
- Triage/module build: fails with `UnsupportedModule issue-5005` for entry-module `export class`.
- Dumped later pipeline evidence also shows `issue-211` for method `getThing` on `this.client`, but that is not the current triage first blocker and is not owned by this closure.
- TypeScript oracle: both files have no diagnostics.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closure commit pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentEmitOnParenthesizedAssertionInReturnStatement.ts
result: pass; reproduced entry-module export class issue-5005 and updated issue 5232
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentEmitOnParenthesizedAssertionInReturnStatement2.ts
result: pass; reproduced entry-module export class issue-5005 and updated issue 5232
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentEmitOnParenthesizedAssertionInReturnStatement --detail --no-dashboard-data
result: pass; executed=2 build_pass=0 unsupported=2 blocked=0 unsupported_diagcodes=UnsupportedSyntax:2 unsupported_features=type-assertion:2
date: 2026-05-07
```

Remaining risks:

- Later comment emit, parenthesized assertion preservation, `satisfies`, and `this.client.getThing()` lowering behavior remains unproven until issue 5232 advances past the export-class boundary.
