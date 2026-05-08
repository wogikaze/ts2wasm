---
id: 3356
title: "Implement Modulenoneerrors"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432, 5324]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as superseded by
`issues/open/5324-support-dependency-export-class-declarations.md`.
Fresh triage for `moduleNoneErrors.ts` reaches the existing issue-5005
dependency-module `export class` boundary.

## Problem

Reference test results show 1 case failing in directory `moduleNoneErrors` with
diagnostics: import-export. Fresh triage shows tokens, AST, and resolved dumps
all succeed for:

```ts
export class Foo {
    foo: string;
}
```

Problem: this generated bucket duplicates the existing dependency-module
`export class` implementation owner in issue 5324.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleNoneErrors.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoneErrors.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5324
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closure

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
- [x] Existing issue 5324 owns the current dependency-module `export class` issue-5005 boundary
- [x] This closure includes failing path, diagnostic code, source context, visible symbols, parser/resolved evidence, and TypeScript AST evidence
- [x] No child issue is needed from 3356 because the current blocker is already implementation-ready in issue 5324

## Validation

Required commands for this closure:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoneErrors.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNoneErrors.ts
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Not run:

- Cargo gates; no Rust source changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by issue 5324

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleNoneErrors.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNoneErrors.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoneErrors.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/moduleNoneErrors.ts: UnsupportedSyntax: import-export
```

Current diagnostic:

```text
UnsupportedModule: issue-5005: dependency module declaration export uses a form outside the current static export slice at 7..37
```

Source context:

```text
// @target: es5, es2015
// @module: none
// @Filename: a.ts
export class Foo {
    foo: string;
}
```

Compiler evidence:

```text
tokens: ok; Export, Class, Ident("Foo"), class field tokens, and braces are present
ast: ok; ExportDecl(ClassDecl Foo) is represented
resolved: ok; ClassDecl Foo is represented
visible symbols: []
module build: issue-5005 dependency-module export class boundary
```

TypeScript oracle:

```text
AST topLevel includes ClassDeclaration `export class Foo { foo: string; }`.
Diagnostic TS2564 reports the uninitialized `foo` property, proving the class
export is parsed before TypeScript's semantic diagnostic.
```

Superseding issue:

- `issues/open/5324-support-dependency-export-class-declarations.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoneErrors.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNoneErrors.ts
result: pass; current issue-5005 dependency export-class blocker is superseded by issue 5324
date: 2026-05-08
```

Remaining risks:

- After issue 5324 lands, this reference should advance to the TypeScript
  semantic diagnostic for the uninitialized class property, or to a narrower
  class-field/module-none diagnostic.
