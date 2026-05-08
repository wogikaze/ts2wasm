---
id: 3361
title: "Implement Moduleprologueamd"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432, 5232]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as superseded by
`issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`. Fresh triage for
`modulePrologueAMD.ts` reaches the existing entry-module `export class`
issue-5005 boundary after parsing the `"use strict"` prologue.

## Problem

Reference test results show 1 case failing in directory `modulePrologueAMD`
with diagnostics: import-export. Fresh triage shows tokens, AST, and resolved
dumps all succeed for:

```ts
"use strict";

export class Foo {}
```

Problem: this generated bucket duplicates the existing entry-module
`export class` implementation owner in issue 5232.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/modulePrologueAMD.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/modulePrologueAMD.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5232
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
- [x] Existing issue 5232 owns the current entry-module `export class` issue-5005 boundary
- [x] This closure includes failing path, diagnostic code, source context, visible symbols, parser/resolved evidence, and TypeScript AST evidence
- [x] No child issue is needed from 3361 because the current blocker is already implementation-ready in issue 5232

## Validation

Required commands for this closure:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePrologueAMD.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePrologueAMD.ts
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

- [x] none; superseded by issue 5232

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/modulePrologueAMD.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePrologueAMD.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePrologueAMD.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=module-system-amd:1
reference/typescript/tests/cases/compiler/modulePrologueAMD.ts: UnsupportedSyntax: module-system-amd
```

Current diagnostic:

```text
UnsupportedModule: issue-5005: entry module `export Foo` uses a declaration form outside the current static export slice; only export const and export default are supported at 61..73
```

Source context:

```text
// @module: amd
"use strict";

export class Foo {}
```

Compiler evidence:

```text
tokens: ok; String("use strict"), Semicolon, Export, Class, Ident("Foo"), braces
ast: ok; Expr(String "use strict") and ExportDecl(ClassDecl Foo)
resolved: ok; Expr(String "use strict") and ClassDecl Foo
visible symbols: []
module build: issue-5005 entry-module export class boundary
```

TypeScript oracle:

```text
ok=true
diagnostics=[]
AST topLevel includes ExpressionStatement `"use strict";` and ClassDeclaration `export class Foo {}`.
```

Superseding issue:

- `issues/open/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePrologueAMD.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax/module-system-amd
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePrologueAMD.ts
result: pass; current issue-5005 entry export-class blocker is superseded by issue 5232
date: 2026-05-08
```

Remaining risks:

- After issue 5232 lands, this reference may expose AMD module prologue emit
  parity, strict prologue preservation, or module-system-amd diagnostics.
