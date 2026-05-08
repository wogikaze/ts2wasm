---
id: 3354
title: "Implement Modulenodeimportrequireemit"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432, 5402]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage moduleNodeImportRequireEmit across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Closed as superseded by `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`.

Fresh triage on 2026-05-08 shows `moduleNodeImportRequireEmit.ts` does not
reach `import foo = require("foo")`. The current first blocker is the preceding
virtual `// @filename: package.json` section, which is parsed as TypeScript
source and fails at the JSON property colon.

Problem: moduleNodeImportRequireEmit is a generated bucket whose current first
blocker is already owned by issue 5402.

## Current failure

Fresh reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNodeImportRequireEmit.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNodeImportRequireEmit.ts --detail --no-dashboard-data
```

Observed coverage:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/moduleNodeImportRequireEmit.ts: UnsupportedSyntax: import-export
```

Fresh triage:

```text
UnsupportedSyntax: expected Semicolon, got Some(Colon) at 101..102
```

## Desired final state

This generated bucket is superseded by issue 5402. Do not implement directly
from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below.
- [x] Confirm whether existing open/done issues already cover this bucket.
- [x] Supersede this bucket with issue 5402, which already owns the current blocker.
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closure.

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing issue 5402 owns the current package.json virtual-section blocker.
- [x] Fresh triage evidence records the failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence.
- [x] No child issue is needed from 3354 because the current blocker is already implementation-ready in 5402.

## Validation

Required commands for this closure:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNodeImportRequireEmit.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNodeImportRequireEmit.ts
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

- [x] none; superseded by issue 5402

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleNodeImportRequireEmit.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated fresh on 2026-05-08.

Source context:

```text
1 | // @target: es5,es2016,es2020,esnext
2 | // @module: nodenext
3 | // @filename: package.json
4 | {
```

Compiler evidence:

```text
tokens: ok; JSON object tokens are present as LeftBrace, String("type"), Colon, String("module"), RightBrace.
visible symbols: []
ast/resolved: fail at JSON property colon with `expected Semicolon`.
```

TypeScript oracle:

```text
Top level includes Block for the package.json body, ModuleDeclaration for `declare module "foo";`, ImportEqualsDeclaration for `import foo = require("foo");`, and ExpressionStatement `foo;`.
Diagnostics include TS1005 for the package.json colon, TS2664 for module augmentation, and TS2307 for missing `foo`.
```

Superseding issue:

- `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNodeImportRequireEmit.ts --detail --no-dashboard-data
result: pass; reproduced UnsupportedSyntax/import-export with executed=1, unsupported=1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNodeImportRequireEmit.ts
result: pass; current first blocker is package.json property colon, superseded by issue 5402
date: 2026-05-08
```

Remaining risks:

- After issue 5402 skips package.json sections, this reference can expose later import-equals `require("foo")`, package resolution, or NodeNext emit blockers.
